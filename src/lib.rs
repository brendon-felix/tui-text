#![allow(
    dead_code,
    clippy::module_name_repetitions,
    clippy::cast_possible_truncation
)]
pub mod actions;
pub mod clipboard;
mod debug;
pub mod events;
mod helper;
mod state;
mod view;

pub use events::EditorEventHandler;
pub use state::{mode::EditorMode, EditorState};
pub use view::{theme::EditorTheme, EditorView};

/// A data structure that contains chars organized in rows and columns
pub type Lines = jagged::Jagged<char>;
pub use jagged::index::RowIndex;
pub use jagged::Index2;
