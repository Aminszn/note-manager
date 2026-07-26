// src/domain/user/commands.rs

use uuid::Uuid;

use super::{
    dto::UserDto,
    repo::UserRepository,
    service::UserService,
};

pub struct UserCommands<R: UserRepository> {
    service: UserService<R>,
}

impl<R: UserRepository> UserCommands<R> {
    /// Creates a new command handler.
    pub fn new(service: UserService<R>) -> Self {
        Self { service }
    }

    /// Register a new user.
    pub fn register(
        &self,
        username: String,
        password_hash: String,
    ) -> Result<UserDto, String> {
        self.service.create_user(username, password_hash)
    }


    /// View a user.
    pub fn get_user(&self, id: Uuid) -> Result<UserDto, String> {
        self.service.get_user(id)
    }

    /// Change username.
    pub fn change_username(
        &self,
        id: Uuid,
        username: String,
    ) -> Result<UserDto, String> {
        self.service.update_username(id, username)
    }

    /// Change password.
    pub fn change_password(
        &self,
        id: Uuid,
        password_hash: String,
    ) -> Result<(), String> {
        self.service.update_password(id, password_hash)
    }

    /// Delete account.
    pub fn delete_account(&self, id: Uuid) -> Result<(), String> {
        self.service.delete_user(id)
    }
}