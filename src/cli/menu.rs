// src/cli/menu.rs

use dialoguer::{theme::ColorfulTheme, Select};

use super::command::Command;

/// Displays the guest menu.
pub struct Menu;

impl Menu {
    /// Menu shown before authentication.
    pub fn guest_menu() -> Command {
        let items = [
            "Register",
            "Login",
            "Exit",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Welcome")
            .items(&items)
            .default(0)
            .interact()
            .expect("Failed to display guest menu");

        match selection {
            0 => Command::Register,
            1 => Command::Login,
            _ => Command::Exit,
        }
    }

    /// Menu shown after authentication.
    pub fn user_menu() -> Command {
        let items = [
            "Create Note",
            "View Notes",
            "View Note",
            "Update Note",
            "Delete Note",
            "Change Password",
            "Logout",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Main Menu")
            .items(&items)
            .default(0)
            .interact()
            .expect("Failed to display user menu");

        match selection {
            0 => Command::CreateNote,
            1 => Command::ViewNotes,
            2 => Command::ViewNote,
            3 => Command::UpdateNote,
            4 => Command::DeleteNote,
            5 => Command::ChangePassword,
            _ => Command::Logout,
        }
    }
}