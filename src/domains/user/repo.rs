// src/domain/user/repo.rs

use uuid::Uuid;

use super::entity::User;

pub trait UserRepository {
    /// Saves a new user.
    fn create(&self, user: &User) -> Result<(), String>;

    /// Finds a user by ID.
    fn find_by_id(&self, id: Uuid) -> Result<Option<User>, String>;

    /// Finds a user by username.
    fn find_by_username(&self, username: &str) -> Result<Option<User>, String>;

    /// Returns every user.
    fn find_all(&self) -> Result<Vec<User>, String>;

    /// Updates an existing user.
    fn update(&self, user: &User) -> Result<(), String>;

    /// Deletes a user.
    fn delete(&self, id: Uuid) -> Result<(), String>;
}