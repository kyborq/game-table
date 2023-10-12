use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::game::Game;

pub async fn get_game(pool: &PgPool, name: &str) -> Result<Game, Error> {
    sqlx::query_as!(
        Game,
        r#"
        SELECT * FROM game
        WHERE name = $1
        "#,
        name,
    )
    .fetch_one(pool)
    .await
}

pub async fn new_game(pool: &PgPool, name: &str) -> Result<Game, Error> {
    let token = Uuid::new_v4().to_string();

    sqlx::query_as!(
        Game,
        r#"
        INSERT INTO game (name, token) VALUES ($1, $2)
        RETURNING *
        "#,
        name,
        token
    )
    .fetch_one(pool)
    .await
}
