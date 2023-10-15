use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::game::Game;

pub async fn create_game(pool: &PgPool, name: &str) -> Result<Game, Error> {
    let id = Uuid::new_v4();

    let result = sqlx::query_as!(
        Game,
        r#"INSERT INTO games (id, name) VALUES ($1, $2) RETURNING *"#,
        id,
        name,
    )
    .fetch_one(pool)
    .await;

    match result {
        Err(error) => Err(error),
        Ok(game) => Ok(game),
    }
}

pub async fn get_game(pool: &PgPool, name: &str) -> Result<Game, Error> {
    let result = sqlx::query_as!(Game, r#"SELECT * FROM games WHERE name = $1"#, name)
        .fetch_one(pool)
        .await;

    match result {
        Err(error) => Err(error),
        Ok(game) => Ok(game),
    }
}
