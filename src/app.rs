// src/app.rs

use uuid::Uuid;

use crate::{
    auth::AuthService,
    cli::CliRouter,
    domains::{
        note::NoteService,
        user::UserService,
    },
    infrastructure::{
        Database,
        SqliteNoteRepository,
        SqliteUserRepository,
    },
};

pub fn run() {

    // Database

    let database =
        Database::new("notes.db").expect("Failed to open database");

    database
        .initialize()
        .expect("Failed to initialize database");

    // Repositories

    let user_repository =
        SqliteUserRepository::new(database.connection());

    let note_repository =
        SqliteNoteRepository::new(database.connection());

    // Services

    let user_service =
        UserService::new(user_repository);

    let note_service =
        NoteService::new(note_repository);

    let mut auth_service =
        AuthService::new(user_service);

    // CLI
    let mut router =
        CliRouter::new(auth_service, note_service);

    router.run();


}