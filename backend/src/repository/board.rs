use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::board::Board;

pub async fn create_board(pool: &PgPool, name: &String, game_id: &String) -> Result<Board, Error> {
    let id = Uuid::new_v4();
    let game_id = Uuid::try_parse(&game_id).unwrap();

    let result = sqlx::query_as!(
        Board,
        r#"INSERT INTO board (id, game_id, name) VALUES ($1, $2, $3) RETURNING *"#,
        id,
        game_id,
        name
    )
    .fetch_one(pool)
    .await;

    match result {
        Err(error) => Err(error),
        Ok(board) => Ok(board),
    }
}
