#![allow(dead_code)]

use actix_web::{
    App,
    body::MessageBody,
    dev::{Service, ServiceResponse},
    test::{TestRequest, call_service, init_service},
    web::Data,
};
use diesel_async::{AsyncPgConnection, RunQueryDsl, SimpleAsyncConnection};
use image::{ImageBuffer, ImageFormat, Rgb, RgbImage};
use parameros_api::{
    Env, app,
    db::{connection, models::NewUser, schema::users},
    migrate::run_migrations,
};
use std::io::Cursor;

pub fn site_url() -> &'static str {
    "https://example.test"
}

/// `None` without `TEST_DATABASE_URL`: the test then returns early. The rust-ci `test` job has no
/// Postgres; the `API / Test` job sets the variable and `REQUIRE_TEST_DB`, which turns a skip into a
/// failure.
pub async fn test_app() -> Option<(
    impl Service<
        actix_http::Request,
        Response = ServiceResponse<impl MessageBody>,
        Error = actix_web::Error,
    >,
    Env,
)> {
    test_app_with_site(site_url()).await
}

pub async fn test_app_with_site(
    site: &str,
) -> Option<(
    impl Service<
        actix_http::Request,
        Response = ServiceResponse<impl MessageBody>,
        Error = actix_web::Error,
    >,
    Env,
)> {
    let Ok(db_url) = std::env::var("TEST_DATABASE_URL") else {
        assert!(
            std::env::var_os("REQUIRE_TEST_DB").is_none(),
            "REQUIRE_TEST_DB is set but TEST_DATABASE_URL is missing"
        );
        eprintln!("TEST_DATABASE_URL not set, skipping");
        return None;
    };
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
                permissions: vec![],
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
        site_url: site.into(),
    };

    let app = init_service(
        App::new()
            .app_data(Data::new(env.clone()))
            .configure(app::configure),
    )
    .await;
    Some((app, env))
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

/// In-memory test image in the given format.
pub fn encoded(w: u32, h: u32, fmt: ImageFormat) -> Vec<u8> {
    let img: RgbImage = ImageBuffer::from_fn(w, h, |x, y| {
        Rgb([
            u8::try_from(x % 256).unwrap(),
            u8::try_from(y % 256).unwrap(),
            128,
        ])
    });
    let mut out = Cursor::new(Vec::new());
    img.write_to(&mut out, fmt).expect("encode fixture");
    out.into_inner()
}

/// Multipart body with boundary `XB`; the file goes in the `media` part.
pub fn multipart(fields: &[(&str, &str)], file: Option<(&str, &[u8])>) -> Vec<u8> {
    let mut body = Vec::new();
    for (k, v) in fields {
        body.extend(
            format!("--XB\r\nContent-Disposition: form-data; name=\"{k}\"\r\n\r\n{v}\r\n")
                .as_bytes(),
        );
    }
    if let Some((filename, data)) = file {
        body.extend(
            format!(
                "--XB\r\nContent-Disposition: form-data; name=\"media\"; filename=\"{filename}\"\r\nContent-Type: application/octet-stream\r\n\r\n"
            )
            .as_bytes(),
        );
        body.extend(data);
        body.extend(b"\r\n");
    }
    body.extend(b"--XB--\r\n");
    body
}
