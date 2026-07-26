// src/domain/user/dto.rs

use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::entity::User;

/// DTO returned when exposing user data.
#[derive(Debug, Clone)]
pub struct UserDto {
    pub id: Uuid,
    pub username: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id(),
            username: user.username().to_string(),
            created_at: *user.created_at(),
            updated_at: *user.updated_at(),
        }
    }
}