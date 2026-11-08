use crate::Env;
use crate::db::{models::Tournament, schema::tournament::dsl::tournament};
use crate::utils::{HttpError, db, internal_error};
use actix_web::{HttpResponse, get, web::Data};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use log::error;

#[get("/tournament")]
pub async fn get_tournament(env: Data<Env>) -> Result<HttpResponse, HttpError> {
    let mut conn = db(&env).await?;
    let row = tournament
        .select(Tournament::as_select())
        .first(&mut conn)
        .await
        .optional()
        .map_err(|err| {
            error!("Database error: {err}");
            internal_error("Database error")
        })?;
    Ok(match row {
        Some(t) => HttpResponse::Ok().json(t),
        None => HttpResponse::NoContent().finish(),
    })
}
