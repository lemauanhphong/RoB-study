use aes_gcm::{
    aead::{Aead, OsRng},
    AeadCore, Aes256Gcm, Key, KeyInit,
};
use once_cell::sync::Lazy;
use rsa::pkcs1::EncodeRsaPrivateKey;
use zeroize::Zeroize;

static mut AES_KEY: Lazy<Key<Aes256Gcm>> = Lazy::new(|| Aes256Gcm::generate_key(OsRng));

fn main() {
    unsafe {
        AES_KEY.to_pkcs1_pem();
        AES_KEY.zeroize();

        let nonce = Aes256Gcm::generate_nonce(OsRng);

        let cipher = Aes256Gcm::new(&AES_KEY);
        let ciphertext = cipher.encrypt(&nonce, "abcdef".as_ref()).unwrap();

        dbg!(ciphertext);
    }
}
