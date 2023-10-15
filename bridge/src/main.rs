use actix_web::{web, App, HttpServer, Responder};
// use tokio::time::{sleep, Duration};
use serde::Deserialize;
use app_state::{AppState, get_app_state};

// use env_logger::Env;

#[derive(Deserialize, Debug)]
struct InsertWalletAddress {
  address: String,
  public_key: String,
  private_key: String,
}

// async fn test_postgres() -> Result<String> {
//   // let (client, connection) = tokio_postgres::connect(
//   //   Config::new(), NoTls
//   // ).await.unwrap();
//   let client = get_pg_connection().await.unwrap();

//   // Now we can execute a simple statement that just returns its parameter.
//   let rows = client
//     .query("SELECT $1::TEXT", &[&"hello world"])
//     .await
//     .unwrap();

//   // And then check that we got back the same string we sent over.
//   let value: &str = rows[0].get(0);
//   // println!("Test: {:?}", value);
//   Ok(String::from(value))
// }

async fn insert_wallet_address(
  app_state: web::Data<AppState>,
  body: web::Json<InsertWalletAddress>,
) -> impl Responder {
  app_state
    .pg_client
    .query(
      "INSERT INTO wallet_address (address, public_key, private_key) VALUES ($1::TEXT, $2::TEXT, $3::TEXT)",
      &[&body.address, &body.public_key, &body.private_key],
    )
    .await
    .unwrap();

  format!("Inserted!!! {}", body.address)
}

async fn me() -> impl Responder {
  "This is amazing Tron Bridge"
}

async fn not_found() -> impl Responder {
  "Looks like no page here"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  println!("Server has started");

  let state = web::Data::new(get_app_state().await);

  HttpServer::new(move || {
    App::new()
      .app_data(state.clone())
      .service(
        web::scope("/api")
        .route(
          "/me",
          web::post().to(me),
        )
        .route(
            "/insert_wallet_address",
            web::post().to(insert_wallet_address),
          )
      )
      .default_service(web::route().to(not_found))
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await
}
