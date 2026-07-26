// src/infrastructure/database/note_repo.rs

use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result};
use uuid::Uuid;

use crate::domains::note::{Note, NoteRepository};

pub struct SqliteNoteRepository<'a> {
    connection: &'a Connection,
}

impl<'a> SqliteNoteRepository<'a> {
    pub fn new(connection: &'a Connection) -> Self {
        Self { connection }
    }
}

impl<'a> NoteRepository for SqliteNoteRepository<'a> {
    fn create(&self, note: &Note) -> Result<(), String> {
        self.connection
            .execute(
                "
                INSERT INTO notes (
                    id,
                    user_id,
                    title,
                    content,
                    created_at,
                    updated_at
                )
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                ",
                params![
                    note.id().to_string(),
                    note.user_id().to_string(),
                    note.title(),
                    note.content(),
                    note.created_at().to_rfc3339(),
                    note.updated_at().to_rfc3339(),
                ],
            )
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    fn find_by_id(&self, id: Uuid) -> Result<Option<Note>, String> {
        let mut stmt = self
            .connection
            .prepare(
                "
                SELECT
                    id,
                    user_id,
                    title,
                    content,
                    created_at,
                    updated_at
                FROM notes
                WHERE id = ?1
                ",
            )
            .map_err(|e| e.to_string())?;

        let mut rows = stmt
            .query(params![id.to_string()])
            .map_err(|e| e.to_string())?;

        if let Some(row) = rows.next().map_err(|e| e.to_string())? {
            let note = Note::new(
                Uuid::parse_str(&row.get::<_, String>(0).unwrap()).unwrap(),
                Uuid::parse_str(&row.get::<_, String>(1).unwrap()).unwrap(),
                row.get(2).unwrap(),
                row.get(3).unwrap(),
                DateTime::parse_from_rfc3339(&row.get::<_, String>(4).unwrap())
                    .unwrap()
                    .with_timezone(&Utc),
                DateTime::parse_from_rfc3339(&row.get::<_, String>(5).unwrap())
                    .unwrap()
                    .with_timezone(&Utc),
            );

            Ok(Some(note))
        } else {
            Ok(None)
        }
    }

    fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Note>, String> {
        let mut stmt = self
            .connection
            .prepare(
                "
                SELECT
                    id,
                    user_id,
                    title,
                    content,
                    created_at,
                    updated_at
                FROM notes
                WHERE user_id = ?1
                ORDER BY updated_at DESC
                ",
            )
            .map_err(|e| e.to_string())?;

        let notes = stmt
            .query_map(params![user_id.to_string()], |row| {
                Ok(Note::new(
                    Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                    Uuid::parse_str(&row.get::<_, String>(1)?).unwrap(),
                    row.get(2)?,
                    row.get(3)?,
                    DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                        .unwrap()
                        .with_timezone(&Utc),
                    DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                        .unwrap()
                        .with_timezone(&Utc),
                ))
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;

        Ok(notes)
    }

    fn find_all(&self) -> Result<Vec<Note>, String> {
        let mut stmt = self
            .connection
            .prepare(
                "
                SELECT
                    id,
                    user_id,
                    title,
                    content,
                    created_at,
                    updated_at
                FROM notes
                ORDER BY updated_at DESC
                ",
            )
            .map_err(|e| e.to_string())?;

        let notes = stmt
            .query_map([], |row| {
                Ok(Note::new(
                    Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                    Uuid::parse_str(&row.get::<_, String>(1)?).unwrap(),
                    row.get(2)?,
                    row.get(3)?,
                    DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                        .unwrap()
                        .with_timezone(&Utc),
                    DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                        .unwrap()
                        .with_timezone(&Utc),
                ))
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;

        Ok(notes)
    }

    fn update(&self, note: &Note) -> Result<(), String> {
        self.connection
            .execute(
                "
                UPDATE notes
                SET
                    title = ?1,
                    content = ?2,
                    updated_at = ?3
                WHERE id = ?4
                ",
                params![
                    note.title(),
                    note.content(),
                    note.updated_at().to_rfc3339(),
                    note.id().to_string(),
                ],
            )
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    fn delete(&self, id: Uuid) -> Result<(), String> {
        self.connection
            .execute(
                "
                DELETE FROM notes
                WHERE id = ?1
                ",
                params![id.to_string()],
            )
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}