use axum::{extract, prelude::*, response};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
pub async fn main() {
  let app = route("/users/:user_id/posts/:post_id", get(user_post));
  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  hyper::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn user_post(extract::UrlParams(params): extract::UrlParams<(u64, u64)>) -> String {
  let user_id = params.0;
  let post_id = params.1;
  format!("user_id: {}, post_id: {}", user_id, post_id)
}
