mod common;

use actix_web::{
    http::header::AUTHORIZATION,
    test::{TestRequest, call_service, read_body},
};
use diesel_async::{AsyncPgConnection, SimpleAsyncConnection};

async fn sitemap<S, B>(app: &S) -> String
where
    S: actix_web::dev::Service<
            actix_http::Request,
            Response = actix_web::dev::ServiceResponse<B>,
            Error = actix_web::Error,
        >,
    B: actix_web::body::MessageBody,
{
    let res = call_service(app, TestRequest::get().uri("/sitemap.xml").to_request()).await;
    assert_eq!(res.status(), 200);
    assert_eq!(
        res.headers().get("content-type").unwrap(),
        "application/xml; charset=utf-8"
    );
    String::from_utf8(read_body(res).await.to_vec()).unwrap()
}

#[actix_web::test]
async fn sitemap_lists_fixed_pages_in_both_languages() {
    let Some((app, _env)) = common::test_app().await else {
        return;
    };
    let xml = sitemap(&app).await;
    assert!(xml.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
    for p in [
        "",
        "/About",
        "/team",
        "/logbuch",
        "/prmrs/forest-project",
        "/Imprint",
        "/Privacy-Policy",
    ] {
        assert!(xml.contains(&format!("<loc>https://example.test{p}</loc>")) || p.is_empty());
        let en = if p.is_empty() { "/" } else { p };
        assert!(
            xml.contains(&format!("<loc>https://example.test/en{en}</loc>")),
            "{p}"
        );
    }
    assert!(xml.contains("<loc>https://example.test/</loc>"));
    assert!(!xml.contains("example.test//"));
}

#[actix_web::test]
async fn sitemap_includes_post_with_lastmod() {
    let Some((app, env)) = common::test_app().await else {
        return;
    };
    {
        let mut conn = env.pool.get().await.unwrap();
        let conn: &mut AsyncPgConnection = &mut conn;
        conn.batch_execute(
            "INSERT INTO posts (uuid, name, heading, content, created_at, updated_at) VALUES \
             ('11111111-1111-1111-1111-111111111111','team','H','C','2024-01-02 03:04:05','2024-05-06 07:08:09'), \
             ('22222222-2222-2222-2222-222222222222','forest-project','H','C','2025-01-02 03:04:05',NULL)",
        )
        .await
        .unwrap();
    }
    let xml = sitemap(&app).await;
    assert!(xml.contains("<loc>https://example.test/post/11111111-1111-1111-1111-111111111111</loc>\n    <lastmod>2024-05-06</lastmod>"));
    assert!(xml.contains("<loc>https://example.test/en/post/22222222-2222-2222-2222-222222222222</loc>\n    <lastmod>2025-01-02</lastmod>"));
    // newest first
    assert!(xml.find("2222222").unwrap() < xml.find("1111111").unwrap());
}

#[actix_web::test]
async fn sitemap_escapes_xml() {
    let Some((app, _env)) = common::test_app_with_site("https://example.test/?a=1&b=<2>/").await
    else {
        return;
    };
    let xml = sitemap(&app).await;
    assert!(xml.contains("<loc>https://example.test/?a=1&amp;b=&lt;2&gt;/About</loc>"));
    assert!(!xml.contains("&b="));
}

#[actix_web::test]
async fn sitemap_has_hreflang_alternates() {
    let Some((app, _env)) = common::test_app().await else {
        return;
    };
    let xml = sitemap(&app).await;
    assert!(xml.contains(
        "<xhtml:link rel=\"alternate\" hreflang=\"de\" href=\"https://example.test/About\"/>"
    ));
    assert!(xml.contains(
        "<xhtml:link rel=\"alternate\" hreflang=\"en\" href=\"https://example.test/en/About\"/>"
    ));
    assert!(xml.contains(
        "<xhtml:link rel=\"alternate\" hreflang=\"x-default\" href=\"https://example.test/About\"/>"
    ));
    // each url block has exactly 3 alternates
    assert_eq!(
        xml.matches("<url>").count() * 3,
        xml.matches("<xhtml:link").count()
    );
}

#[actix_web::test]
async fn sitemap_cache_invalidated_on_post_create() {
    if std::env::var("TEST_REDIS_URL").is_err() {
        return;
    }
    let Some((app, _env)) = common::test_app().await else {
        return;
    };
    let auth = common::login(&app).await;
    let before = sitemap(&app).await;
    assert!(!before.contains("/post/"));

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
    assert!(
        sitemap(&app)
            .await
            .contains("<loc>https://example.test/post/")
    );
}
