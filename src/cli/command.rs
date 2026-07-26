// src/cli/commands.rs

/// Represents every command the CLI application supports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {

  // Guest Commands

    Register,
    Login,
    Exit,

    // Authenticated Commands

    CreateNote,
    ViewNotes,
    ViewNote,
    UpdateNote,
    DeleteNote,

    ChangePassword,
    Logout,
}

impl Command {

    /// Returns whether this command requires an authenticated user.
    
    pub fn requires_authentication(&self) -> bool {
        matches!(
            self,
            Self::CreateNote
                | Self::ViewNotes
                | Self::ViewNote
                | Self::UpdateNote
                | Self::DeleteNote
                | Self::ChangePassword
                | Self::Logout
        )
    }
}