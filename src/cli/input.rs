// src/cli/input.rs

use dialoguer::{theme::ColorfulTheme, Input, Password};

/// Handles all user input for the CLI.
pub struct CliInput;

impl CliInput {
    /// Prompts for a username.
    pub fn username() -> String {
        Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Username")
            .interact_text()
            .expect("Failed to read username")
    }

    /// Prompts for a password.
    pub fn password() -> String {
        Password::with_theme(&ColorfulTheme::default())
            .with_prompt("Password")
            .interact()
            .expect("Failed to read password")
    }

    /// Prompts for a new password.
    pub fn new_password() -> String {
        Password::with_theme(&ColorfulTheme::default())
            .with_prompt("New Password")
            .interact()
            .expect("Failed to read password")
    }

    /// Prompts for a note title.
    pub fn note_title() -> String {
        Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Note Title")
            .interact_text()
            .expect("Failed to read note title")
    }

    /// Prompts for note content.
    pub fn note_content() -> String {
        Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Note Content")
            .interact_text()
            .expect("Failed to read note content")
    }

    /// Prompts for a note ID.
    pub fn note_id() -> String {
        Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Note ID")
            .interact_text()
            .expect("Failed to read note ID")
    }

    /// Waits for the user before continuing.
    pub fn pause() {
        let _: String = Input::<String>::with_theme(
            &ColorfulTheme::default(),
        )
        .with_prompt("Press Enter to continue")
        .allow_empty(true)
        .interact_text()
        .expect("Failed to pause");
    }
}