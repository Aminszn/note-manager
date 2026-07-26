// src/auth/session.rs

use uuid::Uuid;

#[derive(Debug, Default)]
pub struct Session {
    current_user_id: Option<Uuid>,
    logged_in: bool,
}

impl Session {
    /// Creates a new empty session.
    pub fn new() -> Self {
        Self {
            current_user_id: None,
            logged_in: false,
        }
    }

    /// Logs a user into the current session.
    pub fn login(&mut self, user_id: Uuid) {
        self.current_user_id = Some(user_id);
        self.logged_in = true;
    }

    /// Logs the current user out.
    pub fn logout(&mut self) {
        self.current_user_id = None;
        self.logged_in = false;
    }

    /// Returns true if a user is logged in.
    pub fn is_authenticated(&self) -> bool {
        self.logged_in
    }

    /// Returns the current user's ID.
    pub fn current_user_id(&self) -> Option<Uuid> {
        self.current_user_id
    }

    /// Returns true if no user is logged in.
    pub fn is_guest(&self) -> bool {
        !self.logged_in
    }
}