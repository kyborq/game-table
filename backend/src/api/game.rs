use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use sqlx::PgPool;

use crate::repository::game::get_game;

#[derive(Debug, Deserialize)]
pub struct NewGameForm {
    pub name: String,
}

pub async fn create(
    pool: State<PgPool>,
    form: Json<NewGameForm>,
) -> Result<impl IntoResponse, StatusCode> {
    let game = get_game(&pool, &form.name).await;
    // .unwrap_or(StatusCode::NOT_FOUND);

    // match get_game(&pool, &form.name).await {
    //     Ok(game) => Ok(game),
    //     Err(_) => Err(Ok(StatusCode::NOT_ACCEPTABLE)),
    // };

    // match existed_game {
    //     Ok(_) => (StatusCode::FORBIDDEN, Response::Error("Fuck!".to_string())),
    //     Err(_) => match new_game(&pool, &form.name).await {
    //         Ok(game) => (StatusCode::CREATED, Response::Body(game)),
    //         Err(_) => (
    //             StatusCode::SERVICE_UNAVAILABLE,
    //             Response::Error("Fuck!".to_string()),
    //         ),
    //     },
    // }

    Ok(StatusCode::NOT_FOUND)
}
