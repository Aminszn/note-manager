// src/domains/note/mod.rs

pub mod builder;
pub mod commands;
pub mod dto;
pub mod entity;
pub mod repo;
pub mod service;

pub use builder::NoteBuilder;
pub use commands::{
    CreateNoteCommand,
    DeleteNoteCommand,
    GetNoteCommand,
    GetUserNotesCommand,
    UpdateNoteCommand,
};
pub use dto::NoteDto;
pub use entity::Note;
pub use repo::NoteRepository;
pub use service::NoteService;