mod api;
pub mod command;
mod cursor;
pub mod editor;
mod error;
mod map_cursor;
mod render;

pub use api::EditorLike;
pub use editor::Editor;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;
