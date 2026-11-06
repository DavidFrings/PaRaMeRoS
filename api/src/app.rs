use crate::api::post::{
    delete_post::delete_post,
    get_post::post,
    media::{delete_media, replace_media},
    new_post::new_post,
    posts::posts,
    update_post::update_post,
};
use crate::api::tournament::{delete::delete_tournament, get::get_tournament, put::put_tournament};
use crate::api::users::{auth::auth, register::register, verify::auth_verify};
use actix_cors::Cors;
use actix_web::{
    http::header::{AUTHORIZATION, CONTENT_TYPE},
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
        .service(delete_tournament);
}

pub fn cors(origins: &[String]) -> Cors {
    let mut cors = Cors::default();

    for origin in origins {
        cors = cors.allowed_origin(origin);
    }

    cors.allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![CONTENT_TYPE, AUTHORIZATION])
        .supports_credentials()
}
