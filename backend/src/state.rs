use sqlx::{Pool, Postgres};
use crate::circle::circle_client::CircleClient;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
    pub circle: CircleClient,
    pub entity_secret_ciphertext: String,
}