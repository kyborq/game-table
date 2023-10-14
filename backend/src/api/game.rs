use axum::{
    extract::{Path, State},
    Json,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    model::game::Game,
    repository::game::{create_game, get_game},
    response::CustomResponse,
};

#[derive(Debug, Deserialize)]
pub struct NewGame {
    name: String,
}

pub async fn create(
    State(pool): State<PgPool>,
    Json(new_game): Json<NewGame>,
) -> CustomResponse<Game> {
    let result = create_game(&pool, &new_game.name).await;

    match result {
        Ok(game) => CustomResponse::Success(game),
        Err(_) => CustomResponse::Error("Something is wrong".to_string()),
    }
}

pub async fn info(State(pool): State<PgPool>, Path(name): Path<String>) -> CustomResponse<Game> {
    let result = get_game(&pool, &name).await;

    match result {
        Ok(game) => CustomResponse::Success(game),
        Err(_) => CustomResponse::Error("Something is wrong".to_string()),
    }
}
