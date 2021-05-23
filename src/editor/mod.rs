mod api;
pub mod command;
mod command_input;
pub mod cursor;
mod cursor_pos;
mod edit_schedule;
mod error;
mod mode;
mod render;
pub mod state;
pub mod text_capture;

pub use api::EditorLike;
pub use error::Error;
pub use mode::Mode;
pub use render::Draw;
pub use state::State;

pub type Result<T> = std::result::Result<T, Error>;
