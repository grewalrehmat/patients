use actix_web::{post, web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::{
    models::{NewUser, User},
    schema::users::dsl::{users, username as uname_col},
    utils::{hash::hash_password, auth::{verify_password, generate_token}},
    DbPool,
};
use serde::Deserialize;
use serde_json::json;

#[post("/register")]
async fn register_user(
    pool: web::Data<DbPool>,
    new_user: web::Json<NewUser>,
) -> impl Responder {
    let conn = &mut pool.get().expect("DB pool fail");

    let hashed = match hash_password(&new_user.password_hash) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().body("Password hash error"),
    };

    let user_data = NewUser {
        username: new_user.username.clone(),
        password_hash: hashed,
        role: new_user.role.clone(),
    };

    let result = diesel::insert_into(users)
        .values(&user_data)
        .execute(conn);

    match result {
        Ok(_) => HttpResponse::Ok().body("User registered"),
        Err(_) => HttpResponse::BadRequest().body("Registration failed"),
    }
}

#[derive(Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[post("/login")]
async fn login_user(
    pool: web::Data<DbPool>,
    creds: web::Json<LoginData>,
) -> impl Responder {
    let conn = &mut pool.get().expect("DB pool fail");

    let user = users
        .filter(uname_col.eq(&creds.username))
        .first::<User>(conn);

    match user {
        Ok(u) =>{
            if verify_password(&u.password_hash, &creds.password) {
            let token = create_jwt(&u)?;
            Ok(HttpResponse::Ok().json(json!({ "token": token })))
            } else {
                Ok(HttpResponse::Unauthorized().body("Invalid credentials"))
            }
        }
        Err(_) => HttpResponse::Unauthorized().body("Invalid credentials"),
    }
}