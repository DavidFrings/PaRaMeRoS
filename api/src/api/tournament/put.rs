use crate::db::{models::Tournament, schema::tournament::dsl::tournament};
use crate::utils::{HttpError, bad_request, db, verify};
use crate::{Env, query};
use actix_web::{
    HttpRequest, HttpResponse, put,
    web::{Data, Json},
};
use chrono::{DateTime, FixedOffset, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TournamentBody {
    name: String,
    place: Option<String>,
    starts_at: DateTime<FixedOffset>,
    ends_at: Option<DateTime<FixedOffset>>,
}

#[put("/tournament")]
pub async fn put_tournament(
    req: HttpRequest,
    body: Json<TournamentBody>,
    env: Data<Env>,
) -> Result<HttpResponse, HttpError> {
    verify(&req, &env).await?;

    let body = body.into_inner();
    let name = body.name.trim().to_owned();
    if name.is_empty() || name.chars().count() > 120 {
        return Err(bad_request("name must be 1-120 characters"));
    }
    let place = body
        .place
        .map(|p| p.trim().to_owned())
        .filter(|p| !p.is_empty());
    if place.as_ref().is_some_and(|p| p.chars().count() > 120) {
        return Err(bad_request("place must be at most 120 characters"));
    }
    if body.ends_at.is_some_and(|e| e <= body.starts_at) {
        return Err(bad_request("ends_at must be after starts_at"));
    }

    let row = Tournament {
        name,
        place,
        starts_at: body.starts_at.with_timezone(&Utc),
        ends_at: body.ends_at.map(|e| e.with_timezone(&Utc)),
        updated_at: Utc::now(),
    };
    let mut conn = db(&env).await?;
    let stored = query!(
        diesel::insert_into(tournament)
            .values(&row)
            .on_conflict(crate::db::schema::tournament::id)
            .do_update()
            .set(&row)
            .returning(Tournament::as_returning())
            .get_result(&mut conn)
    );
    Ok(HttpResponse::Ok().json(stored))
}
