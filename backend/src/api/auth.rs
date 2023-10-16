use axum::{extract::State, Json};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    model::user::UserPublic,
    repository::user::{get_user, login_user, register_user},
    response::CustomResponse,
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
            // Generate new token

            // Save to cookie

            // CustomResponse::Success(user)

            // TODO: Add token generation
            CustomResponse::WithCookie(user, "asdf".to_string())
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
