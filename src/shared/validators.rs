// src/shared/validators.rs

pub struct Validators;

impl Validators {
    /// Validates a username.
    pub fn validate_username(username: &str) -> Result<(), String> {
        let username = username.trim();

        if username.is_empty() {
            return Err("Username cannot be empty.".to_string());
        }

        if username.len() < 3 {
            return Err("Username must be at least 3 characters.".to_string());
        }

        if username.len() > 30 {
            return Err("Username cannot exceed 30 characters.".to_string());
        }

        if !username
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            return Err(
                "Username may only contain letters, numbers and underscores."
                    .to_string(),
            );
        }

        Ok(())
    }

    /// Validates a password before hashing.
    pub fn validate_password(password: &str) -> Result<(), String> {
        if password.is_empty() {
            return Err("Password cannot be empty.".to_string());
        }

        if password.len() < 8 {
            return Err(
                "Password must be at least 8 characters."
                    .to_string(),
            );
        }

        if password.len() > 128 {
            return Err(
                "Password cannot exceed 128 characters."
                    .to_string(),
            );
        }

        Ok(())
    }

    /// Validates a note title.
    pub fn validate_note_title(title: &str) -> Result<(), String> {
        let title = title.trim();

        if title.is_empty() {
            return Err("Title cannot be empty.".to_string());
        }

        if title.len() > 100 {
            return Err(
                "Title cannot exceed 100 characters."
                    .to_string(),
            );
        }

        Ok(())
    }

    /// Validates note content.
    pub fn validate_note_content(content: &str) -> Result<(), String> {
        if content.trim().is_empty() {
            return Err("Note content cannot be empty.".to_string());
        }

        Ok(())
    }
}