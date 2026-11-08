mod common;

use actix_web::test::{TestRequest, call_and_read_body};

#[actix_web::test]
async fn posts_list_empty() {
    let Some((app, _env)) = common::test_app().await else {
        return;
    };
    let res = TestRequest::get().uri("/posts/logbuch").to_request();
    let body = call_and_read_body(&app, res).await;
    assert_eq!(body, "[]");
}
