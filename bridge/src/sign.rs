use serde_json::{Value, Error};
use rand::Rng;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::Seed;
use malachite_nz::natural::random::get_random_natural_with_bits;
use app_ecdsa::{natural_from_hex, g};

fn get_rnd_seed() -> Seed {
  let mut bytes: [u8; 32] = [0; 32];
  for i in 0..32 {
    bytes[i] = rand::thread_rng().gen_range(0..255);
  }
  Seed::from_bytes(bytes)
}

fn sign_hex_number(hexed: &str) -> String {
  println!("hexed: {:?}", hexed);
  let numb = natural_from_hex(&hexed);
  let k = get_random_natural_with_bits(&mut random_primitive_ints(get_rnd_seed()), 255);
  let r = g() * k.clone();
  println!("k: {:?}", k.to_string());
  println!("number: {:?}", numb.to_string());
  println!("r: {:?}", r.x);
  String::from("fsdfsfdsfdsf")
}

pub async fn sign_transaction(data: String) -> Result<String, Error> {
  let jsoned_transaction: Value = serde_json::from_str(&data)?;

  println!("{:?}", sign_hex_number(&jsoned_transaction["txID"].as_str().unwrap()));
  Ok(String::from("Signed"))
}
