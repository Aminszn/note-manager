// src/domains/note/service.rs

use uuid::Uuid;

use super::{
    builder::NoteBuilder,
    dto::NoteDto,
    repo::NoteRepository,
};

pub struct NoteService<R: NoteRepository> {
    repository: R,
}

impl<R: NoteRepository> NoteService<R> {
    /// Creates a new NoteService.
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Creates and persists a new note.
    pub fn create_note(
        &self,
        user_id: Uuid,
        title: String,
        content: String,
    ) -> Result<NoteDto, String> {
        let note = NoteBuilder::new()
            .user_id(user_id)
            .title(title)
            .content(content)
            .build()?;

        self.repository.create(&note)?;

        Ok(note.into())
    }

    /// Retrieves a note by its ID.
    pub fn get_note(
        &self,
        id: Uuid,
    ) -> Result<Option<NoteDto>, String> {
        let note = self.repository.find_by_id(id)?;

        Ok(note.map(NoteDto::from))
    }

    /// Retrieves all notes for a specific user.
    pub fn get_notes_by_user(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<NoteDto>, String> {
        let notes = self.repository.find_by_user_id(user_id)?;

        Ok(notes.into_iter().map(NoteDto::from).collect())
    }

    /// Retrieves every note.
    pub fn get_all_notes(&self) -> Result<Vec<NoteDto>, String> {
        let notes = self.repository.find_all()?;

        Ok(notes.into_iter().map(NoteDto::from).collect())
    }

    /// Updates an existing note.
    pub fn update_note(
        &self,
        id: Uuid,
        title: String,
        content: String,
    ) -> Result<NoteDto, String> {
        let mut note = self
            .repository
            .find_by_id(id)?
            .ok_or("Note not found")?;

        note.change_title(title);
        note.change_content(content);

        self.repository.update(&note)?;

        Ok(note.into())
    }

    /// Deletes a note by its ID.
    pub fn delete_note(&self, id: Uuid) -> Result<(), String> {
        self.repository.delete(id)
    }
}