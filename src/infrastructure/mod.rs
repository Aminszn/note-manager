// src/infrastructure/mod.rs

pub mod database;
pub mod storage;

pub use database::{
    Database,
    SqliteNoteRepository,
    SqliteUserRepository,
};