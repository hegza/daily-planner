use crossterm::event::KeyEvent;

use crate::keys::BINDINGS;

use super::{Mode, State};

#[macro_export]
macro_rules! bind {
    ($key_code:expr, $modifiers:expr, $cmd:expr, $filter:expr) => {
        Binding {
            key: KeyEvent {
                code: $key_code,
                modifiers: $modifiers,
            },
            command: $cmd,
            filter: $filter,
        }
    };
}

#[macro_export]
macro_rules! bind_key {
    ($key:expr, $cmd:expr, $filter:expr) => {
        bind!(KeyCode::Char($key), KeyModifiers::NONE, $cmd, $filter)
    };
}

#[derive(Debug, Clone)]
pub enum Command {
    Quit,
    MoveCursor(MoveCursor),
    InsertMode,
    CursorMode,
    TimeMode,
    GoToMode,
    DeleteMode,
    InsertTimeBoxBelow,
    InsertTimeBoxAbove,
    ToggleCrossOver,
    ToggleTimeAdjustPolicyFixed,
    ToggleBetweenSpanAndTime,
    PasteAbove,
    PasteBelow,
    MoveTimeCursor,
    SwapTimeSubMode,
    AdjustTime { hours: i8, minutes: i8 },
    DeleteTime,
    GoToColumn(ColumnKind),
    CutCurrentLine,
    OpenCommandInput,
    Multi(&'static [Command]),
}

#[derive(Debug, Clone)]
pub enum MoveCursor {
    Dir(Dir),
    Top,
    Bottom,
}

#[derive(Debug, Clone)]
pub enum ColumnKind {
    Index(usize),
    Last,
}

#[derive(Debug, Clone)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub enum Filter {
    Mode(Mode),
    Modes(&'static [Mode]),
    Global,
}

impl Filter {
    pub fn match_mode(&self, mode: &Mode) -> bool {
        match self {
            Filter::Global => true,
            Filter::Mode(m) => m == mode,
            Filter::Modes(ms) => ms.contains(mode),
        }
    }
}

#[derive(Debug)]
pub struct Binding {
    pub key: KeyEvent,
    pub command: Command,
    pub filter: Filter,
}

impl Binding {
    fn match_command(&self, key: &KeyEvent, mode: &Mode) -> Option<Command> {
        if self.filter.match_mode(mode) && key == &self.key {
            Some(self.command.clone())
        } else {
            None
        }
    }
}

impl Command {
    /// None -> Command::Noop.
    pub fn map(key_event: KeyEvent, editor: &State) -> Option<Command> {
        BINDINGS
            .iter()
            .find_map(|binding| binding.match_command(&key_event, &editor.mode.borrow()))
    }
}
