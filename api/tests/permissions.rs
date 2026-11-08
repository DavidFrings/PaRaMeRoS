mod common;

use actix_web::{
    body::MessageBody,
    dev::{Service, ServiceResponse},
    http::header::AUTHORIZATION,
    test::{TestRequest, call_service, read_body_json},
};
use diesel_async::{AsyncPgConnection, RunQueryDsl, SimpleAsyncConnection};
use serde_json::{Value, json};
use std::fmt::Write as _;

const ALL: [&str; 5] = ["logbuch", "team", "coaches", "forest-project", "tournament"];

async fn register<S, B>(app: &S, user: &str, permissions: Value) -> u16
where
    S: Service<actix_http::Request, Response = ServiceResponse<B>, Error = actix_web::Error>,
{
    let req = TestRequest::post()
        .uri("/register")
        .set_json(json!({
            "username": user, "password": "pw-123456", "admin": false,
            "admin_pass": "admin-pass", "permissions": permissions,
        }))
        .to_request();
    call_service(app, req).await.status().as_u16()
}

async fn login_as<S, B>(app: &S, user: &str) -> String
where
    S: Service<actix_http::Request, Response = ServiceResponse<B>, Error = actix_web::Error>,
    B: MessageBody,
{
    let req = TestRequest::post()
        .uri("/auth")
        .set_json(json!({"username": user, "password": "pw-123456"}))
        .to_request();
    let res = call_service(app, req).await;
    assert_eq!(res.status(), 200);
    res.headers()
        .get("authorization")
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
}

/// Register `editor` with logbuch + tournament and log in.
async fn editor<S, B>(app: &S) -> String
where
    S: Service<actix_http::Request, Response = ServiceResponse<B>, Error = actix_web::Error>,
    B: MessageBody,
{
    assert_eq!(
        register(app, "editor", json!(["logbuch", "tournament"])).await,
        201
    );
    login_as(app, "editor").await
}

async fn create<S, B>(app: &S, auth: &str, section: &str) -> ServiceResponse<B>
where
    S: Service<actix_http::Request, Response = ServiceResponse<B>, Error = actix_web::Error>,
{
    let mut body = String::new();
    for (k, v) in [("name", section), ("heading", "H"), ("content", "C")] {
        // Writing to a String cannot fail.
        let _ = write!(
            body,
            "--XB\r\nContent-Disposition: form-data; name=\"{k}\"\r\n\r\n{v}\r\n"
        );
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

async fn create_id<S, B>(app: &S, auth: &str, section: &str) -> String
where
    S: Service<actix_http::Request, Response = ServiceResponse<B>, Error = actix_web::Error>,
    B: MessageBody,
{
    let res = create(app, auth, section).await;
    assert_eq!(res.status(), 200, "create {section}");
    read_body_json::<Value, _>(res).await["id"]
        .as_str()
        .unwrap()
        .to_owned()
}

fn put_post(auth: &str, id: &str, body: Value) -> actix_http::Request {
    TestRequest::put()
        .uri(&format!("/post/{id}"))
        .insert_header((AUTHORIZATION, auth))
        .set_json(body)
        .to_request()
}

fn delete(auth: &str, uri: &str) -> actix_http::Request {
    TestRequest::delete()
        .uri(uri)
        .insert_header((AUTHORIZATION, auth))
        .to_request()
}

fn put_media(auth: &str, id: &str) -> actix_http::Request {
    TestRequest::put()
        .uri(&format!("/post/{id}/media"))
        .insert_header((AUTHORIZATION, auth))
        .insert_header(("Content-Type", "multipart/form-data; boundary=XB"))
        .set_payload(
            "--XB\r\nContent-Disposition: form-data; name=\"media_type\"\r\n\r\nimg\r\n--XB--\r\n",
        )
        .to_request()
}

fn put_tournament(auth: &str) -> actix_http::Request {
    TestRequest::put()
        .uri("/tournament")
        .insert_header((AUTHORIZATION, auth))
        .set_json(json!({"name": "Cup", "starts_at": "2027-03-28T10:00:00+02:00"}))
        .to_request()
}

async fn verify_json<S, B>(app: &S, auth: &str) -> Value
where
    S: Service<actix_http::Request, Response = ServiceResponse<B>, Error = actix_web::Error>,
    B: MessageBody,
{
    let req = TestRequest::get()
        .uri("/auth/verify")
        .insert_header((AUTHORIZATION, auth))
        .to_request();
    let res = call_service(app, req).await;
    assert_eq!(res.status(), 200);
    read_body_json(res).await
}

#[actix_web::test]
async fn limited_user_manages_logbuch_and_tournament() {
    let Some((app, _env)) = common::test_app().await else {
        return;
    };
    let auth = editor(&app).await;

    // `progress-logs` is the logbuch alias and needs the logbuch permission.
    let alias = create_id(&app, &auth, "progress-logs").await;
    let id = create_id(&app, &auth, "logbuch").await;
    let res = call_service(&app, put_post(&auth, &id, json!({"heading": "Neu"}))).await;
    assert_eq!(res.status(), 202);
    let res = call_service(&app, delete(&auth, &format!("/post/{id}/media"))).await;
    assert_eq!(res.status(), 204);
    // Permission passes; the empty form then fails validation.
    assert_eq!(
        call_service(&app, put_media(&auth, &id)).await.status(),
        400
    );
    for p in [&id, &alias] {
        let res = call_service(&app, delete(&auth, &format!("/post/{p}"))).await;
        assert_eq!(res.status(), 202);
    }

    assert_eq!(
        call_service(&app, put_tournament(&auth)).await.status(),
        200
    );
    let res = call_service(&app, delete(&auth, "/tournament")).await;
    assert_eq!(res.status(), 204);
}

#[actix_web::test]
async fn limited_user_gets_403_outside_permissions() {
    let Some((app, _env)) = common::test_app().await else {
        return;
    };
    let admin = common::login(&app).await;
    let auth = editor(&app).await;

    assert_eq!(create(&app, &auth, "forest-project").await.status(), 403);
    assert_eq!(
        call_service(
            &app,
            TestRequest::get().uri("/posts/forest-project").to_request()
        )
        .await
        .status(),
        200
    );

    let team = create_id(&app, &admin, "team").await;
    let res = call_service(&app, put_post(&auth, &team, json!({"heading": "x"}))).await;
    assert_eq!(res.status(), 403);
    let res = call_service(&app, delete(&auth, &format!("/post/{team}"))).await;
    assert_eq!(res.status(), 403);
    let res = call_service(&app, delete(&auth, &format!("/post/{team}/media"))).await;
    assert_eq!(res.status(), 403);
    assert_eq!(
        call_service(&app, put_media(&auth, &team)).await.status(),
        403
    );

    // Moving a permitted post into a forbidden section is a write to that section.
    let log = create_id(&app, &auth, "logbuch").await;
    let res = call_service(
        &app,
        put_post(&auth, &log, json!({"name": "forest-project"})),
    )
    .await;
    assert_eq!(res.status(), 403);
    let res = call_service(
        &app,
        TestRequest::get().uri(&format!("/post/{log}")).to_request(),
    )
    .await;
    assert_eq!(read_body_json::<Value, _>(res).await["name"], "logbuch");

    // Unknown posts stay 404, before any permission check.
    let res = call_service(
        &app,
        delete(&auth, &format!("/post/{}", uuid::Uuid::new_v4())),
    )
    .await;
    assert_eq!(res.status(), 404);
}

#[actix_web::test]
async fn user_without_tournament_permission_gets_403() {
    let Some((app, _env)) = common::test_app().await else {
        return;
    };
    assert_eq!(register(&app, "writer", json!(["team"])).await, 201);
    let auth = login_as(&app, "writer").await;
    assert_eq!(
        call_service(&app, put_tournament(&auth)).await.status(),
        403
    );
    let res = call_service(&app, delete(&auth, "/tournament")).await;
    assert_eq!(res.status(), 403);
    assert_eq!(create(&app, &auth, "logbuch").await.status(), 403);
    assert_eq!(create(&app, &auth, "team").await.status(), 200);
}

#[actix_web::test]
async fn admin_can_write_everything() {
    let Some((app, _env)) = common::test_app().await else {
        return;
    };
    let admin = common::login(&app).await;
    for s in ["logbuch", "team", "coaches", "forest-project"] {
        let id = create_id(&app, &admin, s).await;
        let res = call_service(&app, put_post(&admin, &id, json!({"name": "team"}))).await;
        assert_eq!(res.status(), 202, "{s}");
        let res = call_service(&app, delete(&admin, &format!("/post/{id}"))).await;
        assert_eq!(res.status(), 202, "{s}");
    }
    assert_eq!(
        call_service(&app, put_tournament(&admin)).await.status(),
        200
    );
    let res = call_service(&app, delete(&admin, "/tournament")).await;
    assert_eq!(res.status(), 204);
}

#[actix_web::test]
async fn verify_returns_permissions() {
    let Some((app, _env)) = common::test_app().await else {
        return;
    };
    let admin = common::login(&app).await;
    assert_eq!(
        verify_json(&app, &admin).await,
        json!({"admin": true, "permissions": ALL})
    );
    let auth = editor(&app).await;
    assert_eq!(
        verify_json(&app, &auth).await,
        json!({"admin": false, "permissions": ["logbuch", "tournament"]})
    );
}

#[actix_web::test]
async fn register_validates_and_dedupes_permissions() {
    let Some((app, _env)) = common::test_app().await else {
        return;
    };
    assert_eq!(register(&app, "bad", json!(["logbuch", "evil"])).await, 400);
    assert_eq!(
        register(&app, "dup", json!(["team", "team", "logbuch"])).await,
        201
    );
    let auth = login_as(&app, "dup").await;
    assert_eq!(
        verify_json(&app, &auth).await["permissions"],
        json!(["team", "logbuch"])
    );
}

#[actix_web::test]
async fn register_without_permissions_field_stores_none() {
    let Some((app, _env)) = common::test_app().await else {
        return;
    };
    let req = TestRequest::post()
        .uri("/register")
        .set_json(json!({
            "username": "old", "password": "pw-123456", "admin": false, "admin_pass": "admin-pass",
        }))
        .to_request();
    assert_eq!(call_service(&app, req).await.status(), 201);
    let auth = login_as(&app, "old").await;
    assert_eq!(verify_json(&app, &auth).await["permissions"], json!([]));
}

const W05_UP: &str = include_str!("../migrations/w05-user-permissions/up.sql");
const W05_DOWN: &str = include_str!("../migrations/w05-user-permissions/down.sql");

#[actix_web::test]
async fn migration_w05_down_up_roundtrip_backfills_non_admins() {
    async fn text(conn: &mut AsyncPgConnection, sql: &str) -> String {
        diesel::select(diesel::dsl::sql::<diesel::sql_types::Text>(sql))
            .get_result(conn)
            .await
            .unwrap()
    }
    let Some((_app, env)) = common::test_app().await else {
        return;
    };
    let mut conn = env.pool.get().await.unwrap();
    let conn: &mut AsyncPgConnection = &mut conn;
    let col = "count(*)::text FROM information_schema.columns \
        WHERE table_name='users' AND column_name='permissions'";

    conn.batch_execute(W05_DOWN).await.unwrap();
    assert_eq!(text(conn, col).await, "0");
    conn.batch_execute("INSERT INTO users (username,password,admin) VALUES ('old','x',false)")
        .await
        .unwrap();

    conn.batch_execute(W05_UP).await.unwrap();
    assert_eq!(text(conn, col).await, "1");
    assert_eq!(
        text(conn, "permissions::text FROM users WHERE username='old'").await,
        "{logbuch,team,coaches,forest-project,tournament}"
    );
    assert_eq!(
        text(conn, "permissions::text FROM users WHERE username='admin'").await,
        "{}"
    );

    conn.batch_execute(W05_DOWN).await.unwrap();
    assert_eq!(text(conn, col).await, "0");
    conn.batch_execute(W05_UP).await.unwrap(); // leave schema as migrated
}

async fn sql(env: &parameros_api::Env, stmt: &str) {
    let mut conn = env.pool.get().await.unwrap();
    let conn: &mut AsyncPgConnection = &mut conn;
    conn.batch_execute(stmt).await.unwrap();
}

fn upload_files(env: &parameros_api::Env) -> usize {
    std::fs::read_dir(&env.upload_dir).unwrap().count()
}

fn multipart_req(uri: &str, method: &str, auth: &str, body: Vec<u8>) -> actix_http::Request {
    let req = match method {
        "PUT" => TestRequest::put(),
        _ => TestRequest::post(),
    };
    req.uri(uri)
        .insert_header((AUTHORIZATION, auth))
        .insert_header(("Content-Type", "multipart/form-data; boundary=XB"))
        .set_payload(body)
        .to_request()
}

#[actix_web::test]
async fn null_permission_elements_grant_nothing() {
    let Some((app, env)) = common::test_app().await else {
        return;
    };
    let auth = editor(&app).await;
    sql(
        &env,
        "UPDATE users SET permissions = ARRAY['logbuch', NULL] WHERE username = 'editor'",
    )
    .await;
    assert_eq!(create(&app, &auth, "logbuch").await.status(), 200);
    assert_eq!(create(&app, &auth, "forest-project").await.status(), 403);
    assert_eq!(
        verify_json(&app, &auth).await,
        json!({"admin": false, "permissions": ["logbuch"]})
    );
}

#[actix_web::test]
async fn user_with_no_permissions_cannot_write() {
    let Some((app, _env)) = common::test_app().await else {
        return;
    };
    let admin = common::login(&app).await;
    let post = create_id(&app, &admin, "logbuch").await;
    assert_eq!(register(&app, "nobody", json!([])).await, 201);
    let auth = login_as(&app, "nobody").await;
    assert_eq!(create(&app, &auth, "logbuch").await.status(), 403);
    assert_eq!(
        call_service(&app, put_tournament(&auth)).await.status(),
        403
    );
    let res = call_service(&app, delete(&auth, &format!("/post/{post}"))).await;
    assert_eq!(res.status(), 403);
}

#[actix_web::test]
async fn revoked_permission_applies_to_existing_token() {
    let Some((app, env)) = common::test_app().await else {
        return;
    };
    let auth = editor(&app).await;
    assert_eq!(create(&app, &auth, "logbuch").await.status(), 200);
    sql(
        &env,
        "UPDATE users SET permissions = '{}' WHERE username = 'editor'",
    )
    .await;
    assert_eq!(create(&app, &auth, "logbuch").await.status(), 403);
    sql(&env, "DELETE FROM users WHERE username = 'editor'").await;
    assert_eq!(create(&app, &auth, "logbuch").await.status(), 401);
}

#[actix_web::test]
async fn forbidden_uploads_write_no_file() {
    let Some((app, env)) = common::test_app().await else {
        return;
    };
    let admin = common::login(&app).await;
    let team = create_id(&app, &admin, "team").await;
    let auth = editor(&app).await;
    let img = common::encoded(400, 300, image::ImageFormat::Png);

    let body = common::multipart(
        &[
            ("name", "forest-project"),
            ("heading", "H"),
            ("content", "C"),
            ("media_type", "img"),
            ("media_creator", "me"),
        ],
        Some(("p.png", &img)),
    );
    let res = call_service(&app, multipart_req("/post", "POST", &auth, body)).await;
    assert_eq!(res.status(), 403);

    let body = common::multipart(
        &[("media_type", "img"), ("media_creator", "me")],
        Some(("p.png", &img)),
    );
    let uri = format!("/post/{team}/media");
    let res = call_service(&app, multipart_req(&uri, "PUT", &auth, body)).await;
    assert_eq!(res.status(), 403);
    assert_eq!(upload_files(&env), 0);
}

#[actix_web::test]
async fn post_without_any_section_permission_is_rejected_early() {
    let Some((app, _env)) = common::test_app().await else {
        return;
    };
    assert_eq!(register(&app, "cup", json!(["tournament"])).await, 201);
    let auth = login_as(&app, "cup").await;
    let img = common::encoded(1600, 1200, image::ImageFormat::Png);
    let body = common::multipart(
        &[("media_type", "img"), ("media_creator", "me")],
        Some(("big.png", &img)),
    );
    // No `name` field: only the early check can answer 403 instead of 400.
    let res = call_service(&app, multipart_req("/post", "POST", &auth, body)).await;
    assert_eq!(res.status(), 403);
}
