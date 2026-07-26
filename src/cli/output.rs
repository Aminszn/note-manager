// src/cli/output.rs

use colored::Colorize;

use crate::domains::{
    note::NoteDto,
    user::UserDto,
};

/// Handles all CLI output.
pub struct CliOutput;

impl CliOutput {
    /// Prints a section header.
    pub fn header(title: &str) {
        println!();
        println!("{}", title.bold().blue());
        println!("{}", "-".repeat(title.len()).blue());
    }

    /// Prints a success message.
    pub fn success(message: &str) {
        println!("{} {}", "✓".green(), message.green());
    }

    /// Prints an error message.
    pub fn error(message: &str) {
        println!("{} {}", "✗".red(), message.red());
    }

    /// Prints a normal informational message.
    pub fn info(message: &str) {
        println!("{}", message);
    }

    /// Displays a user.
    pub fn user(user: &UserDto) {
        println!("ID         : {}", user.id);
        println!("Username   : {}", user.username);
        println!("Created At : {}", user.created_at);
        println!("Updated At : {}", user.updated_at);
    }

    /// Displays a note.
    pub fn note(note: &NoteDto) {
        println!("ID         : {}", note.id);
        println!("Title      : {}", note.title);
        println!("Content    : {}", note.content);
        println!("Created At : {}", note.created_at);
        println!("Updated At : {}", note.updated_at);
    }

    /// Displays multiple notes.
    pub fn notes(notes: &[NoteDto]) {
        if notes.is_empty() {
            println!("No notes found.");
            return;
        }

        for (index, note) in notes.iter().enumerate() {
            println!();
            println!("Note {}", index + 1);
            println!("--------");
            println!("ID      : {}", note.id);
            println!("Title   : {}", note.title);
            println!("Content : {}", note.content);
        }
    }
}