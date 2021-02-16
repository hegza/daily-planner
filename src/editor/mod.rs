mod api;
pub mod command;
pub mod cursor;
mod cursor_pos;
pub mod editor;
mod error;
mod render;

pub use api::EditorLike;
pub use editor::Editor;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;
