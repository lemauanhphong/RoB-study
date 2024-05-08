mod utils;

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    AeadCore, Aes256Gcm, Key,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn encrypt(plaintext: Vec<u8>) -> Vec<u8> {
    // let key = Aes256Gcm::generate_key(OsRng);
    let key = String::from("a").repeat(32);

    let key = Key::<Aes256Gcm>::from_slice(key.as_bytes());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let cipher = Aes256Gcm::new(key);

    let ciphertext = cipher.encrypt(&nonce, plaintext.as_ref()).unwrap();

    let mut encrypted = nonce.to_vec();
    encrypted.extend_from_slice(&ciphertext);

    encrypted
}
