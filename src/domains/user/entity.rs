// src/domains/user/entity.rs

use chrono::{DateTime, Utc};
use uuid::Uuid;

/// User aggregate root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    id: Uuid,
    username: String,
    password_hash: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl User {
    /// Creates a fully initialized User.
    pub fn new(
        id: Uuid,
        username: String,
        password_hash: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            username,
            password_hash,
            created_at,
            updated_at,
        }
    }

    // -------------------------
    // Getters
    // -------------------------

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password_hash(&self) -> &str {
        &self.password_hash
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    // -------------------------
    // Domain Behavior
    // -------------------------

    /// Changes the username.
    pub fn change_username(&mut self, username: String) {
        self.username = username;
        self.touch();
    }

    /// Changes the password hash.
    pub fn change_password_hash(&mut self, password_hash: String) {
        self.password_hash = password_hash;
        self.touch();
    }

    /// Updates the modification timestamp.
    fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}