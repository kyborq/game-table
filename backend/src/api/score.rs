use axum::extract::{Path, State};
use sqlx::PgPool;

pub async fn save(State(pool): State<PgPool>, Path(table_id): Path<u32>) -> &'static str {
    "Put that score in the table!"
}

pub async fn show(State(pool): State<PgPool>, Path(table_id): Path<u32>) -> String {
    format!("Show me score from the table {}!", table_id).to_string()
}
