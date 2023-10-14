mod api;
mod model;
mod repository;
mod response;
mod utils;

use axum::{
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let app = Router::new()
        .route("/games", post(api::game::create))
        .route("/game/:name", get(api::game::info))
        .with_state(pool);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
