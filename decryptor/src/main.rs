use aes_gcm::{
    aead::{Aead, Nonce},
    Aes256Gcm, Key, KeyInit,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use rsa::{pkcs1::DecodeRsaPrivateKey, Pkcs1v15Encrypt, RsaPrivateKey};
use std::fs;

fn main() {
    let machine_info = String::from("ZEZieWkya2YwT2dtbWJMbpX1ZTE46th81aHOs/9qdBbswNnrI/VjdOWZ7yvSf777tSdQtr7vG53+iqhyLYJ0sd64PY8EXSO08WPbk8kCR8f9NW2oQXPsrwgzPTWVOAjyUv+GAfOWTn07Z1bWDDdyOhSnTknrFm/AhY8rprh7asDj8a/EUIBXDILFjDwGbQ8H/m45gP0FvY+cxAEk8cjtgvSLoTP2CQnS4Dg5uGvbZ3mvAEQhyre+eELCGFZXUX9p5fS1lxj72W2/L/Az9ruJoBtuiMfwAabcYELkxwirjWwIbrj8AHft7mfOcJ7gNQQiZLT67v7JFlMW3XM/cNoWsabgkDVtKkJnqzSLndsKWyE=");
    let machine_info = STANDARD.decode(machine_info).unwrap();

    let (machine_id, encrypted_aes_key) = machine_info.split_at(16);
    let machine_id = String::from_utf8(machine_id.to_vec()).unwrap();

    println!("machine id: {}", machine_id);

    let pem = fs::read_to_string("../server/priv.pem").unwrap();
    let rsa_private_key = RsaPrivateKey::from_pkcs1_pem(&pem).unwrap();

    let aes_key = rsa_private_key
        .decrypt(Pkcs1v15Encrypt, encrypted_aes_key)
        .unwrap();

    let buf = fs::read("/tmp/hehe/a").unwrap();
    let (nonce, encrypted_data) = buf.split_at(12);

    let aes_key = Key::<Aes256Gcm>::from_slice(&aes_key);
    let nonce = Nonce::<Aes256Gcm>::from_slice(nonce);

    let cipher = Aes256Gcm::new(aes_key);

    let plaintext = String::from_utf8(cipher.decrypt(nonce, encrypted_data).unwrap()).unwrap();

    println!("decrypted data: {}", plaintext);
}
