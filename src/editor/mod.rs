mod api;
mod editor;
mod error;

pub use api::EditorLike;
pub use editor::Editor;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;
