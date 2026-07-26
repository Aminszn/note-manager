// src/domains/note/dto.rs

use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::entity::Note;

/// Data Transfer Object for Note.
#[derive(Debug, Clone)]
pub struct NoteDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Note> for NoteDto {
    fn from(note: Note) -> Self {
        Self {
            id: note.id(),
            user_id: note.user_id(),
            title: note.title().to_string(),
            content: note.content().to_string(),
            created_at: *note.created_at(),
            updated_at: *note.updated_at(),
        }
    }
}