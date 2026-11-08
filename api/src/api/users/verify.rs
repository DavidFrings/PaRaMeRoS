use crate::Env;
use crate::db::models::AuthVerifyResponse;
use crate::utils::{HttpError, PERMISSIONS, can_set_created_at, role_allows, verify};
use actix_web::{HttpRequest, HttpResponse, get, web::Data};

#[get("/auth/verify")]
async fn auth_verify(req: HttpRequest, env: Data<Env>) -> Result<HttpResponse, HttpError> {
    let usr = verify(&req, &env).await?;

    Ok(HttpResponse::Ok().json(AuthVerifyResponse {
        permissions: PERMISSIONS
            .into_iter()
            .filter(|p| role_allows(&usr.role, p))
            .collect(),
        can_set_created_at: can_set_created_at(&usr),
        role: usr.role,
    }))
}
