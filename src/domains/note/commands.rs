// src/domains/note/commands.rs

use uuid::Uuid;

/// Command for creating a note.
#[derive(Debug)]
pub struct CreateNoteCommand {
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
}

/// Command for retrieving a note by ID.
#[derive(Debug)]
pub struct GetNoteCommand {
    pub id: Uuid,
}

/// Command for retrieving all notes belonging to a user.
#[derive(Debug)]
pub struct GetUserNotesCommand {
    pub user_id: Uuid,
}

/// Command for updating an existing note.
#[derive(Debug)]
pub struct UpdateNoteCommand {
    pub id: Uuid,
    pub title: String,
    pub content: String,
}

/// Command for deleting a note.
#[derive(Debug)]
pub struct DeleteNoteCommand {
    pub id: Uuid,
}