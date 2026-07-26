// src/auth/hasher.rs

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash,
        PasswordHasher as ArgonPasswordHasher,
        PasswordVerifier,
        SaltString,
    },
    Argon2,
};

pub struct PasswordHasher;

impl PasswordHasher {
    /// Hashes a plain text password.
    pub fn hash_password(password: &str) -> Result<String, String> {
        let salt = SaltString::generate(&mut OsRng);

        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| e.to_string())?
            .to_string();

        Ok(password_hash)
    }

    /// Verifies a plain text password against a stored hash.
    pub fn verify_password(
        password: &str,
        password_hash: &str,
    ) -> Result<bool, String> {
        let parsed_hash = PasswordHash::new(password_hash)
            .map_err(|e| e.to_string())?;

        match Argon2::default().verify_password(
            password.as_bytes(),
            &parsed_hash,
        ) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}