use actix_web::{post, web::{Data, Json}, HttpResponse};
use bcrypt::{hash, DEFAULT_COST};
use diesel_async::RunQueryDsl;
use diesel::SelectableHelper;
use crate::db::{models::{NewUser, User}, schema::users::dsl::users};
use crate::Env;
use crate::utils::{internal_error, HttpError};

#[post("/register")]
pub async fn register(new_usr: Json<NewUser>, env: Data<Env>) -> Result<HttpResponse, HttpError> {
    let mut usr = new_usr.into_inner();
    
    let hashed = hash(&usr.password, DEFAULT_COST)
        .map_err(|err| 
            internal_error(format!("Password hashing failed: {}", err))
        )?;
    usr.password = hashed;

    let mut conn = env.pool.get()
        .await
        .map_err(|err| 
            internal_error(format!("Database connection error: {}", err))
        )?;

    let res = diesel::insert_into(users)
        .values(&usr)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(|err|
            internal_error(format!("Error creating user: {}", err))
        )?;
    
    Ok(HttpResponse::Created().json(res))
}