// src/domains/note/entity.rs

use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Domain entity representing a user's note.

#[derive(Debug, Clone)]
pub struct Note {
    id: Uuid,
    user_id: Uuid,
    title: String,
    content: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Note {

    /// Creates a fully initialized Note.
    
    pub fn new(
        id: Uuid,
        user_id: Uuid,
        title: String,
        content: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            user_id,
            title,
            content,
            created_at,
            updated_at,
        }
    }

    // Getters

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }


    /// Changes the note title.
    pub fn change_title(&mut self, title: String) {
        self.title = title.trim().to_string();
        self.touch();
    }

    /// Changes the note content.
    pub fn change_content(&mut self, content: String) {
        self.content = content.trim().to_string();
        self.touch();
    }

    /// Updates the modification timestamp.
    fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}