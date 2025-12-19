use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use reqwest::header::{CONTENT_TYPE, ACCEPT, AUTHORIZATION};


#[derive(Debug, Clone)]
pub struct CircleClient {
    pub http: Client,
    pub api_key: String,
    pub base_url: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWalletSetRequest{
    idempotencyKey: String,
    name: String,
    entitySecretCiphertext:String,
}

#[derive(Debug, Serialize)]
pub struct CreateUserWalletSetRequest{
#[serde(rename = "idempotencyKey")]
pub idempotency_key: String,
 #[serde(rename = "accountType")]
pub account_type: String,

pub blockchains: Vec<String>,
pub count: u32,
#[serde(rename = "entitySecretCiphertext")]
pub entity_secret_ciphertext: String,
#[serde(rename = "walletSetId")]
pub wallet_set_id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWalletSetResponse{
    pub data: WalletSetResponseData
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WalletSetResponseData{
    pub id: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UsersWalletResponseData{
   pub id: String,
   pub state: String,
   pub WalletSetId: String,
   pub custodyType: String,
   pub blockchain: String,
   pub accountType: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUsersWalletResponseData{
   pub data:UsersWalletResponseData
}
impl CircleClient{
    pub fn new() -> Self{
        Self{
            http: Client::new(),
            api_key: env::var("CIRCLE_WALLET_API_KEY")
                .expect("CIRCLE_WALLET_API_KEY not set"),
            base_url: env::var("CIRCLE_BASE_URL")
                .expect("CIRCLE_BASE_URL not set")
        }
    }
}

pub async fn create_wallet_set(client: &CircleClient, entity_secret_ciphertext: &str) -> Result<String, reqwest::Error>{

    // let client = CircleClient::new();

    let body = CreateWalletSetRequest{
        idempotencyKey: uuid::Uuid::new_v4().to_string(),
        name: "My Wallet Set".to_string(),
        entitySecretCiphertext: entity_secret_ciphertext.to_string()
    };

    let res = client
        .http
        .post(format!("{}/walletSets", client.base_url))
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", &client.api_key))
        .json(&body)
        .send()
        .await?
        .error_for_status()?
        .json::<CreateWalletSetResponse>()
        .await?;

    Ok(res.data.id)
}

pub async fn create_user_wallet(client: &CircleClient, entity_secret_ciphertext: &str, wallet_set_id: &str)->Result<UsersWalletResponseData, reqwest::Error>{


    let body = CreateUserWalletSetRequest{
        idempotency_key: uuid::Uuid::new_v4().to_string(),
        account_type: "SCA".to_string(),
        blockchains: vec!["ARC".to_string()],
        count:2,
        wallet_set_id: wallet_set_id.to_string(),
        entity_secret_ciphertext: entity_secret_ciphertext.to_string()
        };

        let res = client.http.post(format!("{}/wallets", client.base_url))
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", &client.api_key))
        .json(&body)
        .send()
        .await?
        .error_for_status()?
        .json::<CreateUsersWalletResponseData>()
        .await?;
    Ok(res.data)
}