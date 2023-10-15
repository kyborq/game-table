use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::score::{Score, ScoreDisplay};

pub async fn create_score(
    pool: &PgPool,
    score_count: &i32,
    player_name: &String,
    game_id: &String,
) -> Result<Score, Error> {
    let id = Uuid::new_v4();
    let game_id = Uuid::try_parse(&game_id).unwrap();

    let result = sqlx::query_as!(
        Score,
        r#"INSERT INTO score (id, game_id, total, player) VALUES ($1, $2, $3, $4) RETURNING *"#,
        id,
        game_id,
        score_count,
        player_name,
    )
    .fetch_one(pool)
    .await;

    match result {
        Err(error) => Err(error),
        Ok(score) => Ok(score),
    }
}

pub async fn get_score_list(pool: &PgPool, game_name: &String) -> Result<Vec<ScoreDisplay>, Error> {
    let result = sqlx::query_as!(
        ScoreDisplay,
        r#"
            SELECT max(s.total) AS total, s.player 
            FROM score s
            INNER JOIN games g on g.id = s.game_id 
            WHERE g.name = $1
            GROUP BY s.player
        "#,
        game_name
    )
    .fetch_all(pool)
    .await;

    match result {
        Err(error) => Err(error),
        Ok(score_list) => Ok(score_list),
    }
}

pub async fn get_player_score(
    pool: &PgPool,
    game_name: &String,
    player_name: &String,
) -> Result<Vec<ScoreDisplay>, Error> {
    let result = sqlx::query_as!(
        ScoreDisplay,
        r#"
            SELECT s.total, s.player 
            FROM score s
            INNER JOIN games g on g.id = s.game_id 
            WHERE g.name = $1
            AND s.player = $2
        "#,
        game_name,
        player_name
    )
    .fetch_all(pool)
    .await;

    match result {
        Err(error) => Err(error),
        Ok(score_list) => Ok(score_list),
    }
}
