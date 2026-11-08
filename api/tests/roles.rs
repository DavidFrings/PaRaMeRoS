mod common;

use actix_web::{
    body::MessageBody,
    dev::{Service, ServiceResponse},
    http::header::AUTHORIZATION,
    test::{TestRequest, call_service, read_body, read_body_json},
};
use diesel_async::{AsyncPgConnection, RunQueryDsl, SimpleAsyncConnection};
use serde_json::{Value, json};

const ALL: [&str; 5] = ["logbuch", "team", "coaches", "forest-project", "tournament"];

fn register_req(body: &Value) -> actix_http::Request {
    TestRequest::post()
        .uri("/register")
        .set_json(body)
        .to_request()
}

/// Register `user` with `role` (`None` leaves the field out) and return the status.
async fn register<S, B>(app: &S, user: &str, role: Option<&str>) -> u16
where
    S: Service<actix_http::Request, Response = ServiceResponse<B>, Error = actix_web::Error>,
{
    let mut body = json!({"username": user, "password": "pw-123456", "admin_pass": "admin-pass"});
    if let Some(role) = role {
        body["role"] = json!(role);
    }
    call_service(app, register_req(&body))
        .await
        .status()
        .as_u16()
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

/// Register `user` with `role` and log in.
async fn user_with<S, B>(app: &S, user: &str, role: &str) -> String
where
    S: Service<actix_http::Request, Response = ServiceResponse<B>, Error = actix_web::Error>,
    B: MessageBody,
{
    assert_eq!(register(app, user, Some(role)).await, 201);
    login_as(app, user).await
}

async fn create<S, B>(app: &S, auth: &str, section: &str) -> ServiceResponse<B>
where
    S: Service<actix_http::Request, Response = ServiceResponse<B>, Error = actix_web::Error>,
{
    let body = common::multipart(
        &[("name", section), ("heading", "H"), ("content", "C")],
        None,
    );
    call_service(app, multipart_req("/post", "POST", auth, body)).await
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
    multipart_req(
        &format!("/post/{id}/media"),
        "PUT",
        auth,
        b"--XB\r\nContent-Disposition: form-data; name=\"media_type\"\r\n\r\nimg\r\n--XB--\r\n"
            .to_vec(),
    )
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

async fn get_post<S, B>(app: &S, id: &str) -> Value
where
    S: Service<actix_http::Request, Response = ServiceResponse<B>, Error = actix_web::Error>,
    B: MessageBody,
{
    let req = TestRequest::get().uri(&format!("/post/{id}")).to_request();
    read_body_json(call_service(app, req).await).await
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
async fn normal_writes_logbuch_coaches_team_and_tournament() {
    let Some((app, _env)) = common::test_app().await else {
        return;
    };
    let auth = user_with(&app, "editor", "normal").await;

    // `progress-logs` is the logbuch alias.
    let alias = create_id(&app, &auth, "progress-logs").await;
    let id = create_id(&app, &auth, "logbuch").await;
    let res = call_service(&app, put_post(&auth, &id, json!({"heading": "Neu"}))).await;
    assert_eq!(res.status(), 202);
    let res = call_service(&app, delete(&auth, &format!("/post/{id}/media"))).await;
    assert_eq!(res.status(), 204);
    // Role passes; the empty form then fails validation.
    assert_eq!(
        call_service(&app, put_media(&auth, &id)).await.status(),
        400
    );
    for s in ["coaches", "team"] {
        let p = create_id(&app, &auth, s).await;
        let res = call_service(&app, put_post(&auth, &p, json!({"heading": "x"}))).await;
        assert_eq!(res.status(), 202, "{s}");
        let res = call_service(&app, delete(&auth, &format!("/post/{p}"))).await;
        assert_eq!(res.status(), 202, "{s}");
    }
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
async fn normal_gets_403_on_forest_project() {
    let Some((app, _env)) = common::test_app().await else {
        return;
    };
    let admin = common::login(&app).await;
    let auth = user_with(&app, "editor", "normal").await;

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

    let forest = create_id(&app, &admin, "forest-project").await;
    let res = call_service(&app, put_post(&auth, &forest, json!({"heading": "x"}))).await;
    assert_eq!(res.status(), 403);
    let res = call_service(&app, delete(&auth, &format!("/post/{forest}"))).await;
    assert_eq!(res.status(), 403);
    let res = call_service(&app, delete(&auth, &format!("/post/{forest}/media"))).await;
    assert_eq!(res.status(), 403);
    assert_eq!(
        call_service(&app, put_media(&auth, &forest)).await.status(),
        403
    );

    // Moving a permitted post into forest-project is a write to forest-project.
    let log = create_id(&app, &auth, "logbuch").await;
    let res = call_service(
        &app,
        put_post(&auth, &log, json!({"name": "forest-project"})),
    )
    .await;
    assert_eq!(res.status(), 403);
    assert_eq!(get_post(&app, &log).await["name"], "logbuch");

    // Unknown posts stay 404, before any role check.
    let res = call_service(
        &app,
        delete(&auth, &format!("/post/{}", uuid::Uuid::new_v4())),
    )
    .await;
    assert_eq!(res.status(), 404);
}

#[actix_web::test]
async fn forest_writes_forest_project_and_shared_sections() {
    let Some((app, _env)) = common::test_app().await else {
        return;
    };
    let auth = user_with(&app, "ranger", "forest").await;
    for s in ["forest-project", "logbuch", "team", "coaches"] {
        let id = create_id(&app, &auth, s).await;
        let res = call_service(&app, put_post(&auth, &id, json!({"heading": "x"}))).await;
        assert_eq!(res.status(), 202, "{s}");
        let res = call_service(&app, delete(&auth, &format!("/post/{id}"))).await;
        assert_eq!(res.status(), 202, "{s}");
    }
    assert_eq!(
        call_service(&app, put_tournament(&auth)).await.status(),
        200
    );
}

#[actix_web::test]
async fn super_writes_everything() {
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
async fn verify_reports_role_and_derived_permissions() {
    let Some((app, _env)) = common::test_app().await else {
        return;
    };
    let admin = common::login(&app).await;
    assert_eq!(
        verify_json(&app, &admin).await,
        json!({"role": "super", "permissions": ALL, "can_set_created_at": true})
    );
    let auth = user_with(&app, "ranger", "forest").await;
    assert_eq!(
        verify_json(&app, &auth).await,
        json!({
            "role": "forest",
            "permissions": ["logbuch", "team", "coaches", "forest-project", "tournament"],
            "can_set_created_at": false,
        })
    );
    let auth = user_with(&app, "editor", "normal").await;
    assert_eq!(
        verify_json(&app, &auth).await,
        json!({
            "role": "normal",
            "permissions": ["logbuch", "team", "coaches", "tournament"],
            "can_set_created_at": false,
        })
    );
}

#[actix_web::test]
async fn register_defaults_to_normal() {
    let Some((app, _env)) = common::test_app().await else {
        return;
    };
    assert_eq!(register(&app, "plain", None).await, 201);
    let auth = login_as(&app, "plain").await;
    assert_eq!(verify_json(&app, &auth).await["role"], "normal");
}

#[actix_web::test]
async fn register_rejects_unknown_role_without_echo() {
    let Some((app, _env)) = common::test_app().await else {
        return;
    };
    let res = call_service(
        &app,
        register_req(&json!({
            "username": "bad", "password": "pw-123456", "admin_pass": "admin-pass",
            "role": "<script>root</script>",
        })),
    )
    .await;
    assert_eq!(res.status(), 400);
    assert_eq!(read_body(res).await, "Unknown role");
}

#[actix_web::test]
async fn register_rejects_legacy_fields() {
    let Some((app, _env)) = common::test_app().await else {
        return;
    };
    for extra in [json!({"admin": true}), json!({"permissions": ["logbuch"]})] {
        let mut body =
            json!({"username": "old", "password": "pw-123456", "admin_pass": "admin-pass"});
        body.as_object_mut()
            .unwrap()
            .extend(extra.as_object().unwrap().clone());
        let res = call_service(&app, register_req(&body)).await;
        assert_eq!(res.status(), 400, "{extra}");
    }
}

#[actix_web::test]
async fn role_change_applies_to_existing_token() {
    let Some((app, env)) = common::test_app().await else {
        return;
    };
    let auth = user_with(&app, "editor", "normal").await;
    assert_eq!(create(&app, &auth, "forest-project").await.status(), 403);
    sql(
        &env,
        "UPDATE users SET role = 'forest' WHERE username = 'editor'",
    )
    .await;
    assert_eq!(create(&app, &auth, "forest-project").await.status(), 200);
    sql(
        &env,
        "UPDATE users SET role = 'normal' WHERE username = 'editor'",
    )
    .await;
    assert_eq!(create(&app, &auth, "forest-project").await.status(), 403);
    assert_eq!(verify_json(&app, &auth).await["role"], "normal");
    sql(&env, "DELETE FROM users WHERE username = 'editor'").await;
    assert_eq!(create(&app, &auth, "logbuch").await.status(), 401);
}

#[actix_web::test]
async fn role_column_rejects_unknown_values() {
    let Some((_app, env)) = common::test_app().await else {
        return;
    };
    let mut conn = env.pool.get().await.unwrap();
    let conn: &mut AsyncPgConnection = &mut conn;
    assert!(
        conn.batch_execute("UPDATE users SET role = 'root' WHERE username = 'admin'")
            .await
            .is_err()
    );
}

#[actix_web::test]
async fn forbidden_uploads_write_no_file() {
    let Some((app, env)) = common::test_app().await else {
        return;
    };
    let admin = common::login(&app).await;
    let forest = create_id(&app, &admin, "forest-project").await;
    let auth = user_with(&app, "editor", "normal").await;
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
    let uri = format!("/post/{forest}/media");
    let res = call_service(&app, multipart_req(&uri, "PUT", &auth, body)).await;
    assert_eq!(res.status(), 403);
    assert_eq!(upload_files(&env), 0);
}

const W06_UP: &str = include_str!("../migrations/w06-user-roles/up.sql");
const W06_DOWN: &str = include_str!("../migrations/w06-user-roles/down.sql");

#[actix_web::test]
async fn migration_w06_down_up_roundtrip() {
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
    let cols = "string_agg(column_name, ',' ORDER BY column_name) FROM information_schema.columns \
        WHERE table_name='users' AND column_name IN ('admin','permissions','role')";
    let user = |name: &str| {
        format!("admin::text || ' ' || permissions::text FROM users WHERE username='{name}'")
    };
    let role = |name: &str| format!("role FROM users WHERE username='{name}'");
    conn.batch_execute(
        "INSERT INTO users (username,password,role) VALUES ('n','x','normal'),('f','x','forest')",
    )
    .await
    .unwrap();

    conn.batch_execute(W06_DOWN).await.unwrap();
    assert_eq!(text(conn, cols).await, "admin,permissions");
    assert_eq!(
        text(conn, &user("admin")).await,
        "true {logbuch,tournament,coaches,team,forest-project}"
    );
    assert_eq!(
        text(conn, &user("n")).await,
        "false {logbuch,tournament,coaches,team}"
    );
    assert_eq!(
        text(conn, &user("f")).await,
        "false {logbuch,tournament,coaches,team,forest-project}"
    );

    // Owner decision: every existing account becomes super.
    conn.batch_execute(W06_UP).await.unwrap();
    assert_eq!(text(conn, cols).await, "role");
    for name in ["admin", "n", "f"] {
        assert_eq!(text(conn, &role(name)).await, "super", "{name}");
    }

    conn.batch_execute("UPDATE users SET role = 'normal' WHERE username = 'n'")
        .await
        .unwrap();
    conn.batch_execute(W06_DOWN).await.unwrap();
    assert_eq!(
        text(conn, &user("n")).await,
        "false {logbuch,tournament,coaches,team}"
    );
    assert_eq!(
        text(conn, &user("f")).await,
        "true {logbuch,tournament,coaches,team,forest-project}"
    );
    conn.batch_execute(W06_UP).await.unwrap(); // leave schema as migrated
    assert_eq!(text(conn, cols).await, "role");
    // New accounts default to normal.
    conn.batch_execute("INSERT INTO users (username,password) VALUES ('new','x')")
        .await
        .unwrap();
    assert_eq!(text(conn, &role("new")).await, "normal");
}

fn post_with_date(auth: &str, section: &str, created_at: &str) -> actix_http::Request {
    let body = common::multipart(
        &[
            ("name", section),
            ("heading", "H"),
            ("content", "C"),
            ("created_at", created_at),
        ],
        None,
    );
    multipart_req("/post", "POST", auth, body)
}

#[actix_web::test]
async fn super_sets_created_at_on_post_and_put() {
    let Some((app, _env)) = common::test_app().await else {
        return;
    };
    let admin = common::login(&app).await;

    let res = call_service(
        &app,
        post_with_date(&admin, "logbuch", "2024-01-28T10:00:00+01:00"),
    )
    .await;
    assert_eq!(res.status(), 200);
    let id = read_body_json::<Value, _>(res).await["id"]
        .as_str()
        .unwrap()
        .to_owned();
    let p = get_post(&app, &id).await;
    assert_eq!(p["created_at"], "2024-01-28T09:00:00");
    assert_eq!(p["updated_at"], Value::Null);

    // Warm the list cache; the date change must invalidate it.
    let list = |app| async move {
        let req = TestRequest::get().uri("/posts/logbuch").to_request();
        read_body_json::<Value, _>(call_service(app, req).await).await
    };
    assert_eq!(list(&app).await[0]["created_at"], "2024-01-28T09:00:00");

    // Correcting the date alone is not a content edit.
    let res = call_service(
        &app,
        put_post(
            &admin,
            &id,
            json!({"created_at": "2023-06-01T12:30:00+02:00"}),
        ),
    )
    .await;
    assert_eq!(res.status(), 202);
    let p = get_post(&app, &id).await;
    assert_eq!(p["created_at"], "2023-06-01T10:30:00");
    assert_eq!(p["updated_at"], Value::Null);
    assert_eq!(list(&app).await[0]["created_at"], "2023-06-01T10:30:00");

    let res = call_service(
        &app,
        put_post(
            &admin,
            &id,
            json!({"heading": "Neu", "created_at": "2023-06-02T00:00:00Z"}),
        ),
    )
    .await;
    assert_eq!(res.status(), 202);
    let p = get_post(&app, &id).await;
    assert_eq!(p["created_at"], "2023-06-02T00:00:00");
    assert_ne!(p["updated_at"], Value::Null);
}

#[actix_web::test]
async fn non_super_gets_403_on_created_at() {
    let Some((app, _env)) = common::test_app().await else {
        return;
    };
    for (user, role) in [("ranger", "forest"), ("editor", "normal")] {
        let auth = user_with(&app, user, role).await;
        let res = call_service(
            &app,
            post_with_date(&auth, "logbuch", "2024-01-28T10:00:00+01:00"),
        )
        .await;
        assert_eq!(res.status(), 403, "{role}");

        let id = create_id(&app, &auth, "logbuch").await;
        let before = get_post(&app, &id).await;
        let res = call_service(
            &app,
            put_post(
                &auth,
                &id,
                json!({"heading": "x", "created_at": "2024-01-28T10:00:00+01:00"}),
            ),
        )
        .await;
        assert_eq!(res.status(), 403, "{role}");
        assert_eq!(get_post(&app, &id).await, before, "{role}");

        // An empty field means "now" and needs no capability.
        let res = call_service(&app, post_with_date(&auth, "logbuch", "")).await;
        assert_eq!(res.status(), 200, "{role}");
    }
}

#[actix_web::test]
async fn non_super_created_at_writes_no_file() {
    let Some((app, env)) = common::test_app().await else {
        return;
    };
    let auth = user_with(&app, "ranger", "forest").await;
    let img = common::encoded(400, 300, image::ImageFormat::Png);
    let body = common::multipart(
        &[
            ("name", "forest-project"),
            ("heading", "H"),
            ("content", "C"),
            ("created_at", "2024-01-28T10:00:00+01:00"),
            ("media_type", "img"),
            ("media_creator", "me"),
        ],
        Some(("p.png", &img)),
    );
    let res = call_service(&app, multipart_req("/post", "POST", &auth, body)).await;
    assert_eq!(res.status(), 403);
    assert_eq!(upload_files(&env), 0);
}

#[actix_web::test]
async fn invalid_or_out_of_range_created_at_is_400() {
    let Some((app, _env)) = common::test_app().await else {
        return;
    };
    let admin = common::login(&app).await;
    let id = create_id(&app, &admin, "logbuch").await;
    let before = get_post(&app, &id).await;
    let future = (chrono::Utc::now() + chrono::TimeDelta::days(2)).to_rfc3339();
    for bad in [
        "garbage",
        "2024-01-28T10:00:00",
        "2024-01-28",
        "2016-12-31T12:00:00Z",
        future.as_str(),
    ] {
        let res = call_service(&app, post_with_date(&admin, "logbuch", bad)).await;
        assert_eq!(res.status(), 400, "POST {bad}");
        assert_eq!(read_body(res).await, "Invalid created_at", "POST {bad}");
        let res = call_service(&app, put_post(&admin, &id, json!({"created_at": bad}))).await;
        assert_eq!(res.status(), 400, "PUT {bad}");
    }
    assert_eq!(get_post(&app, &id).await, before);
    // Within a day ahead and from 2017 on are fine.
    let soon = (chrono::Utc::now() + chrono::TimeDelta::hours(20)).to_rfc3339();
    for ok in ["2017-01-01T12:00:00Z", soon.as_str()] {
        let res = call_service(&app, post_with_date(&admin, "logbuch", ok)).await;
        assert_eq!(res.status(), 200, "{ok}");
    }
}
