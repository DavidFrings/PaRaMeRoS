mod common;

use actix_web::{
    App,
    http::header::AUTHORIZATION,
    test::{TestRequest, call_and_read_body, call_service, init_service, read_body},
    web::Data,
};
use chrono::{Duration, Utc};
use diesel_async::RunQueryDsl;
use parameros_api::{
    app,
    db::{
        models::NewPost,
        schema::{posts, users},
    },
};
use std::time::Instant;
use uuid::Uuid;

fn new_post(name: &str, heading: &str, media_name: Option<&str>) -> NewPost {
    NewPost {
        uuid: Uuid::new_v4(),
        name: name.into(),
        heading: heading.into(),
        content: "c".into(),
        media_type: media_name.map(|_| "image".into()),
        media_name: media_name.map(Into::into),
        media_creator: media_name.map(|_| "x".into()),
        created_at: Utc::now().naive_utc(),
        updated_at: None,
        heading_en: None,
        content_en: None,
        season: None,
    }
}

async fn cors_app() -> impl actix_web::dev::Service<
    actix_http::Request,
    Response = actix_web::dev::ServiceResponse<impl actix_web::body::MessageBody>,
    Error = actix_web::Error,
> {
    let (_, env) = common::test_app().await;
    init_service(
        App::new()
            .wrap(app::cors(&["http://localhost:3000".to_string()]))
            .app_data(Data::new(env))
            .configure(app::configure),
    )
    .await
}

fn preflight(origin: &str) -> actix_http::Request {
    TestRequest::default()
        .method(actix_web::http::Method::OPTIONS)
        .uri("/post")
        .insert_header(("Origin", origin))
        .insert_header(("Access-Control-Request-Method", "POST"))
        .to_request()
}

#[actix_web::test]
async fn cors_rejects_empty_origin() {
    let app = cors_app().await;
    let res = call_service(&app, preflight("")).await;
    assert!(res.headers().get("access-control-allow-origin").is_none());
}

#[actix_web::test]
async fn cors_allows_configured_origin() {
    let app = cors_app().await;
    let res = call_service(&app, preflight("http://localhost:3000")).await;
    assert_eq!(
        res.headers().get("access-control-allow-origin").unwrap(),
        "http://localhost:3000"
    );
}

#[actix_web::test]
async fn get_unknown_post_returns_404() {
    let (app, _env) = common::test_app().await;
    let req = TestRequest::get()
        .uri(&format!("/post/{}", Uuid::new_v4()))
        .to_request();
    let res = call_service(&app, req).await;
    assert_eq!(res.status(), 404);
}

#[actix_web::test]
async fn db_errors_do_not_leak() {
    let (app, _env) = common::test_app().await;
    let req = TestRequest::get()
        .uri(&format!("/post/{}", Uuid::new_v4()))
        .to_request();
    let res = call_service(&app, req).await;
    let body = String::from_utf8(read_body(res).await.to_vec())
        .unwrap()
        .to_lowercase();
    assert!(!body.contains("diesel") && !body.contains("db"), "{body}");
}

#[actix_web::test]
async fn verify_rejects_token_without_bearer_prefix() {
    let (app, _env) = common::test_app().await;
    let token = common::login(&app).await;
    let raw = token.strip_prefix("Bearer ").unwrap();
    let req = TestRequest::delete()
        .uri(&format!("/post/{}", Uuid::new_v4()))
        .insert_header((AUTHORIZATION, raw))
        .to_request();
    assert_eq!(call_service(&app, req).await.status(), 401);
}

#[actix_web::test]
async fn verify_rejects_deleted_user() {
    let (app, env) = common::test_app().await;
    let token = common::login(&app).await;
    {
        let mut conn = env.pool.get().await.unwrap();
        diesel::delete(users::table)
            .execute(&mut *conn)
            .await
            .unwrap();
    }
    let req = TestRequest::delete()
        .uri(&format!("/post/{}", Uuid::new_v4()))
        .insert_header((AUTHORIZATION, token))
        .to_request();
    assert_eq!(call_service(&app, req).await.status(), 401);
}

#[actix_web::test]
async fn delete_post_with_missing_file_succeeds() {
    let (app, env) = common::test_app().await;
    let token = common::login(&app).await;
    let post = new_post("logbuch", "h", Some("gone.webp"));
    let id = post.uuid;
    {
        let mut conn = env.pool.get().await.unwrap();
        diesel::insert_into(posts::table)
            .values(&post)
            .execute(&mut *conn)
            .await
            .unwrap();
    }
    let req = TestRequest::delete()
        .uri(&format!("/post/{id}"))
        .insert_header((AUTHORIZATION, token))
        .to_request();
    assert_eq!(call_service(&app, req).await.status(), 202);
}

#[actix_web::test]
async fn posts_sorted_newest_first() {
    let (app, env) = common::test_app().await;
    let mut old = new_post("logbuch", "old", None);
    old.created_at = (Utc::now() - Duration::minutes(1)).naive_utc();
    let new = new_post("logbuch", "new", None);
    {
        let mut conn = env.pool.get().await.unwrap();
        // insert the older one last so insertion order cannot explain the result
        diesel::insert_into(posts::table)
            .values(&new)
            .execute(&mut *conn)
            .await
            .unwrap();
        diesel::insert_into(posts::table)
            .values(&old)
            .execute(&mut *conn)
            .await
            .unwrap();
    }
    let req = TestRequest::get().uri("/posts/logbuch").to_request();
    let body: serde_json::Value =
        serde_json::from_slice(&call_and_read_body(&app, req).await).unwrap();
    assert_eq!(body[0]["heading"], "new");
    assert_eq!(body[1]["heading"], "old");
}

#[actix_web::test]
async fn login_unknown_user_takes_bcrypt_time() {
    let (app, _env) = common::test_app().await;
    for i in 1..=10 {
        let req = TestRequest::post()
            .uri("/auth")
            .insert_header(("X-Real-IP", format!("10.0.0.{i}")))
            .set_json(serde_json::json!({"username": "nobody", "password": "x"}))
            .to_request();
        let start = Instant::now();
        let res = call_service(&app, req).await;
        let elapsed = start.elapsed();
        assert_eq!(res.status(), 401);
        assert!(elapsed.as_millis() >= 50, "{elapsed:?}");
    }
}
