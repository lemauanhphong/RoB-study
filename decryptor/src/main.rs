use aes_gcm::{
    aead::{Aead, Nonce},
    Aes256Gcm, Key, KeyInit,
};
use std::{
    env::args,
    fs::{self, read_dir},
};

fn decrypt_file(path: &str, aes_key: &[u8]) {
    let buf = fs::read(path).unwrap();
    let (nonce, encrypted_data) = buf.split_at(12);

    let aes_key = Key::<Aes256Gcm>::from_slice(aes_key);
    let nonce = Nonce::<Aes256Gcm>::from_slice(nonce);

    let cipher = Aes256Gcm::new(aes_key);

    match cipher.decrypt(nonce, encrypted_data) {
        Ok(plaintext) => {
            fs::write(path, plaintext).unwrap();

            println!("Decrypted: {}", path);
        }
        Err(_) => {
            print!("Wrong key!");
        }
    }
}

fn visit(dir: &str, aes_key: &[u8]) {
    for entry in read_dir(dir).unwrap() {
        let path = entry.unwrap().path();

        if path.is_file() {
            decrypt_file(path.to_str().unwrap(), aes_key);
        } else {
            visit(path.to_str().unwrap(), aes_key);
        }
    }
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        println!("Usage: ./decryptor FOLDER");
        return;
    }

    let aes_key =
        hex::decode("42587F1D866C9C98D013A27F17B12995095A8D2DBA382DB43A08BF500E80B149").unwrap();

    visit(&args[1], &aes_key);
}
