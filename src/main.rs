use crate::models::Feed;
use axum::extract::Path;
use axum::response::Json;
use axum::{routing::get, Router};
use error::AppResult;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::{json, Value};

mod controller;
mod error;
mod models;

#[derive(Debug, Deserialize)]
struct AddFeedPayload {
    url: String,
}

async fn all_feeds() -> Json<Value> {
    Json(json!({
        "data": "all_feeds"
    }))
}

async fn add_feed(Json(payload): Json<AddFeedPayload>) -> AppResult<(StatusCode, Json<Value>)> {
    let feed_source = controller::fetch_feed(&payload.url).await?;

    let feed = Feed::try_from(feed_source)?;

    // TODO: add to db

    let data = json!({ "data": feed });
    Ok((StatusCode::OK, Json(data)))
}

async fn feed_items(Path(feed_id): Path<i32>) -> Json<Value> {
    Json(json!({ "data": format!("feed_items for feed {feed_id}") }))
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/feeds", get(all_feeds).post(add_feed))
        .route("/feeds/:feed_id/items", get(feed_items));

    let addr = "127.0.0.1:3000".parse().unwrap();
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
