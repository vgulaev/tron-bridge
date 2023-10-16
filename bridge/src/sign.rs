use serde_json::{Value, Error};
use app_ecdsa::hex_str_to_limbs;

fn sign_hex_number(hexed: &str) -> String {
  println!("hexed: {:?}", hexed);
  let numb = malachite::Natural::from_limbs_desc(&hex_str_to_limbs(&hexed));
  println!("number: {:?}", numb.to_string());
  String::from("fsdfsfdsfdsf")
}

pub async fn sign_transaction(data: String) -> Result<String, Error> {
  let jsoned_transaction: Value = serde_json::from_str(&data)?;

  println!("{:?}", sign_hex_number(&jsoned_transaction["txID"].as_str().unwrap()));
  Ok(String::from("Signed"))
}
