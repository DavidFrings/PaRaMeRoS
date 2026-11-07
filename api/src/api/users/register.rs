use crate::client_ip::client_ip;
use crate::db::{
    models::{NewUser, RegisterRequest, User},
    schema::users::dsl::users,
};
use crate::redis_store::hit_limit;
use crate::utils::{HttpError, db, forbidden, internal_error, too_many_requests};
use crate::{Env, query};
use actix_web::{
    HttpRequest, HttpResponse, post,
    web::{Data, Json},
};
use bcrypt::{DEFAULT_COST, hash};
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;

#[post("/register")]
pub async fn register(
    http_req: HttpRequest,
    request: Json<RegisterRequest>,
    env: Data<Env>,
) -> Result<HttpResponse, HttpError> {
    if let Some(retry) = hit_limit(
        &env,
        &format!("rl:register:{}", client_ip(&http_req)),
        3,
        3600,
    )
    .await
    {
        return Err(too_many_requests(retry));
    }

    let req = request.into_inner();

    if req.admin_pass != env.admin_pass {
        return Err(forbidden("You are not allowed to create a new user!"));
    }

    let hashed = hash(&req.password, DEFAULT_COST)
        .map_err(|err| internal_error(format!("Password hashing failed: {}", err)))?;

    let new_user = NewUser {
        username: req.username,
        password: hashed,
        admin: req.admin,
    };

    let mut conn = db(&env).await?;

    query!(
        diesel::insert_into(users)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(&mut conn)
    );

    Ok(HttpResponse::Created().finish())
}
