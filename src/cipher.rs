use crate::errors::throw;
use aes::cipher::{generic_array::GenericArray, typenum::U32};
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    AeadCore, Aes256Gcm, Nonce,
};
use argon2::{password_hash::SaltString, Argon2};
use base64::{engine::general_purpose, Engine as _};

type KeyType = GenericArray<u8, U32>;

const ENCODE_ENGINE: base64::engine::GeneralPurpose = general_purpose::STANDARD;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CipherOut {
    pub txt: String,
    pub nonce: String,
    pub salt: String,
}

pub fn encrypt_key(api_key: &str, password: &str) -> CipherOut {
    let salt = SaltString::generate(&mut OsRng);
    let key = generate_key_from_password(password, salt.as_str().as_bytes());

    match key {
        Ok(key) => {
            let cipher = Aes256Gcm::new(&key);

            let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
            let ciphertext = cipher.encrypt(&nonce, api_key.as_bytes().as_ref()).unwrap();

            let encoded_ciphertext = ENCODE_ENGINE.encode(ciphertext);
            let encoded_nonce = ENCODE_ENGINE.encode(nonce);

            return CipherOut {
                txt: encoded_ciphertext,
                nonce: encoded_nonce,
                salt: String::from(salt.as_str()),
            };
        }
        Err(err) => {
            // this outputs in red text and exits the code
            throw(&format!("{err}"));
            unreachable!();
        }
    };
}

pub fn decrypt_key(
    password: &str,
    encrypted_text: &str,
    nonce: &str,
    salt: &str,
) -> Result<String, String> {
    let key = generate_key_from_password(password, salt.as_bytes());

    match key {
        Ok(key) => {
            let cipher = Aes256Gcm::new(&key);

            let decoded_ciphertext = ENCODE_ENGINE.decode(encrypted_text).unwrap();
            let decoded_nonce = ENCODE_ENGINE.decode(nonce).unwrap();

            let nonce = Nonce::from_slice(&decoded_nonce);
            let decrypt_text = cipher.decrypt(nonce, decoded_ciphertext.as_ref());

            match decrypt_text {
                Ok(decrypt_text) => {
                    // unwrap here should be fine
                    // this is because the output of the decryption
                    // should always be utf8
                    let plaintext = String::from_utf8(decrypt_text).unwrap();

                    return Ok(plaintext);
                }
                Err(_) => {
                    return Err(String::from(
                        "Unable to verify password, check if you have entered it correctly",
                    ))
                }
            };
        }
        Err(err) => {
            throw(&format!("{err}"));
            unreachable!();
        }
    };
}

fn generate_key_from_password(password: &str, salt: &[u8]) -> Result<KeyType, String> {
    let mut key = GenericArray::<u8, U32>::default();

    let res = Argon2::default().hash_password_into(password.as_bytes(), salt, &mut key);

    match res {
        Ok(_) => return Ok(key),
        Err(err) => return Err(format!("Unable to generate hash from the password: {err}")),
    };
}
