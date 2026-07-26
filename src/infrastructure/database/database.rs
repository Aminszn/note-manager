// src/infrastructure/database/database.rs

use rusqlite::{Connection, Result};

pub struct Database {
    connection: Connection,
}

impl Database {
    /// Opens (or creates) the SQLite database.
    pub fn new(path: &str) -> Result<Self> {
        let connection = Connection::open(path)?;

        Ok(Self { connection })
    }

    /// Creates all database tables.
    pub fn initialize(&self) -> Result<()> {
        self.connection.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL UNIQUE,
                password_hash TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            
            CREATE TABLE IF NOT EXISTS notes (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,

                FOREIGN KEY (user_id) REFERENCES users(id)
                    ON DELETE CASCADE
            );
            ",
        )?;

        Ok(())
    }

    /// Returns a reference to the SQLite connection.
    pub fn connection(&self) -> &Connection {
        &self.connection
    }
}