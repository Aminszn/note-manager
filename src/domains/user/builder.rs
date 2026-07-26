// src/domains/user/builder.rs

use chrono::Utc;
use uuid::Uuid;

use crate::shared::validators::Validators;

use super::entity::User;

#[derive(Debug, Default)]
pub struct UserBuilder {
    id: Option<Uuid>,
    username: Option<String>,
    password_hash: Option<String>,
}

impl UserBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    pub fn password_hash(mut self, password_hash: impl Into<String>) -> Self {
        self.password_hash = Some(password_hash.into());
        self
    }

    /// Builds a complete User entity.
    pub fn build(self) -> Result<User, String> {
        let now = Utc::now();

        let username = self.username.ok_or("Username is required.")?;
        Validators::validate_username(&username)?;

        let password_hash = self
            .password_hash
            .ok_or("Password is required.")?;

        Ok(User::new(
            self.id.unwrap_or_else(Uuid::new_v4),
            username.trim().to_string(),
            password_hash,
            now,
            now,
        ))
    }
}