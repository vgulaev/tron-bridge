use actix_web::{
  http::header::ContentType, middleware::Logger, web, App, HttpResponse, HttpServer, Responder,
};
// use tokio::time::{sleep, Duration};
use app_config::TRON_URL;
use app_state::{get_app_state, AppState};
use env_logger::Env;
use serde::Deserialize;
use serde_json::json;
mod sign;

#[derive(Deserialize, Debug)]
struct InsertWalletAddress {
  address: String,
  public_key: String,
  private_key: String,
}

#[derive(Deserialize, Debug)]
struct TronAddress {
  address: String,
}

#[derive(Deserialize, Debug)]
struct CreateSignedTransaction {
  from: String,
  to: String,
  amount: u128,
}

async fn insert_wallet_address(
  app_state: web::Data<AppState>,
  body: web::Json<InsertWalletAddress>,
) -> impl Responder {
  app_state
    .client.pg
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

async fn not_found() -> HttpResponse {
  HttpResponse::NotFound()
    .content_type(ContentType::plaintext())
    .body("Looks like no page here")
}

async fn accounts(body: web::Json<TronAddress>) -> HttpResponse {
  let url = format!("{TRON_URL}/v1/accounts/{}", body.address);
  let resp = reqwest::get(url).await.unwrap().text().await.unwrap();
  HttpResponse::Ok()
    .content_type(ContentType::json())
    .body(resp)
}

async fn create_signed_transaction(
  app_state: web::Data<AppState>,
  body: web::Json<CreateSignedTransaction>,
) -> impl Responder {
  let rows = app_state
    .client
    .pg
    .query(
      "SELECT private_key FROM wallet_address WHERE address = $1::TEXT",
      &[&body.from],
    )
    .await
    .unwrap();
  let private_key: &str = rows[0].get(0);
  let url = format!("{TRON_URL}/wallet/createtransaction");
  let resp = app_state
    .client
    .http
    .post(url)
    .body(
      json!({
        "owner_address": body.from,
        "to_address": body.to,
        "amount": body.amount,
        "visible": true,
      })
      .to_string(),
    )
    .send()
    .await
    .unwrap()
    .text()
    .await
    .unwrap();
  let signed = sign::sign_transaction(resp, private_key).await.unwrap();

  let url = format!("{TRON_URL}/wallet/broadcasttransaction");

  let resp = app_state
    .client
    .http
    .post(url)
    .body(signed)
    .send()
    .await
    .unwrap();

  format!("create_signed_transaction {:?}\n{:?}", body, resp)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  println!("Server has started");

  let state = web::Data::new(get_app_state().await);
  env_logger::init_from_env(Env::default().default_filter_or("info"));

  HttpServer::new(move || {
    App::new()
      .wrap(Logger::default())
      .app_data(state.clone())
      .service(
        web::scope("/api")
          .route("/me", web::post().to(me))
          .route(
            "/insert_wallet_address",
            web::post().to(insert_wallet_address),
          )
          .route(
            "/create_signed_transaction",
            web::post().to(create_signed_transaction),
          )
          .service(web::scope("tron").route("/accounts", web::post().to(accounts))),
      )
      .default_service(web::route().to(not_found))
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await
}
