mod common;

use actix_web::{
    http::header::AUTHORIZATION,
    test::{TestRequest, call_service, read_body_json},
};
use chrono::DateTime;
use serde_json::{Value, json};

fn put(auth: Option<&str>, body: Value) -> actix_http::Request {
    let mut r = TestRequest::put().uri("/tournament").set_json(body);
    if let Some(a) = auth {
        r = r.insert_header((AUTHORIZATION, a));
    }
    r.to_request()
}

fn body() -> Value {
    json!({"name": " Cup ", "place": "", "starts_at": "2027-03-28T10:00:00+02:00", "ends_at": "2027-03-29T10:00:00+02:00"})
}

#[actix_web::test]
async fn tournament_empty_returns_204() {
    let Some((app, _)) = common::test_app().await else {
        return;
    };
    let res = call_service(&app, TestRequest::get().uri("/tournament").to_request()).await;
    assert_eq!(res.status(), 204);
}

#[actix_web::test]
async fn put_requires_auth() {
    let Some((app, _)) = common::test_app().await else {
        return;
    };
    assert_eq!(
        call_service(&app, put(Some("Bearer bad"), body()))
            .await
            .status(),
        401
    );
}

#[actix_web::test]
async fn put_then_get() {
    let Some((app, _)) = common::test_app().await else {
        return;
    };
    let auth = common::login(&app).await;
    let res = call_service(&app, put(Some(&auth), body())).await;
    assert_eq!(res.status(), 200);
    let res = call_service(&app, TestRequest::get().uri("/tournament").to_request()).await;
    assert_eq!(res.status(), 200);
    let v: Value = read_body_json(res).await;
    assert_eq!(v["name"], "Cup");
    assert!(v["place"].is_null());
    assert!(v["updated_at"].is_string());
}

#[actix_web::test]
async fn tournament_roundtrips_offset_instant() {
    let Some((app, _)) = common::test_app().await else {
        return;
    };
    let auth = common::login(&app).await;
    call_service(&app, put(Some(&auth), body())).await;
    let res = call_service(&app, TestRequest::get().uri("/tournament").to_request()).await;
    let v: Value = read_body_json(res).await;
    let got = DateTime::parse_from_rfc3339(v["starts_at"].as_str().unwrap()).unwrap();
    let want = DateTime::parse_from_rfc3339("2027-03-28T10:00:00+02:00").unwrap();
    assert_eq!(got, want);
}

#[actix_web::test]
async fn end_before_start_rejected() {
    let Some((app, _)) = common::test_app().await else {
        return;
    };
    let auth = common::login(&app).await;
    let mut b = body();
    b["ends_at"] = json!("2027-03-28T10:00:00+02:00");
    assert_eq!(call_service(&app, put(Some(&auth), b)).await.status(), 400);
    let mut b = body();
    b["name"] = json!("  ");
    assert_eq!(call_service(&app, put(Some(&auth), b)).await.status(), 400);
}

#[actix_web::test]
async fn delete_clears() {
    let Some((app, _)) = common::test_app().await else {
        return;
    };
    let auth = common::login(&app).await;
    call_service(&app, put(Some(&auth), body())).await;
    let del = || {
        TestRequest::delete()
            .uri("/tournament")
            .insert_header((AUTHORIZATION, auth.clone()))
            .to_request()
    };
    assert_eq!(call_service(&app, del()).await.status(), 204);
    assert_eq!(call_service(&app, del()).await.status(), 204);
    let res = call_service(&app, TestRequest::get().uri("/tournament").to_request()).await;
    assert_eq!(res.status(), 204);
}

#[actix_web::test]
async fn second_put_clears_place_and_end() {
    let Some((app, _)) = common::test_app().await else {
        return;
    };
    let auth = common::login(&app).await;
    let mut b = body();
    b["place"] = json!("Berlin");
    assert_eq!(call_service(&app, put(Some(&auth), b)).await.status(), 200);
    for second in [
        json!({"name": "Cup", "starts_at": "2027-03-28T10:00:00+02:00"}),
        json!({"name": "Cup", "place": "", "starts_at": "2027-03-28T10:00:00+02:00"}),
    ] {
        // restore values so each variant starts from a filled row
        let mut b = body();
        b["place"] = json!("Berlin");
        call_service(&app, put(Some(&auth), b)).await;
        assert_eq!(
            call_service(&app, put(Some(&auth), second)).await.status(),
            200
        );
        let res = call_service(&app, TestRequest::get().uri("/tournament").to_request()).await;
        let v: Value = read_body_json(res).await;
        assert!(v["place"].is_null(), "{v}");
        assert!(v["ends_at"].is_null(), "{v}");
    }
}

#[actix_web::test]
async fn long_name_rejected() {
    let Some((app, _)) = common::test_app().await else {
        return;
    };
    let auth = common::login(&app).await;
    let mut b = body();
    b["name"] = json!("x".repeat(121));
    assert_eq!(call_service(&app, put(Some(&auth), b)).await.status(), 400);
}
