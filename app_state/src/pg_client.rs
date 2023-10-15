use tokio_postgres::{Client, Config, NoTls};
use std::io::Result;
use app_config;

pub async fn get_pg_connection() -> Result<Client> {
  let (client, connection) = Config::new()
    .host(app_config::PG_HOST)
    .port(5432)
    .user(app_config::PG_USER)
    .password(app_config::PG_PASS)
    .dbname(app_config::PG_DATABASE)
    // ("postgres://sh:7aw5AXSg@sh_postgres_dev:5432/barnoex_api_db")
    .connect(NoTls)
    .await
    .unwrap();

  tokio::spawn(async move {
    if let Err(e) = connection.await {
      eprintln!("connection error: {}", e);
    }
  });

  Ok(client)
}
