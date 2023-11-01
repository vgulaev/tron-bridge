use tokio_postgres;
mod pg_client;
use app_config;

pub struct Clients {
  pub pg: tokio_postgres::Client,
  pub http: reqwest::Client,
}

pub struct AppState {
  pub app_name: String,
  pub client: Clients,
  pub api_host: String,
}

pub async fn get_app_state() -> AppState {
  let cfg = app_config::Config::new();
  AppState {
    app_name: String::from("Hello!!!"),
    client: Clients {
      pg: pg_client::get_pg_connection().await.unwrap(),
      http: reqwest::Client::new(),
     },
    api_host: cfg.api_host().to_string(),
  }
}
