use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub login: String,
    pub password: String,
    pub token: String,
}
