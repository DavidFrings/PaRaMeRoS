//! Redis rate limiting and response cache. Every call fails open: on error or
//! timeout it logs a warning and acts as "no limit" / "cache miss".
use crate::Env;
use actix_web::rt::time::timeout;
use log::warn;
use redis::{AsyncCommands, aio::ConnectionManager};
use std::{future::Future, time::Duration};

const OP_TIMEOUT: Duration = Duration::from_millis(500);

/// Run a Redis operation with a timeout; `None` when Redis is absent, failing or slow.
async fn run<T, F, Fut>(env: &Env, f: F) -> Option<T>
where
    F: FnOnce(ConnectionManager) -> Fut,
    Fut: Future<Output = redis::RedisResult<T>>,
{
    let conn = env.redis.clone()?;
    match timeout(OP_TIMEOUT, f(conn)).await {
        Ok(Ok(v)) => Some(v),
        Ok(Err(err)) => {
            warn!("Redis error: {err}");
            None
        }
        Err(_) => {
            warn!("Redis operation timed out");
            None
        }
    }
}

/// Seconds until `key` expires when its counter is already at or over `max`.
pub async fn check_limit(env: &Env, key: &str, max: u32) -> Option<u64> {
    run(env, |mut c| async move {
        let count: Option<u32> = c.get(key).await?;
        if count.unwrap_or(0) < max {
            return Ok(None);
        }
        let ttl: i64 = c.ttl(key).await?;
        Ok(Some(ttl.max(1) as u64))
    })
    .await
    .flatten()
}

/// Count one hit; `Some(retry_after_secs)` once the count exceeds `max`.
pub async fn hit_limit(env: &Env, key: &str, max: u32, window_secs: u64) -> Option<u64> {
    run(env, |mut c| async move {
        let (count, ttl): (u32, i64) = redis::pipe()
            .atomic()
            .incr(key, 1)
            .cmd("EXPIRE")
            .arg(key)
            .arg(window_secs)
            .arg("NX")
            .ignore()
            .ttl(key)
            .query_async(&mut c)
            .await?;
        Ok((count > max).then(|| ttl.max(1) as u64))
    })
    .await
    .flatten()
}

pub async fn reset_limit(env: &Env, key: &str) {
    run(env, |mut c| async move { c.del::<_, ()>(key).await }).await;
}

pub async fn cache_get(env: &Env, key: &str) -> Option<String> {
    run(
        env,
        |mut c| async move { c.get::<_, Option<String>>(key).await },
    )
    .await
    .flatten()
}

pub async fn cache_set(env: &Env, key: &str, json: &str, ttl_secs: u64) {
    run(env, |mut c| async move {
        c.set_ex::<_, _, ()>(key, json, ttl_secs).await
    })
    .await;
}

/// Drop all cached post reads (and the sitemap). Uses SCAN, never KEYS.
pub async fn cache_clear_posts(env: &Env) {
    run(env, |mut c| async move {
        let mut cursor = 0u64;
        loop {
            let (next, keys): (u64, Vec<String>) = redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg("cache:*")
                .arg("COUNT")
                .arg(100)
                .query_async(&mut c)
                .await?;
            if !keys.is_empty() {
                c.del::<_, ()>(keys).await?;
            }
            if next == 0 {
                return Ok(());
            }
            cursor = next;
        }
    })
    .await;
}
