mod api;
pub mod command;
mod command_input;
pub mod cursor;
mod cursor_pos;
pub mod editor;
mod error;
mod render;
mod text_capture;

pub use api::EditorLike;
pub use editor::Editor;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;
