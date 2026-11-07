#![allow(dead_code)]

use actix_web::{
    App,
    body::MessageBody,
    dev::{Service, ServiceResponse},
    test::{TestRequest, call_service, init_service},
    web::Data,
};
use diesel_async::{AsyncPgConnection, RunQueryDsl, SimpleAsyncConnection};
use parameros_api::{
    Env, app,
    db::{connection, models::NewUser, schema::users},
    migrate::run_migrations,
};

pub fn site_url() -> &'static str {
    "https://example.test"
}

pub async fn test_app() -> (
    impl Service<
        actix_http::Request,
        Response = ServiceResponse<impl MessageBody>,
        Error = actix_web::Error,
    >,
    Env,
) {
    let db_url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL not set");
    run_migrations(&db_url).await.expect("migrations failed");

    let pool = connection::establish(db_url).await;
    {
        let mut conn = pool.get().await.expect("db connection");
        let conn: &mut AsyncPgConnection = &mut conn;
        conn.batch_execute("TRUNCATE posts, users, tournament RESTART IDENTITY CASCADE")
            .await
            .expect("truncate");
        diesel::insert_into(users::table)
            .values(NewUser {
                username: "admin".into(),
                password: bcrypt::hash("admin-pass", 4).expect("hash"),
                admin: true,
            })
            .execute(conn)
            .await
            .expect("insert admin");
    }

    let redis = match std::env::var("TEST_REDIS_URL") {
        Ok(url) => {
            let mut manager =
                redis::aio::ConnectionManager::new(redis::Client::open(url).expect("redis url"))
                    .await
                    .expect("redis connect");
            // Keep rate-limit and cache keys from leaking between tests.
            redis::cmd("FLUSHDB")
                .query_async::<()>(&mut manager)
                .await
                .expect("flushdb");
            Some(manager)
        }
        Err(_) => None,
    };

    let env = Env {
        pool,
        jwt_secret: "test-secret".into(),
        jwt_expire: 5,
        admin_pass: "admin-pass".into(),
        cors_origins: vec![],
        // Unique per test so file assertions can't see other tests' uploads.
        upload_dir: tempfile::Builder::new()
            .prefix("parameros-test-uploads-")
            .tempdir()
            .expect("upload dir")
            .keep(),
        redis,
        site_url: site_url().into(),
    };

    let app = init_service(
        App::new()
            .app_data(Data::new(env.clone()))
            .configure(app::configure),
    )
    .await;
    (app, env)
}

/// Log in as the seeded admin and return the `Authorization` header value.
pub async fn login<S, B>(app: &S) -> String
where
    S: Service<actix_http::Request, Response = ServiceResponse<B>, Error = actix_web::Error>,
    B: MessageBody,
{
    let req = TestRequest::post()
        .uri("/auth")
        .set_json(serde_json::json!({"username": "admin", "password": "admin-pass"}))
        .to_request();
    let res = call_service(app, req).await;
    assert_eq!(res.status(), 200);
    res.headers()
        .get("authorization")
        .expect("authorization header")
        .to_str()
        .unwrap()
        .to_owned()
}
