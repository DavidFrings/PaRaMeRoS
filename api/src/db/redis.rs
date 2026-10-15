use ::redis::{AsyncCommands, Client, aio::ConnectionManager};
use log::warn;
use serde::{Serialize, de::DeserializeOwned};
#[derive(Clone)]
pub struct Cache {
    conn: ConnectionManager,
}

impl Cache {
    pub(crate) async fn establish(url: String) -> Self {
        let client = Client::open(url.clone()).unwrap_or_else(|_| panic!("Invalid Redis URL: {}", url));
        let conn = ConnectionManager::new(client).await
            .unwrap_or_else(|_| panic!("Could not connect ro Redis"));
        Self { conn }
    }

    pub(crate) async fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        let mut conn = self.conn.clone();
        let raw: Option<String> = conn.get(key).await
            .unwrap_or_else(|err| {
                warn!("Redis GET {} failed: {}", key, err);
                None
            });
        serde_json::from_str(&raw?).ok()
    }

    pub(crate) async fn set<T: Serialize>(&self, key: &str, value: &T, ttl_secs: u64) {
        let Ok(json) = serde_json::to_string(value) else {
            return;
        };
        let mut conn = self.conn.clone();
        if let Err(err) = conn.set_ex::<_, _, ()>(key, json, ttl_secs).await {
            warn!("Redis SET {} failed: {}", key, err);
        }
    }

    pub(crate) async fn del(&self, key: &str) {
        let mut conn = self.conn.clone();
        if let Err(err) = conn.del::<_, ()>(key).await {
            warn!("Redis DEL {} failed: {}", key, err);
        }
    }
}