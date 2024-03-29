use anyhow::{anyhow, bail, Result};
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use orion::{aead, kdf};
use rand::rngs::OsRng;

pub fn pwhash(password: &str) -> Result<String> {
    let argon = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    match argon.hash_password(password.as_bytes(), &salt) {
        Ok(b) => Ok(b.to_string()),
        Err(e) => Err(anyhow!(e.to_string())),
    }
}

pub fn verify_pass(password: &str, pwhash: &str) -> Result<bool> {
    let parsed_hash = match PasswordHash::new(pwhash) {
        Ok(ph) => ph,
        Err(e) => bail!(e.to_string()),
    };
    let valid = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();
    Ok(valid)
}

pub fn gen_key(pass: &str, salt: &str) -> Result<aead::SecretKey> {
    let pw = kdf::Password::from_slice(pass.as_bytes())?;
    let salt = kdf::Salt::from_slice(salt.as_bytes())?;
    match kdf::derive_key(&pw, &salt, 3, 1 << 16, 32) {
        Ok(sk) => Ok(sk),
        Err(e) => Err(anyhow!(e.to_string())),
    }
}

pub fn get_salt() -> String {
    SaltString::generate(&mut OsRng).to_string()
}

pub fn encrypt(password: &str, salt: &str, data: &str) -> Result<String> {
    let key = gen_key(password, salt)?;
    let ciphertext = aead::seal(&key, data.as_bytes())?;
    Ok(hex::encode(ciphertext))
}

pub fn decrypt(password: &str, salt: &str, ciphertext: &str) -> Result<String> {
    let key = gen_key(password, salt)?;
    let plaintext = aead::open(&key, &hex::decode(ciphertext)?)?;
    Ok(String::from_utf8(plaintext)?)
}
