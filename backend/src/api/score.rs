use axum::{
    extract::{Path, State},
    Json,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    model::score::{Score, ScoreDisplay},
    repository::score::{create_score, get_player_score, get_score_list},
    response::CustomResponse,
};

#[derive(Debug, Deserialize)]
pub struct NewScore {
    game_id: String,
    player: String,
    score: i32,
}

pub async fn create(
    State(pool): State<PgPool>,
    Json(new_score): Json<NewScore>,
) -> CustomResponse<Score> {
    let result = create_score(
        &pool,
        &new_score.score,
        &new_score.player,
        &new_score.game_id,
    )
    .await;

    match result {
        Ok(score) => CustomResponse::Success(score),
        Err(_) => CustomResponse::Error("Something is wrong".to_string()),
    }
}

pub async fn list(
    State(pool): State<PgPool>,
    Path(game): Path<String>,
) -> CustomResponse<Vec<ScoreDisplay>> {
    let result = get_score_list(&pool, &game).await;

    match result {
        Ok(score) => CustomResponse::Success(score),
        Err(_) => CustomResponse::Error("Something is wrong".to_string()),
    }
}

pub async fn show(
    State(pool): State<PgPool>,
    Path((game, player)): Path<(String, String)>,
) -> CustomResponse<Vec<ScoreDisplay>> {
    let result = get_player_score(&pool, &game, &player).await;

    match result {
        Ok(score) => CustomResponse::Success(score),
        Err(_) => CustomResponse::Error("Something is wrong".to_string()),
    }
}
