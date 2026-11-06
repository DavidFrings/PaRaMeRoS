use crate::db::schema::tournament::dsl::tournament;
use crate::utils::{HttpError, db, verify};
use crate::{Env, query};
use actix_web::{HttpRequest, HttpResponse, delete, web::Data};
use diesel_async::RunQueryDsl;

#[delete("/tournament")]
pub async fn delete_tournament(
    req: HttpRequest,
    env: Data<Env>,
) -> Result<HttpResponse, HttpError> {
    verify(&req, &env).await?;
    let mut conn = db(&env).await?;
    query!(diesel::delete(tournament).execute(&mut conn));
    Ok(HttpResponse::NoContent().finish())
}
