use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProxyResponse {
    pub results: Vec<Proxy>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Proxy {
    pub id: String,
    pub username: String,
    pub password: String,
    pub proxy_address: String,
    pub port: u16,
    pub valid: bool,
    pub last_verification: String,
    pub country_code: String,
    pub city_name: String,
    pub asn_name: String,
    pub asn_number: u32,
    pub high_country_confidence: bool,
    pub created_at: String,
}

pub async fn get_proxies(apikey: &str, page: i32) -> Result<ProxyResponse, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!("https://proxy.webshare.io/api/v2/proxy/list/?mode=direct&page={}&page_size=100&valid=true", page);
    let response = client
        .get(url)
        .header("Authorization", format!("Token {}", apikey))
        .send()
        .await?;

    let proxy_list = response.json::<ProxyResponse>().await?;
    Ok(proxy_list)
}