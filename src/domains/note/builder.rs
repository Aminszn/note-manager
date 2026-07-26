// src/domains/note/builder.rs

use chrono::Utc;
use uuid::Uuid;

use crate::shared::validators::Validators;

use super::entity::Note;

/// Builder for constructing valid Note entities.
pub struct NoteBuilder {
    user_id: Option<Uuid>,
    title: Option<String>,
    content: Option<String>,
}

impl NoteBuilder {
    /// Creates a new NoteBuilder.
    pub fn new() -> Self {
        Self {
            user_id: None,
            title: None,
            content: None,
        }
    }

    /// Sets the owner of the note.
    pub fn user_id(mut self, user_id: Uuid) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// Sets the note title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the note content.
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Builds a fully initialized Note.
    pub fn build(self) -> Result<Note, String> {
        let now = Utc::now();

        let user_id = self.user_id.ok_or("User ID is required.")?;

        let title = self.title.ok_or("Title is required.")?;
        Validators::validate_note_title(&title)?;

        let content = self.content.ok_or("Content is required.")?;
        Validators::validate_note_content(&content)?;

        Ok(Note::new(
            Uuid::new_v4(),
            user_id,
            title.trim().to_string(),
            content.trim().to_string(),
            now,
            now,
        ))
    }
}

impl Default for NoteBuilder {
    fn default() -> Self {
        Self::new()
    }
}