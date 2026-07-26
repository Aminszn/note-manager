// src/domain/user/service.rs

use uuid::Uuid;

use crate::{
    auth::PasswordHasher,
    shared::Validators,
};

use super::{
    builder::UserBuilder,
    dto::UserDto,
    entity::User,
    repo::UserRepository,
};

pub struct UserService<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> UserService<R> {
    /// Creates a new service.
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    
    /// Registers a new user.
    pub fn create_user(
        &self,
        username: String,
        password: String,
    ) -> Result<UserDto, String> {
        Validators::validate_username(&username)?;
        Validators::validate_password(&password)?;

        if self
            .repository
            .find_by_username(&username)?
            .is_some()
        {
            return Err("Username already exists".into());
        }

        let password_hash =
            PasswordHasher::hash_password(&password)?;

        let user = UserBuilder::new()
            .username(username)
            .password_hash(password_hash)
            .build()
            .map_err(|e| e.to_string())?;

        self.repository.create(&user)?;

        Ok(user.into())
    }

    /// Gets a user by ID.
    pub fn get_user(&self, id: Uuid) -> Result<UserDto, String> {
        let user = self
            .repository
            .find_by_id(id)?
            .ok_or("User not found")?;

        Ok(user.into())
    }

    /// Finds a user by username.
    pub fn find_by_username(
        &self,
        username: &str,
    ) -> Result<User, String> {
        self.repository
            .find_by_username(username)?
            .ok_or("User not found".to_string())
    }

    /// Gets every user.
    pub fn get_all_users(&self) -> Result<Vec<UserDto>, String> {
        let users = self.repository.find_all()?;

        Ok(users.into_iter().map(UserDto::from).collect())
    }

    /// Deletes a user.
    pub fn delete_user(&self, id: Uuid) -> Result<(), String> {
        self.repository.delete(id)
    }

    /// Updates a username.
    pub fn update_username(
        &self,
        id: Uuid,
        username: String,
    ) -> Result<UserDto, String> {
        let mut user = self
            .repository
            .find_by_id(id)?
            .ok_or("User not found")?;

        user.change_username(username);

        self.repository.update(&user)?;

        Ok(user.into())
    }

    /// Updates a password.
    pub fn update_password(
        &self,
        id: Uuid,
        new_password: String,
    ) -> Result<(), String> {
        Validators::validate_password(&new_password)?;

        let mut user = self
            .repository
            .find_by_id(id)?
            .ok_or("User not found")?;

        let password_hash =
            PasswordHasher::hash_password(&new_password)?;

        user.change_password_hash(password_hash);

        self.repository.update(&user)
    }
}