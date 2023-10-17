use std::env;

use axum::{extract::State, Json};
use chrono::{Duration, Utc};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    model::user::UserPublic,
    repository::user::{get_user, login_user, register_user},
    response::CustomResponse,
    service::token::generate_token,
};

#[derive(Debug, Deserialize)]
pub struct LoginCredentials {
    login: String,
    password: String,
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(credentials): Json<LoginCredentials>,
) -> CustomResponse<UserPublic> {
    let user = login_user(&pool, &credentials.login, &credentials.password).await;

    match user {
        Ok(user) => {
            let secret = env::var("TOKEN_SECRET").unwrap();
            let expiration_date = Utc::now() + Duration::days(7);
            let token = generate_token(&user.id.to_string(), &secret, expiration_date);

            let cookie_token = format!(
                "token={}; HttpOnly; Expires={}",
                token,
                expiration_date.to_rfc2822()
            );

            CustomResponse::WithCookie(user, cookie_token.to_string())
        }
        Err(_) => CustomResponse::Error("Login or password are incorrect".to_string()),
    }
}

#[derive(Debug, Deserialize)]
pub struct RegisterCredentials {
    login: String,
    password: String,
    email: String,
    repeat_password: String,
}

pub async fn register(
    State(pool): State<PgPool>,
    Json(register): Json<RegisterCredentials>,
) -> CustomResponse<UserPublic> {
    let user = get_user(&pool, &register.login).await;
    if user.is_ok() {
        return CustomResponse::Error("This user already exist".to_string());
    }

    if register.password != register.repeat_password {
        return CustomResponse::Error("Passwords are not equal".to_string());
    }

    let user = register_user(
        &pool,
        &register.login,
        &register.password,
        "&register.token",
    )
    .await;

    match user {
        Ok(user) => CustomResponse::Success(user),
        Err(_) => CustomResponse::Error("Login or password are incorrect".to_string()),
    }
}

pub async fn logout() {
    // ...
}
