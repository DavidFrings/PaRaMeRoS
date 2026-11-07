mod common;

use actix_web::{
    App,
    dev::{Service, ServiceResponse},
    http::header::AUTHORIZATION,
    test::{TestRequest, call_service, init_service, read_body_json},
    web::Data,
};
use diesel_async::{AsyncPgConnection, SimpleAsyncConnection};
use parameros_api::app;
use serde_json::{Value, json};

fn has_redis() -> bool {
    let ok = std::env::var("TEST_REDIS_URL").is_ok();
    if !ok {
        eprintln!("TEST_REDIS_URL not set, skipping");
    }
    ok
}

async fn login_as<S, B>(app: &S, ip: &str, user: &str, pass: &str) -> ServiceResponse<B>
where
    S: Service<actix_http::Request, Response = ServiceResponse<B>, Error = actix_web::Error>,
{
    let req = TestRequest::post()
        .uri("/auth")
        .insert_header(("X-Real-IP", ip))
        .set_json(json!({"username": user, "password": pass}))
        .to_request();
    call_service(app, req).await
}

#[actix_web::test]
async fn sixth_failed_login_returns_429_with_retry_after() {
    if !has_redis() {
        return;
    }
    let (app, _env) = common::test_app().await;
    for _ in 0..5 {
        assert_eq!(
            login_as(&app, "10.1.0.1", "admin", "bad").await.status(),
            401
        );
    }
    let res = login_as(&app, "10.1.0.1", "admin", "bad").await;
    assert_eq!(res.status(), 429);
    let retry: u64 = res
        .headers()
        .get("retry-after")
        .unwrap()
        .to_str()
        .unwrap()
        .parse()
        .unwrap();
    assert!((1..=900).contains(&retry));
    // blocked even with the right password
    assert_eq!(
        login_as(&app, "10.1.0.1", "admin", "admin-pass")
            .await
            .status(),
        429
    );
}

#[actix_web::test]
async fn successful_login_resets_counter() {
    if !has_redis() {
        return;
    }
    let (app, _env) = common::test_app().await;
    for _ in 0..4 {
        assert_eq!(
            login_as(&app, "10.1.0.2", "admin", "bad").await.status(),
            401
        );
    }
    assert_eq!(
        login_as(&app, "10.1.0.2", "admin", "admin-pass")
            .await
            .status(),
        200
    );
    for _ in 0..5 {
        assert_eq!(
            login_as(&app, "10.1.0.2", "admin", "bad").await.status(),
            401
        );
    }
}

#[actix_web::test]
async fn fourth_register_attempt_returns_429() {
    if !has_redis() {
        return;
    }
    let (app, _env) = common::test_app().await;
    for i in 0..4 {
        let req = TestRequest::post()
            .uri("/register")
            .insert_header(("X-Real-IP", "10.1.0.3"))
            .set_json(json!({"username": format!("u{i}"), "password": "pw", "admin_pass": "wrong", "admin": false}))
            .to_request();
        let res = call_service(&app, req).await;
        if i < 3 {
            assert_eq!(res.status(), 403, "attempt {i}");
        } else {
            assert_eq!(res.status(), 429);
            assert!(res.headers().contains_key("retry-after"));
        }
    }
}

#[actix_web::test]
async fn rate_limit_uses_x_real_ip() {
    if !has_redis() {
        return;
    }
    let (app, _env) = common::test_app().await;
    for _ in 0..6 {
        login_as(&app, "10.1.0.4", "admin", "bad").await;
    }
    assert_eq!(
        login_as(&app, "10.1.0.4", "admin", "bad").await.status(),
        429
    );
    assert_eq!(
        login_as(&app, "10.1.0.5", "admin", "bad").await.status(),
        401
    );
}

async fn insert_row(env: &parameros_api::Env, heading: &str) {
    let mut conn = env.pool.get().await.unwrap();
    let conn: &mut AsyncPgConnection = &mut conn;
    conn.batch_execute(&format!(
        "INSERT INTO posts (uuid, name, heading, content, created_at) \
         VALUES (gen_random_uuid(), 'team', '{heading}', 'C', now())"
    ))
    .await
    .unwrap();
}

async fn count<S, B>(app: &S, uri: &str) -> usize
where
    S: Service<actix_http::Request, Response = ServiceResponse<B>, Error = actix_web::Error>,
    B: actix_web::body::MessageBody,
{
    let res = call_service(app, TestRequest::get().uri(uri).to_request()).await;
    assert_eq!(res.status(), 200);
    assert_eq!(
        res.headers().get("content-type").unwrap(),
        "application/json"
    );
    read_body_json::<Vec<Value>, _>(res).await.len()
}

#[actix_web::test]
async fn posts_cached_then_invalidated_on_create() {
    if !has_redis() {
        return;
    }
    let (app, env) = common::test_app().await;
    let auth = common::login(&app).await;
    insert_row(&env, "one").await;
    assert_eq!(count(&app, "/posts/team").await, 1);
    insert_row(&env, "two").await;
    assert_eq!(count(&app, "/posts/team").await, 1, "served from cache");

    let body = "--XB\r\nContent-Disposition: form-data; name=\"name\"\r\n\r\nteam\r\n\
        --XB\r\nContent-Disposition: form-data; name=\"heading\"\r\n\r\nH\r\n\
        --XB\r\nContent-Disposition: form-data; name=\"content\"\r\n\r\nC\r\n--XB--\r\n";
    let req = TestRequest::post()
        .uri("/post")
        .insert_header((AUTHORIZATION, auth))
        .insert_header(("Content-Type", "multipart/form-data; boundary=XB"))
        .set_payload(body)
        .to_request();
    assert_eq!(call_service(&app, req).await.status(), 200);
    assert_eq!(count(&app, "/posts/team").await, 3);
}

#[actix_web::test]
async fn progress_logs_alias_shares_logbuch_cache() {
    if !has_redis() {
        return;
    }
    let (app, env) = common::test_app().await;
    assert_eq!(count(&app, "/posts/logbuch").await, 0);
    let mut conn = env.pool.get().await.unwrap();
    let conn: &mut AsyncPgConnection = &mut conn;
    conn.batch_execute(
        "INSERT INTO posts (uuid, name, heading, content, created_at) \
         VALUES (gen_random_uuid(), 'logbuch', 'x', 'C', now())",
    )
    .await
    .unwrap();
    assert_eq!(count(&app, "/posts/progress-logs").await, 0, "shared key");
}

#[actix_web::test]
async fn api_works_when_redis_down() {
    if !has_redis() {
        return;
    }
    // Fake Redis that answers every command with an error.
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    std::thread::spawn(move || {
        while let Ok((mut s, _)) = listener.accept() {
            std::thread::spawn(move || {
                use std::io::{Read, Write};
                let mut buf = [0u8; 4096];
                while let Ok(n @ 1..) = s.read(&mut buf) {
                    // One error reply per command (RESP arrays start a line with '*').
                    let cmds = (0..n)
                        .filter(|&i| buf[i] == b'*' && (i == 0 || buf[i - 1] == b'\n'))
                        .count()
                        .max(1);
                    if s.write_all(&b"-ERR down\r\n".repeat(cmds)).is_err() {
                        break;
                    }
                }
            });
        }
    });
    let (_app, mut env) = common::test_app().await;
    env.redis = Some(
        redis::aio::ConnectionManager::new(
            redis::Client::open(format!("redis://127.0.0.1:{port}")).unwrap(),
        )
        .await
        .expect("manager to fake redis"),
    );
    let app = init_service(
        App::new()
            .app_data(Data::new(env))
            .configure(app::configure),
    )
    .await;

    for _ in 0..8 {
        // wrong password repeatedly: never 429
        assert_eq!(
            login_as(&app, "10.1.0.9", "admin", "bad").await.status(),
            401
        );
    }
    assert_eq!(
        login_as(&app, "10.1.0.9", "admin", "admin-pass")
            .await
            .status(),
        200
    );
    let res = call_service(&app, TestRequest::get().uri("/posts/team").to_request()).await;
    assert_eq!(res.status(), 200);
}

#[actix_web::test]
async fn put_invalidates_post_and_list_cache() {
    if !has_redis() {
        return;
    }
    let (app, env) = common::test_app().await;
    let auth = common::login(&app).await;
    insert_row(&env, "old").await;
    let list: Vec<Value> = read_body_json(
        call_service(&app, TestRequest::get().uri("/posts/team").to_request()).await,
    )
    .await;
    let id = list[0]["uuid"].as_str().unwrap().to_owned();
    let uri = format!("/post/{id}");
    call_service(&app, TestRequest::get().uri(&uri).to_request()).await;

    let req = TestRequest::put()
        .uri(&uri)
        .insert_header((AUTHORIZATION, auth))
        .set_json(json!({"heading": "new"}))
        .to_request();
    assert_eq!(call_service(&app, req).await.status(), 202);

    let one: Value =
        read_body_json(call_service(&app, TestRequest::get().uri(&uri).to_request()).await).await;
    assert_eq!(one["heading"], "new");
    let list: Vec<Value> = read_body_json(
        call_service(&app, TestRequest::get().uri("/posts/team").to_request()).await,
    )
    .await;
    assert_eq!(list[0]["heading"], "new");
}
