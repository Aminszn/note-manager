// src/cli/router.rs

use uuid::Uuid;

use crate::{
    auth::AuthService,
    domains::{
        note::{NoteRepository, NoteService},
        user::UserRepository,
    },
};

use super::{
    command::Command,
    input::CliInput,
    menu::Menu,
    output::CliOutput,
};

/// Routes CLI commands to the application services.
pub struct CliRouter<UR, NR>
where
    UR: UserRepository,
    NR: NoteRepository,
{
    auth_service: AuthService<UR>,
    note_service: NoteService<NR>,
}

impl<UR, NR> CliRouter<UR, NR>
where
    UR: UserRepository,
    NR: NoteRepository,
{
    /// Creates a new CLI router.
    pub fn new(
        auth_service: AuthService<UR>,
        note_service: NoteService<NR>,
    ) -> Self {
        Self {
            auth_service,
            note_service,
        }
    }

    /// Starts the CLI application.
    pub fn run(&mut self) {
        loop {
            let command = if self.auth_service.is_authenticated() {
                Menu::user_menu()
            } else {
                Menu::guest_menu()
            };

            if !self.execute(command) {
                break;
            }

            CliInput::pause();
        }
    }

    /// Executes a single command.
    fn execute(&mut self, command: Command) -> bool {
        match command {
            //---------------------------------
            // Guest Commands
            //---------------------------------

            Command::Register => self.register(),

            Command::Login => self.login(),

            Command::Exit => {
                CliOutput::success("Goodbye.");
                return false;
            }

            //---------------------------------
            // User Commands
            //---------------------------------

            Command::CreateNote => self.create_note(),

            Command::ViewNotes => self.view_notes(),

            Command::ViewNote => self.view_note(),

            Command::UpdateNote => self.update_note(),

            Command::DeleteNote => self.delete_note(),

            Command::ChangePassword => {
                self.change_password()
            }

            Command::Logout => self.logout(),
        }

        true
    }

    fn register(&mut self) {
        CliOutput::header("Register");

        let username = CliInput::username();
        let password = CliInput::password();

        match self
            .auth_service
            .register(username, password)
        {
            Ok(user) => {
                CliOutput::success(
                    "Registration successful.",
                );
                CliOutput::user(&user);
            }
            Err(error) => CliOutput::error(&error),
        }
    }

    fn login(&mut self) {
        CliOutput::header("Login");

        let username = CliInput::username();
        let password = CliInput::password();

        match self.auth_service.login(
            username,
            password,
        ) {
            Ok(user) => {
                CliOutput::success("Login successful.");
                CliOutput::user(&user);
            }
            Err(error) => CliOutput::error(&error),
        }
    }

    fn logout(&mut self) {
        self.auth_service.logout();

        CliOutput::success("Logged out.");
    }

    fn change_password(&mut self) {
        CliOutput::header("Change Password");

        let new_password =
            CliInput::new_password();

        let user_id = self
            .auth_service
            .current_user_id()
            .unwrap();

        match self.auth_service.change_password(
            user_id,
            new_password,
        ) {
            Ok(_) => CliOutput::success(
                "Password updated.",
            ),
            Err(error) => CliOutput::error(&error),
        }
    }

    fn create_note(&mut self) {
        CliOutput::header("Create Note");

        let title = CliInput::note_title();
        let content =
            CliInput::note_content();

        let user_id = self
            .auth_service
            .current_user_id()
            .unwrap();

        match self.note_service.create_note(
            user_id,
            title,
            content,
        ) {
            Ok(note) => {
                CliOutput::success(
                    "Note created.",
                );
                CliOutput::note(&note);
            }
            Err(error) => CliOutput::error(&error),
        }
    }

    fn view_notes(&self) {
        CliOutput::header("My Notes");

        let user_id = self
            .auth_service
            .current_user_id()
            .unwrap();

        match self
            .note_service
            .get_notes_by_user(user_id)
        {
            Ok(notes) => {
                CliOutput::notes(&notes)
            }
            Err(error) => CliOutput::error(&error),
        }
    }

    fn view_note(&self) {
        CliOutput::header("View Note");

        let id = CliInput::note_id();

        let id = match Uuid::parse_str(&id) {
            Ok(id) => id,
            Err(_) => {
                CliOutput::error(
                    "Invalid UUID.",
                );
                return;
            }
        };

        match self.note_service.get_note(id) {
            Ok(Some(note)) => {
                CliOutput::note(&note)
            }
            Ok(None) => {
                CliOutput::error(
                    "Note not found.",
                )
            }
            Err(error) => {
                CliOutput::error(&error)
            }
        }
    }

    fn update_note(&mut self) {
        CliOutput::header("Update Note");

        let id = CliInput::note_id();

        let id = match Uuid::parse_str(&id) {
            Ok(id) => id,
            Err(_) => {
                CliOutput::error(
                    "Invalid UUID.",
                );
                return;
            }
        };

        let title = CliInput::note_title();
        let content =
            CliInput::note_content();

        match self.note_service.update_note(
            id,
            title,
            content,
        ) {
            Ok(note) => {
                CliOutput::success(
                    "Note updated.",
                );
                CliOutput::note(&note);
            }
            Err(error) => {
                CliOutput::error(&error)
            }
        }
    }

    fn delete_note(&mut self) {
        CliOutput::header("Delete Note");

        let id = CliInput::note_id();

        let id = match Uuid::parse_str(&id) {
            Ok(id) => id,
            Err(_) => {
                CliOutput::error(
                    "Invalid UUID.",
                );
                return;
            }
        };

        match self.note_service.delete_note(id) {
            Ok(_) => CliOutput::success(
                "Note deleted.",
            ),
            Err(error) => {
                CliOutput::error(&error)
            }
        }
    }
}