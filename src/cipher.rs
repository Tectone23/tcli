use aes::cipher::{generic_array::GenericArray, typenum::U32};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose, Engine as _};

use pbkdf2::{pbkdf2, hmac};

pub fn encrypt_key(api_key: &str, password: &str) -> String {
    let key = generate_key_from_password(password);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message
    let ciphertext = cipher.encrypt(nonce, api_key.as_bytes().as_ref()).unwrap();
    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref()).unwrap();

    let eng = general_purpose::STANDARD;
    let a = eng.encode(ciphertext);

    return format!(
        "encrypted: {} {}\noriginal: {}",
        a,
        "unique nonce",
        String::from_utf8(plaintext).unwrap()
    );
}

fn generate_key_from_password(password: &str) -> GenericArray<u8, U32> {
    let salt = "my-salt".as_bytes();
    let mut key = GenericArray::<u8, U32>::default();
    let iterations = 10000;
    let _ = pbkdf2::<hmac::Hmac<sha2::Sha256>>(password.as_bytes(), salt, iterations, &mut key);
    return key;
}
