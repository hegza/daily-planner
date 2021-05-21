#[derive(Clone, Debug, PartialEq)]
pub enum Mode {
    // Move cursor, use general commands
    Cursor,
    // Write content
    Insert,
    // Adjust time
    Time,
    // Go to something (transient)
    GoTo,
    // Delete something (transient)
    Delete,
}

impl Mode {
    pub fn is_transient(&self) -> bool {
        match self {
            Mode::Cursor => false,
            Mode::Insert => false,
            Mode::Time => false,
            Mode::GoTo => true,
            Mode::Delete => true,
        }
    }
}
