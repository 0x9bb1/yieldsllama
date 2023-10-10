use serde::{Deserialize, Serialize};

pub async fn request() -> anyhow::Result<Response> {
    dotenv::dotenv().ok();
    let domain = dotenv::var("LLAMA_DOMAIN")?;
    let response = reqwest::get(domain + "/pools")
        .await?
        .json::<Response>()
        .await?;

    Ok(response)
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: String,
    pub data: Vec<Pool>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Pool {
    pub chain: String,
    pub project: String,
    pub symbol: String,
    #[serde(rename = "tvlUsd")]
    pub tvl_usd: u128,
    #[serde(rename = "apyBase")]
    pub apy_base: Option<f32>,
    #[serde(rename = "apyReward")]
    pub apy_reward: Option<f32>,
    pub apy: Option<f32>,
    pub exposure: String,
}
