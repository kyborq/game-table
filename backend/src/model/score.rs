use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, FromRow)]
pub struct Score {
    pub id: Uuid,
    pub game_id: Uuid,
    pub total: i32,
    pub player: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct ScoreDisplay {
    pub total: Option<i32>,
    pub player: String,
}
