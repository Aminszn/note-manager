// src/domains/note/repo.rs

use uuid::Uuid;

use super::entity::Note;

pub trait NoteRepository {
    /// Saves a new note.
    fn create(&self, note: &Note) -> Result<(), String>;

    /// Finds a note by its ID.
    fn find_by_id(&self, id: Uuid) -> Result<Option<Note>, String>;

    /// Returns all notes belonging to a user.
    fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Note>, String>;

    /// Returns every note in the system.
    fn find_all(&self) -> Result<Vec<Note>, String>;

    /// Updates an existing note.
    fn update(&self, note: &Note) -> Result<(), String>;

    /// Deletes a note.
    fn delete(&self, id: Uuid) -> Result<(), String>;
}