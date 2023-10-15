mod api;
mod model;
mod repository;
mod response;

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

    let api_routes = Router::new()
        .route("/login", post(api::auth::login))
        .route("/register", post(api::auth::register))
        .route("/logout", post(api::auth::logout))
        .route("/games", post(api::game::create))
        .route("/games/:name", get(api::game::info))
        .route("/score", post(api::score::create))
        .route("/score/:game", get(api::score::list))
        .route("/score/:game/:player", get(api::score::show));

    let app = Router::new().nest("/api", api_routes).with_state(pool);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
