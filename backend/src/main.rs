mod api;
mod middlewares;
mod model;
mod repository;
mod response;
mod service;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use middlewares::auth::auth_middleware;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let auth_routes = Router::new()
        .route("/login", post(api::auth::login))
        .route("/register", post(api::auth::register))
        .route("/logout", post(api::auth::logout));

    // Opened to public integrations
    // But with rate limiting for certain game
    let score_routes = Router::new()
        .route("/score", post(api::score::create))
        .route("/score/:game", get(api::score::list))
        .route("/score/:game/:player", get(api::score::show));

    // Only for authorized users
    let game_routes = Router::new()
        .route("/games", post(api::game::create))
        .route("/games/:name", get(api::game::info))
        .layer(middleware::from_fn(auth_middleware));

    let api_routes = Router::new()
        .merge(auth_routes)
        .merge(score_routes)
        .merge(game_routes);

    let app = Router::new().nest("/api", api_routes).with_state(pool);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
