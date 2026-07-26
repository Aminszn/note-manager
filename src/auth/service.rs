// src/auth/service.rs

use uuid::Uuid;

  use crate::{
      auth::{
          hasher::PasswordHasher,
          session::Session,
      },
      domains::user::{
          UserDto,
          UserRepository,
          UserService,
      },
      shared::Validators,
  };

  pub struct AuthService<R: UserRepository> {
      user_service: UserService<R>,
      session: Session,
  }

impl<R: UserRepository> AuthService<R> {
    /// Creates a new authentication service.
    pub fn new(user_service: UserService<R>) -> Self {
      Self {
          user_service,
          session: Session::new(),
      }
  }

    /// Registers a new user.
    pub fn register(
        &mut self,
        username: String,
        password: String,
    ) -> Result<UserDto, String> {
        self.user_service.create_user(
            username,
            password,
        )
    }

    /// Logs a user in.
    pub fn login(
        &mut self,
        username: String,
        password: String,
    ) -> Result<UserDto, String> {
        let user = self
            .user_service
            .find_by_username(&username)?;

        let verified =
            PasswordHasher::verify_password(
                &password,
                user.password_hash(),
            )?;

        if !verified {
            return Err(
                "Invalid username or password."
                    .to_string(),
            );
        }

        self.session.login(user.id());

        Ok(user.into())
    }

    /// Logs the current user out.
    pub fn logout(&mut self) {
        self.session.logout();
    }

    /// Changes the current user's password.
    pub fn change_password(
        &mut self,
        user_id: Uuid,
        new_password: String,
    ) -> Result<(), String> {
        self.user_service
            .update_password(user_id, new_password)
    }

    /// Returns whether a user is logged in.
    pub fn is_authenticated(&self) -> bool {
        self.session.is_authenticated()
    }

    /// Returns the current user's ID.
    pub fn current_user_id(&self) -> Option<Uuid> {
        self.session.current_user_id()
    }
}