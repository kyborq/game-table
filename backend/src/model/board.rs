use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, FromRow)]
pub struct Board {
    pub id: Uuid,
    pub game_id: Uuid,
    pub name: String,
}
