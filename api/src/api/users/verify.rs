use crate::Env;
use crate::db::models::AuthVerifyResponse;
use crate::utils::{HttpError, PERMISSIONS, verify};
use actix_web::{HttpRequest, HttpResponse, get, web::Data};

#[get("/auth/verify")]
async fn auth_verify(req: HttpRequest, env: Data<Env>) -> Result<HttpResponse, HttpError> {
    let usr = verify(&req, &env).await?;

    // Admins get every permission so clients need only one rule.
    let permissions = if usr.admin {
        PERMISSIONS.map(String::from).to_vec()
    } else {
        usr.permissions
    };

    Ok(HttpResponse::Ok().json(AuthVerifyResponse {
        admin: usr.admin,
        permissions,
    }))
}
