// use std::sync::Arc;
// use actix_rt::System;
// use awc::{http::header, Client, Connector};
// use rustls::{ClientConfig, OwnedTrustAnchor, RootCertStore};
// use serde_json::{Result, Value, json};
// // use secp256k1::{Secp256k1, SecretKey, Message};
// // use secp256k1::hashes::sha256;
// use std::str::FromStr;
// use k256::{
//     ecdsa::{SigningKey, Signature, signature::Signer},
//     SecretKey,
// };

fn main() {
    System::new().block_on(async {
        let client_tls_config = Arc::new(rustls_config());
        // let client = Client::default();
        let client = Client::builder()
            // Wikipedia requires a User-Agent header to make requests
            .add_default_header((header::USER_AGENT, "baronex-tron-bridge"))
            .add_default_header((header::ACCEPT, "application/json"))
            .add_default_header((header::CONTENT_TYPE, "application/json"))
            // a "connector" wraps the stream into an encrypted connection
            .connector(Connector::new().rustls_021(Arc::clone(&client_tls_config)))
            .finish();

        // let res = client
        //     .get("https://www.rust-lang.org/")    // <- Create request builder
        //     .insert_header(("User-Agent", "Actix-web"))
        //     .send()                             // <- Send http request
        //     .await
        //     .unwrap();

        // println!("Response: {:?}", res);        // <- server http response

        // let payload = serde_json::json!({
        //     "address": "TXJdFrZbfL1fZcwkAs7HtgGHNdRjYnviwV",
        //     "visible": "true"
        // });

        // let mut res1 = client
        //     .post("https://api.shasta.trongrid.io/wallet/getaccount")
        //     .send_json(&payload)
        //     .await
        //     .unwrap();

        // println!("Response: {:?}", res1);
        // println!("Body: {:?}", res1.body().await.unwrap())

        let payload = serde_json::json!({
            "owner_address": "TXJdFrZbfL1fZcwkAs7HtgGHNdRjYnviwV",
            "to_address": "TJSQdBmanjLzvj8zhZgEvtzmsVqDMt3QKH",
            "amount": 50,
            "visible": true
        });

        let url = "https://api.shasta.trongrid.io/wallet/createtransaction";
        let mut res1 = client
            .post(url)
            .send_json(&payload)
            .await
            .unwrap();

        let raw_transaction = res1.body().await.unwrap();

        let mut v: Value = serde_json::from_slice(&raw_transaction).unwrap();

        let secret_key = "ab4a34b671936ef061602752afe26fd13a31ce75d47d0c02401ae3fdcbca968a".as_bytes();
        let signing_key = SigningKey::from_bytes(secret_key.try_into().unwrap());
        // let secp = Secp256k1::new();
        // let secret_key = SecretKey::from_str("ab4a34b671936ef061602752afe26fd13a31ce75d47d0c02401ae3fdcbca968a").unwrap();
        // let message = Message::from_hashed_data::<sha256::Hash>(&raw_transaction);

        // let sig = secp.sign_ecdsa(&message, &secret_key);
        // v["signature"] = json!(sig.to_string());
        // let secret_key_serialized = "ab4a34b671936ef061602752afe26fd13a31ce75d47d0c02401ae3fdcbca968a";
        // let secret_key = secret_key_serialized.parse::<SecretKey>().unwrap();
        // let signing_key: SigningKey = secret_key.into();
        // let signature = signing_key.sign(&raw_transaction);
        // 47b1f77b3e30cfbbfa41d795dd34475865240617dd1c5a7bad526f5fd89e52cd057c80b665cc2431efab53520e2b1b92a0425033baee915df858ca1c588b0a1800

        println!("Response: {:?}", res1);
        println!("Body: {:?}", v);
        // println!("signature {:?}", sig);

        let url = "https://api.shasta.trongrid.io/wallet/broadcasttransaction";
        let mut res2 = client
            .post(url)
            .send_json(&v)
            .await
            .unwrap();

        println!("broadcasttransaction: {:?}", res2);
        println!("broadcasttransaction: {:?}", res2.body().await.unwrap());
    });
}

/// Create simple rustls client config from root certificates.
fn rustls_config() -> ClientConfig {
    let mut root_store = RootCertStore::empty();
    root_store.add_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.iter().map(|ta| {
        OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));

    rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth()
}
