use serde_json::{Value, Error};
use app_ecdsa::sign_hex_number;

pub async fn sign_transaction(data: String, private_key: &str) -> Result<String, Error> {
  let mut jsoned_transaction: Value = serde_json::from_str(&data)?;
  let signature = sign_hex_number(&jsoned_transaction["txID"].as_str().unwrap(), private_key);
  jsoned_transaction["signature"] = Value::Array(vec![Value::String(signature)]);
  Ok(jsoned_transaction.to_string())
}
