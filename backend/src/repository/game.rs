use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::game::Game;
use crate::utils::generate_game_token;

pub async fn create_game(pool: &PgPool, name: &str) -> Result<Game, Error> {
    let id = Uuid::new_v4();
    let token = generate_game_token();

    let result = sqlx::query_as!(
        Game,
        r#"INSERT INTO game (id, name, token) VALUES ($1, $2, $3) RETURNING *"#,
        id,
        name,
        token
    )
    .fetch_one(pool)
    .await;

    match result {
        Err(error) => Err(error),
        Ok(game) => Ok(game),
    }
}

pub async fn get_game(pool: &PgPool, name: &str) -> Result<Game, Error> {
    let result = sqlx::query_as!(Game, r#"SELECT * FROM game WHERE name = $1"#, name)
        .fetch_one(pool)
        .await;

    match result {
        Err(error) => Err(error),
        Ok(game) => Ok(game),
    }
}
