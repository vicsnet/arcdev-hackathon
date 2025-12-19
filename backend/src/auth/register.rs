use std::env;
use sqlx::Pool;
use sqlx::postgres::Postgres;
use argon2::{ password_hash::{ rand_core::OsRng, PasswordHasher, SaltString }, Argon2 };
// PasswordHash
// PasswordVerifier

use axum::{ Json, http::StatusCode, extract::State };
use serde::{ Deserialize, Serialize };
use crate::{ state::AppState, circle::create_wallet_set, circle::create_user_wallet };

#[derive(Deserialize, Serialize)]
pub struct RegisterRequest {
    email: String,
    password: String,
}

async fn get_or_create_admin_wallet(state: &AppState) -> Result<String, StatusCode> {
    let existing_wallet_id = sqlx
        ::query!("SELECT wallet_set_id FROM system_wallets WHERE name = $1", "ADMIN")
        .fetch_optional(&state.db).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(row) = existing_wallet_id {
        return Ok(row.wallet_set_id);
    }

    let wallet_set_id = create_wallet_set(
        &state.circle,
        &state.entity_secret_ciphertext
    ).await.map_err(|_| StatusCode::BAD_GATEWAY)?;

    sqlx
        ::query!(
            "INSERT INTO system_wallets (name, wallet_set_id) VALUES ($1, $2)",
            "ADMIN",
            wallet_set_id
        )
        .execute(&state.db).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(wallet_set_id)
}
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>
) -> Result<StatusCode, StatusCode> {
    // let database_url = env::var("DATABASE_URL").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // let pool = Pool::<Postgres>::connect(&database_url).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let admin_wallet_set_id = get_or_create_admin_wallet(&state).await?;

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();

    let user_record = sqlx
        ::query!(
            "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING id",
            payload.email,
            password_hash
        )
        .fetch_one(&state.db).await
        .map_err(|_| StatusCode::CONFLICT)?;

    let user_wallet = create_user_wallet(
        &state.circle,
        &state.entity_secret_ciphertext,
        &admin_wallet_set_id
    ).await.map_err(|_| StatusCode::BAD_GATEWAY)?;

    // 5. Insert wallet using user_id
    let user_id = user_record.id;

    sqlx
        ::query!(
            "INSERT INTO user_wallets 
            (user_id, wallet_id, wallet_set_id, state, blockchain, custody_type, account_type) 
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
            user_id,
            user_wallet.id,
            user_wallet.WalletSetId,
            user_wallet.state,
            user_wallet.blockchain,
            user_wallet.custodyType,
            user_wallet.accountType
        )
        .execute(&state.db).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}
