use sqlx::{Error, PgPool};

use crate::model::user::UserPublic;

pub async fn login_user(pool: &PgPool, login: &str, password: &str) -> Result<UserPublic, Error> {
    let result = sqlx::query_as!(
        UserPublic,
        r#"SELECT id, login, token FROM users WHERE login = $1 AND password = $2"#,
        login,
        password
    )
    .fetch_one(pool)
    .await;

    match result {
        Err(error) => Err(error),
        Ok(user) => Ok(user),
    }
}

pub async fn register_user(
    pool: &PgPool,
    login: &str,
    password: &str,
    token: &str,
) -> Result<UserPublic, Error> {
    let result = sqlx::query_as!(
        UserPublic,
        r#"INSERT INTO users (login, password, token) VALUES ($1, $2, $3) RETURNING id, login, token"#,
        login,
        password,
        token
    )
    .fetch_one(pool)
    .await;

    match result {
        Err(error) => Err(error),
        Ok(user) => Ok(user),
    }
}

pub async fn get_user(pool: &PgPool, login: &str) -> Result<UserPublic, Error> {
    let result = sqlx::query_as!(
        UserPublic,
        r#"SELECT id, login, token FROM users WHERE login = $1"#,
        login
    )
    .fetch_one(pool)
    .await;

    match result {
        Err(error) => Err(error),
        Ok(user) => Ok(user),
    }
}
