use axum::{ Router, routing::get, routing::post };
use tokio::net::TcpListener;
use dotenvy::dotenv;
use std::env;
use sqlx::Pool;
use sqlx::postgres::Postgres;

mod auth;
mod circle;
mod state;

use auth::{ register };
use state::AppState;
use circle::circle_client::CircleClient;
#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let pool = Pool::<Postgres>
        ::connect(&database_url).await
        .expect(" Failed to connect to database");
    let entity_secret_ciphertext = std::env
        ::var("CIRCLE_ENTITY_SECRET_CIPHERTEXT")
        .expect("CIRCLE_ENTITY_SECRET_CIPHERTEXT not set");
    let circle = CircleClient::new();

    let state = AppState { db: pool, circle, entity_secret_ciphertext };

    let app = Router::new().route("/register", post(register)).with_state(state);
    // .route("/login", post(login));

    let listener = TcpListener::bind("0.0.0:8000").await.unwrap();
    println!("Listening on http://127.0.0.1:8000");
    axum::serve(listener, app).await.unwrap();
}
