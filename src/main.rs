use once_cell::sync::Lazy;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use tokio::fs;

type Error = Box<dyn std::error::Error>;

static CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

const BASE_URL: &str = "https://api.timeular.com/api/v3";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_key = env::var("TMLR_API_KEY").expect("TMLR_API_KEY needs to be set");
    let api_secret = env::var("TMLR_API_SECRET").expect("TMLR_API_SECRET needs to be set");

    println!("signing in..");
    let token = sign_in(api_key, api_secret).await?;

    Ok(())
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SignInRequest {
    api_key: String,
    api_secret: String,
}

#[derive(Deserialize, Debug)]
struct SignInResponse {
    token: String,
}

async fn sign_in(api_key: String, api_secret: String) -> Result<String, Error> {
    let body = SignInRequest {
        api_key,
        api_secret,
    };
    let resp = CLIENT
        .post(&url("/developer/sign-in"))
        .json(&body)
        .send()
        .await?
        .json::<SignInResponse>()
        .await?;
    Ok(resp.token)
}

#[derive(Deserialize, Debug)]
struct MeResponse {
    data: Me,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Me {
    user_id: String,
    name: String,
    email: String,
    default_space_id: String,
}

async fn fetch_me(token: &str) -> Result<Me, Error> {
    let resp = CLIENT
        .get(&url("/me"))
        .header("Authorization", auth(token))
        .send()
        .await?
        .json::<MeResponse>()
        .await?;
    Ok(resp.data)
}

fn url(path: &str) -> String {
    format!("{}{}", BASE_URL, path)
}

fn auth(token: &str) -> String {
    format!("Bearer {}", token)
}
