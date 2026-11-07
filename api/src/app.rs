use crate::api::post::{
    delete_post::delete_post,
    get_post::post,
    media::{delete_media, replace_media},
    new_post::new_post,
    posts::posts,
    update_post::update_post,
};
use crate::api::sitemap::sitemap;
use crate::api::tournament::{delete::delete_tournament, get::get_tournament, put::put_tournament};
use crate::api::users::{auth::auth, register::register, verify::auth_verify};
use actix_cors::Cors;
use actix_web::{
    http::header::{AUTHORIZATION, CONTENT_TYPE, RETRY_AFTER},
    web::ServiceConfig,
};

/// Register every service. Each new endpoint is added here.
pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(register)
        .service(auth)
        .service(auth_verify)
        .service(post)
        .service(posts)
        .service(new_post)
        .service(update_post)
        .service(delete_post)
        .service(replace_media)
        .service(delete_media)
        .service(get_tournament)
        .service(put_tournament)
        .service(delete_tournament)
        .service(sitemap);
}

pub fn cors(origins: &[String]) -> Cors {
    let mut cors = Cors::default();

    for origin in origins {
        cors = cors.allowed_origin(origin);
    }

    cors.allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![CONTENT_TYPE, AUTHORIZATION])
        .expose_headers(vec![AUTHORIZATION, RETRY_AFTER])
        .supports_credentials()
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, HttpResponse, test, web};

    #[actix_web::test]
    async fn cors_exposes_authorization_and_retry_after() {
        let app = test::init_service(
            App::new()
                .wrap(cors(&["http://a.test".into()]))
                .route("/", web::get().to(HttpResponse::Ok)),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/")
            .insert_header(("Origin", "http://a.test"))
            .to_request();
        let res = test::call_service(&app, req).await;
        let exposed = res
            .headers()
            .get("access-control-expose-headers")
            .map(|v| v.to_str().unwrap().to_lowercase())
            .unwrap_or_default();
        assert!(exposed.contains("authorization"), "{exposed}");
        assert!(exposed.contains("retry-after"), "{exposed}");
    }
}
