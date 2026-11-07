mod common;

use actix_web::{
    dev::{Service, ServiceResponse},
    http::header::AUTHORIZATION,
    test::{TestRequest, call_service, read_body_json},
};
use diesel_async::{AsyncPgConnection, RunQueryDsl, SimpleAsyncConnection};
use serde_json::{Value, json};

async fn create<S, B>(app: &S, auth: &str, fields: &[(&str, &str)]) -> ServiceResponse<B>
where
    S: Service<actix_http::Request, Response = ServiceResponse<B>, Error = actix_web::Error>,
{
    let mut body = String::new();
    for (k, v) in fields {
        body += &format!("--XB\r\nContent-Disposition: form-data; name=\"{k}\"\r\n\r\n{v}\r\n");
    }
    body += "--XB--\r\n";
    let req = TestRequest::post()
        .uri("/post")
        .insert_header((AUTHORIZATION, auth))
        .insert_header(("Content-Type", "multipart/form-data; boundary=XB"))
        .set_payload(body)
        .to_request();
    call_service(app, req).await
}

async fn get_json<S, B>(app: &S, uri: &str) -> Value
where
    S: Service<actix_http::Request, Response = ServiceResponse<B>, Error = actix_web::Error>,
    B: actix_web::body::MessageBody,
{
    let res = call_service(app, TestRequest::get().uri(uri).to_request()).await;
    assert_eq!(res.status(), 200, "{uri}");
    read_body_json(res).await
}

const BASE: [(&str, &str); 3] = [("name", "team"), ("heading", "H"), ("content", "C")];

#[actix_web::test]
async fn create_post_with_english_and_season() {
    let (app, _env) = common::test_app().await;
    let auth = common::login(&app).await;
    let mut f = BASE.to_vec();
    f.extend([
        ("heading_en", "Team"),
        ("content_en", "Text"),
        ("season", "2017/18"),
    ]);
    let res = create(&app, &auth, &f).await;
    assert_eq!(res.status(), 200);
    let id = read_body_json::<Value, _>(res).await["id"].clone();
    let p = get_json(&app, &format!("/post/{}", id.as_str().unwrap())).await;
    assert_eq!(p["heading_en"], "Team");
    assert_eq!(p["content_en"], "Text");
    assert_eq!(p["season"], "2017/18");
}

#[actix_web::test]
async fn progress_logs_alias_reads_logbuch() {
    let (app, _env) = common::test_app().await;
    let auth = common::login(&app).await;
    let res = create(
        &app,
        &auth,
        &[
            ("name", "progress-logs"),
            ("heading", "H"),
            ("content", "C"),
        ],
    )
    .await;
    assert_eq!(res.status(), 200);
    for section in ["progress-logs", "logbuch"] {
        let list = get_json(&app, &format!("/posts/{section}")).await;
        assert_eq!(list.as_array().unwrap().len(), 1, "{section}");
        assert_eq!(list[0]["name"], "logbuch");
    }
}

#[actix_web::test]
async fn unknown_section_rejected() {
    let (app, _env) = common::test_app().await;
    let auth = common::login(&app).await;
    let res = create(
        &app,
        &auth,
        &[("name", "evil"), ("heading", "H"), ("content", "C")],
    )
    .await;
    assert_eq!(res.status(), 400);
    let res = call_service(&app, TestRequest::get().uri("/posts/evil").to_request()).await;
    assert_eq!(res.status(), 400);
}

#[actix_web::test]
async fn invalid_season_rejected() {
    let (app, _env) = common::test_app().await;
    let auth = common::login(&app).await;
    for s in ["2017/19", "17/18"] {
        let mut f = BASE.to_vec();
        f.push(("season", s));
        assert_eq!(create(&app, &auth, &f).await.status(), 400, "{s}");
    }
}

#[actix_web::test]
async fn put_clears_english_with_empty_string() {
    let (app, _env) = common::test_app().await;
    let auth = common::login(&app).await;
    let mut f = BASE.to_vec();
    f.push(("heading_en", "Team"));
    let res = create(&app, &auth, &f).await;
    let id = read_body_json::<Value, _>(res).await["id"]
        .as_str()
        .unwrap()
        .to_string();
    let req = TestRequest::put()
        .uri(&format!("/post/{id}"))
        .insert_header((AUTHORIZATION, auth.as_str()))
        .set_json(json!({"heading_en": ""}))
        .to_request();
    assert_eq!(call_service(&app, req).await.status(), 202);
    let p = get_json(&app, &format!("/post/{id}")).await;
    assert!(p["heading_en"].is_null());
    assert!(p.as_object().unwrap().contains_key("season"));
}

#[actix_web::test]
async fn old_clients_still_work() {
    let (app, _env) = common::test_app().await;
    let auth = common::login(&app).await;
    let res = create(&app, &auth, &BASE).await;
    assert_eq!(res.status(), 200);
    assert!(read_body_json::<Value, _>(res).await["id"].is_string());
}

const W01_UP: &str = include_str!("../migrations/w01-post-fields/up.sql");
const W01_DOWN: &str = include_str!("../migrations/w01-post-fields/down.sql");
const W02_UP: &str = include_str!("../migrations/w02-logbuch/up.sql");
const W02_DOWN: &str = include_str!("../migrations/w02-logbuch/down.sql");

#[actix_web::test]
async fn migrations_w01_w02_down_up_roundtrip() {
    let (_app, env) = common::test_app().await;
    let mut conn = env.pool.get().await.unwrap();
    let conn: &mut AsyncPgConnection = &mut conn;
    let cols = "count(*) FROM information_schema.columns \
        WHERE table_name='posts' AND column_name IN ('heading_en','content_en','season')";
    async fn count(conn: &mut AsyncPgConnection, sql: &str) -> i64 {
        diesel::select(diesel::dsl::sql::<diesel::sql_types::BigInt>(sql))
            .get_result(conn)
            .await
            .unwrap()
    }

    conn.batch_execute(
        "INSERT INTO posts (uuid,name,heading,content,created_at) \
         VALUES (gen_random_uuid(),'logbuch','h','c',now())",
    )
    .await
    .unwrap();

    conn.batch_execute(W02_DOWN).await.unwrap();
    assert_eq!(
        count(conn, "count(*) FROM posts WHERE name='progress-logs'").await,
        1
    );
    conn.batch_execute(W01_DOWN).await.unwrap();
    assert_eq!(count(conn, cols).await, 0);

    conn.batch_execute(W01_UP).await.unwrap();
    assert_eq!(count(conn, cols).await, 3);
    conn.batch_execute(W02_UP).await.unwrap();
    assert_eq!(
        count(conn, "count(*) FROM posts WHERE name='logbuch'").await,
        1
    );
}

#[actix_web::test]
async fn put_unknown_post_returns_404() {
    let (app, _env) = common::test_app().await;
    let auth = common::login(&app).await;
    let req = TestRequest::put()
        .uri(&format!("/post/{}", uuid::Uuid::new_v4()))
        .insert_header((AUTHORIZATION, auth))
        .set_json(json!({ "heading": "x" }))
        .to_request();
    assert_eq!(call_service(&app, req).await.status(), 404);
}
