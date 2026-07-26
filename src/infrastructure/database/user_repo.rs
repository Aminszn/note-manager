// src/infrastructure/database/user_repo.rs

use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result};
use uuid::Uuid;

use crate::domains::user::{User, UserRepository};

pub struct SqliteUserRepository<'a> {
    connection: &'a Connection,
}

impl<'a> SqliteUserRepository<'a> {
    pub fn new(connection: &'a Connection) -> Self {
        Self { connection }
    }
}

impl<'a> UserRepository for SqliteUserRepository<'a> {
    fn create(&self, user: &User) -> Result<(), String> {
        self.connection
            .execute(
                "
                INSERT INTO users (
                    id,
                    username,
                    password_hash,
                    created_at,
                    updated_at
                )
                VALUES (?1, ?2, ?3, ?4, ?5)
                ",
                params![
                    user.id().to_string(),
                    user.username(),
                    user.password_hash(),
                    user.created_at().to_rfc3339(),
                    user.updated_at().to_rfc3339(),
                ],
            )
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    fn find_by_id(&self, id: Uuid) -> Result<Option<User>, String> {
        let mut stmt = self
            .connection
            .prepare(
                "
                SELECT id, username, password_hash, created_at, updated_at
                FROM users
                WHERE id = ?1
                ",
            )
            .map_err(|e| e.to_string())?;

        let mut rows = stmt
            .query(params![id.to_string()])
            .map_err(|e| e.to_string())?;

        if let Some(row) = rows.next().map_err(|e| e.to_string())? {
            let user = User::new(
                Uuid::parse_str(&row.get::<_, String>(0).unwrap()).unwrap(),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
                DateTime::parse_from_rfc3339(&row.get::<_, String>(3).unwrap())
                    .unwrap()
                    .with_timezone(&Utc),
                DateTime::parse_from_rfc3339(&row.get::<_, String>(4).unwrap())
                    .unwrap()
                    .with_timezone(&Utc),
            );

            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    fn find_by_username(&self, username: &str) -> Result<Option<User>, String> {
        let mut stmt = self
            .connection
            .prepare(
                "
                SELECT id, username, password_hash, created_at, updated_at
                FROM users
                WHERE username = ?1
                ",
            )
            .map_err(|e| e.to_string())?;

        let mut rows = stmt
            .query(params![username])
            .map_err(|e| e.to_string())?;

        if let Some(row) = rows.next().map_err(|e| e.to_string())? {
            let user = User::new(
                Uuid::parse_str(&row.get::<_, String>(0).unwrap()).unwrap(),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
                DateTime::parse_from_rfc3339(&row.get::<_, String>(3).unwrap())
                    .unwrap()
                    .with_timezone(&Utc),
                DateTime::parse_from_rfc3339(&row.get::<_, String>(4).unwrap())
                    .unwrap()
                    .with_timezone(&Utc),
            );

            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    fn find_all(&self) -> Result<Vec<User>, String> {
        let mut stmt = self
            .connection
            .prepare(
                "
                SELECT id, username, password_hash, created_at, updated_at
                FROM users
                ",
            )
            .map_err(|e| e.to_string())?;

        let users = stmt
            .query_map([], |row| {
                Ok(User::new(
                    Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                    row.get(1)?,
                    row.get(2)?,
                    DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)
                        .unwrap()
                        .with_timezone(&Utc),
                    DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                        .unwrap()
                        .with_timezone(&Utc),
                ))
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;

        Ok(users)
    }

    fn update(&self, user: &User) -> Result<(), String> {
        self.connection
            .execute(
                "
                UPDATE users
                SET
                    username = ?1,
                    password_hash = ?2,
                    updated_at = ?3
                WHERE id = ?4
                ",
                params![
                    user.username(),
                    user.password_hash(),
                    user.updated_at().to_rfc3339(),
                    user.id().to_string(),
                ],
            )
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    fn delete(&self, id: Uuid) -> Result<(), String> {
        self.connection
            .execute(
                "DELETE FROM users WHERE id = ?1",
                params![id.to_string()],
            )
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}