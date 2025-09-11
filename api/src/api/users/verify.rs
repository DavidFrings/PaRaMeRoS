use crate::db::{
    models::{AuthVerifyResponse, User},
    schema::users::{dsl::users, id},
};
use crate::utils::{HttpError, db, internal_error, verify};
use crate::{Env, query};
use actix_web::{HttpRequest, HttpResponse, get, web::Data};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

#[get("/auth/verify")]
async fn auth_verify(req: HttpRequest, env: Data<Env>) -> Result<HttpResponse, HttpError> {
    let usr_id = verify(&req, &env).await?;

    let mut conn = db(&env).await?;

    let usr = query!(
        users
            .filter(id.eq(&usr_id))
            .limit(1)
            .get_result::<User>(&mut conn)
    );

    let json = AuthVerifyResponse { admin: usr.admin };

    Ok(HttpResponse::Ok().json(json))
}
