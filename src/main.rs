use once_cell::sync::Lazy;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use warp::{http::StatusCode, Filter, Rejection, Reply};

type WebResult<T> = std::result::Result<T, Rejection>;
type Error = Box<dyn std::error::Error>;

static CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

const BASE_URL: &str = "https://api.timeular.com/api/v3";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TrackingStartedPayload {
    user_id: String,
    even_type: String,
    data: TrackingStartedData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TrackingStartedData {
    current_tracking: Tracking,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Tracking {
    id: i64,
    activity: Activity,
    started_at: String,
    note: Note,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Note {
    text: Option<String>,
    tags: Vec<TagOrMention>,
    mentions: Vec<TagOrMention>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TagOrMention {
    id: i64,
    key: String,
    label: String,
    scope: String,
    space_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Activity {
    id: String,
    name: String,
    color: String,
    integration: String,
    space_id: String,
    device_side: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TrackingStoppedPayload {
    user_id: String,
    even_type: String,
    data: TrackingStoppedData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TrackingStoppedData {
    new_time_entry: TimeEntry,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TimeEntry {
    id: String,
    activity: Activity,
    duration: Duration,
    note: Note,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Duration {
    started_at: String,
    stopped_at: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let api_key = env::var("TMLR_API_KEY").expect("TMLR_API_KEY needs to be set");
    // let api_secret = env::var("TMLR_API_SECRET").expect("TMLR_API_SECRET needs to be set");

    // println!("signing in..");
    // let token = sign_in(api_key, api_secret).await?;

    // println!("fetching available events...");
    // let events = fetch_events(&token).await?;
    // println!("available events: {:?}", events);

    let started_tracking_route = warp::path!("started-tracking")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(started_tracking_handler);

    let stopped_tracking_route = warp::path!("stopped-tracking")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(stopped_tracking_handler);

    // TODO: ngrok alternative: https://theboroer.github.io/localtunnel-www/
    // TODO: subscribe to started-tracking and stopped tracking
    // TODO: list subscriptions
    // TODO: create handler for started/stoppped tracking
    // TODO: unsubscribe

    warp::serve(started_tracking_route.or(stopped_tracking_route))
        .run(([127, 0, 0, 1], 8000))
        .await;
    Ok(())
}

async fn started_tracking_handler(body: TrackingStartedPayload) -> WebResult<impl Reply> {
    Ok("OK")
}

async fn stopped_tracking_handler(body: TrackingStoppedPayload) -> WebResult<impl Reply> {
    Ok("OK")
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
struct EventsResponse {
    events: Vec<String>,
}

async fn fetch_events(token: &str) -> Result<Vec<String>, Error> {
    let resp = CLIENT
        .get(&url("/webhooks/event"))
        .header("Authorization", auth(token))
        .send()
        .await?
        .json::<EventsResponse>()
        .await?;
    Ok(resp.events)
}

fn url(path: &str) -> String {
    format!("{}{}", BASE_URL, path)
}

fn auth(token: &str) -> String {
    format!("Bearer {}", token)
}
