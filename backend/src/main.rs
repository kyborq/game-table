mod api;
mod model;
mod repository;
mod response;
mod service;

use axum::{
    http::Request,
    middleware::{self, Next},
    response::Response,
    routing::{get, post, Route},
    Router,
};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

async fn my_middleware<B>(request: Request<B>, next: Next<B>) -> Response {
    // do something with `request`...

    println!("hello from my_middleware");

    let response = next.run(request).await;

    // do something with `response`...

    response
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let auth_routes = Router::new()
        .route("/login", post(api::auth::login))
        .route("/register", post(api::auth::register))
        .route("/logout", post(api::auth::logout));

    let score_routes = Router::new()
        .route("/score", post(api::score::create))
        .route("/score/:game", get(api::score::list))
        .route("/score/:game/:player", get(api::score::show));

    let game_routes = Router::new()
        .route("/games", post(api::game::create))
        .route("/games/:name", get(api::game::info))
        .layer(middleware::from_fn(my_middleware));

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
