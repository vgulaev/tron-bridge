use sha256::digest;
use core::str;
use hex::{FromHex, encode, decode};
use serde_json::{json, Value};
use rand::{distributions::Alphanumeric, Rng};
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng, generic_array::GenericArray}, Aes256Gcm, Key
};

const PASS: &str = "mypass";

fn get_random_str() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(rand::thread_rng().gen_range(16..64))
        .map(char::from)
        .collect()
}

fn get_cipher() -> Aes256Gcm {
    let dst = digest(PASS);
    let pass = <[u8; 32]>::from_hex(dst).unwrap();
    let key: &Key<Aes256Gcm> = &pass.into();
    Aes256Gcm::new(&key)
}

pub fn encript_key(private_key: &str) -> String {
    let cipher = get_cipher();
    let input = json!([
        get_random_str(),
        private_key,
        get_random_str(),
    ]);
    // nonce is 96 bits, 12 bytes
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, input.to_string().as_bytes().as_ref()).unwrap();
    encode([ciphertext.as_slice(), &nonce].concat())
}

pub fn decrypt_key(abracadabra: String) -> String {
    let input = decode(abracadabra).unwrap();
    // let nonce: &Nonce<Aes256Gcm> = Nonce::from_slice();
    let nonce = GenericArray::from_slice(input.get((input.len() - 12)..input.len()).unwrap());
    let cipher = get_cipher();

    let plaintext = cipher.decrypt(&nonce, input.get(0..(input.len() - 12)).unwrap()).unwrap();
    let jsoned: Value = serde_json::from_slice(&plaintext).unwrap();

    String::from(jsoned.as_array().unwrap()[1].as_str().unwrap())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_test() {
    const PK: &str = "ab4a34b671936ef061602752afe26fd13a31ce75d47d0c02401ae3fdcbca968a";
    let abracadabra = encript_key(&PK);
    assert_eq!( abracadabra.len() > 64, true);
    let private_key = decrypt_key(abracadabra);
    assert_eq!(private_key.as_str(), PK);
  }
}
