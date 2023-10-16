use tokio_postgres;
mod pg_client;

pub struct Clients {
  pub pg: tokio_postgres::Client,
  pub http: reqwest::Client,
}

pub struct AppState {
  pub app_name: String,
  pub client: Clients,
}

pub async fn get_app_state() -> AppState {
  AppState {
    app_name: String::from("Hello!!!"),
    client: Clients {
      pg: pg_client::get_pg_connection().await.unwrap(),
      http: reqwest::Client::new(),
     },
  }
}
