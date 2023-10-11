mod api;
mod model;

use axum::{routing::get, Router};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
// use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    // let data = sqlx::query!("SELECT * FROM pg_catalog.pg_tables;").fetch_one(&pool);

    // build our application with a single route
    let app = Router::new()
        .route("/:id", get(api::score::show).post(api::score::save))
        .with_state(pool);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
