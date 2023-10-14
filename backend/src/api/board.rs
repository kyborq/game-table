use axum::{extract::State, Json};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct NewBoard {
    game_id: String,
    name: String,
}

pub async fn create(State(pool): State<PgPool>, Json(new_board): Json<NewBoard>) {

    // ...
}

pub async fn show() {
    // ...
}

pub async fn list() {
    // ...
}
