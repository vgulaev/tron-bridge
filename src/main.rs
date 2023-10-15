use actix_web::{web, App, HttpServer, Responder};
// use tokio::time::{sleep, Duration};
use tokio_postgres::{Client, Config, NoTls};
// use tokio_postgres::NoTls;
// use crate::config::ExampleConfig;
use std::io::Result;
mod app_config;
// use serde::Deserialize;
// use env_logger::Env;

struct AppState {
  app_name: String,
  pg_client: Client,
}

// #[derive(Deserialize)]
// struct InsertWalletAddress {
//   adress: String,
//   public_key: String,
//   private_key: String,
// }

async fn get_pg_connection() -> Result<Client> {
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

async fn test_postgres() -> Result<String> {
  // let (client, connection) = tokio_postgres::connect(
  //   Config::new(), NoTls
  // ).await.unwrap();
  let client = get_pg_connection().await.unwrap();

  // Now we can execute a simple statement that just returns its parameter.
  let rows = client
    .query("SELECT $1::TEXT", &[&"hello world"])
    .await
    .unwrap();

  // And then check that we got back the same string we sent over.
  let value: &str = rows[0].get(0);
  // println!("Test: {:?}", value);
  Ok(String::from(value))
}

async fn insert_wallet_address(
  app_state: web::Data<AppState>
  // body: web::Json<InsertWalletAddress>
) -> impl Responder {
  // sleep(Duration::from_millis(100)).await;
  println!("{:?}", app_state.app_name);
  // println!("Body: {:?}", body);

  let rows = app_state
    .pg_client
    .query("SELECT $1::TEXT", &[&"hello world"])
    .await
    .unwrap();

  let value: &str = rows[0].get(0);

  format!("Hello world!: {}", value)
}

async fn index10(app_state: web::Data<AppState>) -> impl Responder {
  // sleep(Duration::from_millis(100)).await;
  println!("{:?}", app_state.app_name);

  app_state
    .pg_client
    .query("SELECT pg_sleep(10)", &[])
    .await
    .unwrap();

  // let value: &str = rows[0].get(0);

  format!("Stack SQL 10 sec")
}

async fn not_found() -> impl Responder {
  "Looks like no page here"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  println!("Server has started");

  let state = web::Data::new(AppState {
    app_name: String::from("Hello!!!"),
    pg_client: get_pg_connection().await.unwrap(),
  });

  HttpServer::new(move || {
    App::new()
      .app_data(state.clone())
      .service(
        // prefixes all resources and routes attached to it...
        web::scope("/api")
          // ...so this handles requests for `GET /app/index.html`
          .route("/insert_wallet_address", web::post().to(insert_wallet_address))
          .route("/index10.html", web::get().to(index10)),
      )
      .default_service(web::route().to(not_found))
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await
}
