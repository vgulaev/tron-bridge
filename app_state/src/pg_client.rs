use tokio_postgres::{Client, Config, NoTls};
use std::io::Result;
use app_config;

pub async fn get_pg_connection() -> Result<Client> {
  let cfg = app_config::Config::new();
  let (client, connection) = Config::new()
    .host(&cfg.pg.host)
    .port(5432)
    .user(&cfg.pg.user)
    .password(&cfg.pg.pass)
    .dbname(&cfg.pg.db)
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
