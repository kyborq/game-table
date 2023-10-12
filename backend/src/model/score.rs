use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, FromRow)]
pub struct Score {
    pub id: Uuid,
    pub total: i32,
    pub board_id: Uuid,
    pub player: String,
}
