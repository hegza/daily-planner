use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    bind, bind_key,
    editor::{
        command::{Binding, ColumnKind, Command, Dir, Filter, MoveCursor},
        editor::Mode,
    },
};

/*
    Modes:
    - Cursor
    - Insert
    - Time
    - GoTo
    - Delete

    Globals:
    - ctrl + q: quit

    Cursor mode:
    - h, l, left arrow, right arrow: move cursor (and ghost) horizontal
    - j, k, down arrow, up arrow: move cursor vertical, account for horizontal ghost
    - i: insert mode
    - a: move right and insert mode
    - t: time mode
    - o: create line below and move in insert mode
    - O: create line above and move in insert mode
    - I: go to first column and insert
    - A: go to last column and insert
    - 'g' modifier
        - (NYI) 'g' move cursor to begin of file (first column)
        - (NYI) 'G' move cursor to end of file (first column)
    - 'd' modifier 'delete'
        - d: delete line
    - p: paste clipboard

    Time mode:
    - i: insert mode
    - h, l, left arrow, right arrow: adjust time by 1 hour
    - j, l, up arrow, down arrow: adjust time by 15 minutes
    - r: toggle between relative and absolute time mode
    - f: toggle fixed time adjust policy
    - Esc: cursor mode

    Insert mode:
    - Esc: cursor mode
    - Arrow keys: move

    TODO:

    - '/' search, on enter: move cursor to first match
    - 'ctrl + a' toggle item complete
*/

pub const BINDINGS: &[Binding] = &[
    // Global keys
    bind!(
        KeyCode::Char('q'),
        KeyModifiers::CONTROL,
        Command::Quit,
        Filter::Global
    ),
    bind!(
        KeyCode::Char('a'),
        KeyModifiers::CONTROL,
        Command::ToggleCrossOver,
        Filter::Global
    ),
    // Cursor mode
    bind_key!(
        'h',
        Command::MoveCursor(MoveCursor::Dir(Dir::Left)),
        Filter::Mode(Mode::Cursor)
    ),
    bind_key!(
        'l',
        Command::MoveCursor(MoveCursor::Dir(Dir::Right)),
        Filter::Mode(Mode::Cursor)
    ),
    bind_key!(
        'j',
        Command::MoveCursor(MoveCursor::Dir(Dir::Down)),
        Filter::Mode(Mode::Cursor)
    ),
    bind_key!(
        'k',
        Command::MoveCursor(MoveCursor::Dir(Dir::Up)),
        Filter::Mode(Mode::Cursor)
    ),
    bind_key!('i', Command::InsertMode, Filter::Mode(Mode::Cursor)),
    bind_key!(
        'a',
        Command::Multi(&[
            Command::InsertMode,
            Command::MoveCursor(MoveCursor::Dir(Dir::Right))
        ]),
        Filter::Mode(Mode::Cursor)
    ),
    bind_key!('t', Command::TimeMode, Filter::Mode(Mode::Cursor)),
    bind_key!(
        'o',
        Command::Multi(&[Command::InsertTimeBoxBelow, Command::InsertMode]),
        Filter::Mode(Mode::Cursor)
    ),
    bind!(
        KeyCode::Char('O'),
        KeyModifiers::SHIFT,
        Command::Multi(&[Command::InsertTimeBoxAbove, Command::InsertMode]),
        Filter::Mode(Mode::Cursor)
    ),
    bind!(
        KeyCode::Char('I'),
        KeyModifiers::SHIFT,
        Command::Multi(&[
            Command::GoToColumn(ColumnKind::Index(0)),
            Command::InsertMode
        ]),
        Filter::Mode(Mode::Cursor)
    ),
    bind!(
        KeyCode::Char('A'),
        KeyModifiers::SHIFT,
        Command::Multi(&[Command::GoToColumn(ColumnKind::Last), Command::InsertMode]),
        Filter::Mode(Mode::Cursor)
    ),
    bind!(
        KeyCode::Char('g'),
        KeyModifiers::NONE,
        Command::GoToMode,
        Filter::Mode(Mode::Cursor)
    ),
    bind!(
        KeyCode::Char('d'),
        KeyModifiers::NONE,
        Command::DeleteMode,
        Filter::Mode(Mode::Cursor)
    ),
    bind_key!(
        'p',
        Command::Multi(&[
            Command::PasteBelow,
            Command::MoveCursor(MoveCursor::Dir(Dir::Down))
        ]),
        Filter::Mode(Mode::Cursor)
    ),
    bind!(
        KeyCode::Char('P'),
        KeyModifiers::SHIFT,
        Command::PasteAbove,
        Filter::Mode(Mode::Cursor)
    ),
    /*bind!(
        KeyCode::Char(':'),
        KeyModifiers::NONE,
        Command::OpenCommandInput,
        Filter::Mode(Mode::Cursor)
    ),*/
    // Time-mode
    bind_key!('i', Command::InsertMode, Filter::Mode(Mode::Time)),
    bind_key!('t', Command::MoveTimeCursor, Filter::Mode(Mode::Time)),
    bind_key!(
        'h',
        Command::AdjustTime {
            hours: -1,
            minutes: 0
        },
        Filter::Mode(Mode::Time)
    ),
    bind_key!(
        'l',
        Command::AdjustTime {
            hours: 1,
            minutes: 0
        },
        Filter::Mode(Mode::Time)
    ),
    bind_key!(
        'j',
        Command::AdjustTime {
            hours: 0,
            minutes: 15
        },
        Filter::Mode(Mode::Time)
    ),
    bind_key!(
        'k',
        Command::AdjustTime {
            hours: 0,
            minutes: -15
        },
        Filter::Mode(Mode::Time)
    ),
    bind!(
        KeyCode::Esc,
        KeyModifiers::NONE,
        Command::CursorMode,
        Filter::Mode(Mode::Time)
    ),
    bind_key!(
        'f',
        Command::ToggleTimeAdjustPolicyFixed,
        Filter::Mode(Mode::Time)
    ),
    bind_key!(
        's',
        Command::ToggleBetweenSpanAndTime,
        Filter::Mode(Mode::Time)
    ),
    bind_key!('r', Command::SwapTimeSubMode, Filter::Mode(Mode::Time)),
    // Insert-mode
    bind!(
        KeyCode::Esc,
        KeyModifiers::NONE,
        Command::Multi(&[
            Command::MoveCursor(MoveCursor::Dir(Dir::Left)),
            Command::CursorMode
        ]),
        Filter::Mode(Mode::Insert)
    ),
    // GoTo mode
    /*
    bind!(
        KeyCode::Char('g'),
        KeyModifiers::NONE,
        Command::MoveCursor(MoveCursor::Top),
        Filter::Mode(Mode::GoTo)
    ),
    bind!(
        KeyCode::Char('G'),
        KeyModifiers::SHIFT,
        Command::MoveCursor(MoveCursor::Bottom),
        Filter::Mode(Mode::GoTo)
    ),*/
    // Delete mode
    bind!(
        KeyCode::Char('d'),
        KeyModifiers::NONE,
        Command::CutCurrentLine,
        Filter::Mode(Mode::Delete)
    ),
    bind_key!('t', Command::DeleteTime, Filter::Mode(Mode::Delete)),
    // Multiple modes
    bind!(
        KeyCode::Down,
        KeyModifiers::NONE,
        Command::MoveCursor(MoveCursor::Dir(Dir::Down)),
        Filter::Modes(&[Mode::Insert, Mode::Cursor])
    ),
    bind!(
        KeyCode::Up,
        KeyModifiers::NONE,
        Command::MoveCursor(MoveCursor::Dir(Dir::Up)),
        Filter::Modes(&[Mode::Insert, Mode::Cursor])
    ),
    bind!(
        KeyCode::Left,
        KeyModifiers::NONE,
        Command::MoveCursor(MoveCursor::Dir(Dir::Left)),
        Filter::Modes(&[Mode::Insert, Mode::Cursor])
    ),
    bind!(
        KeyCode::Right,
        KeyModifiers::NONE,
        Command::MoveCursor(MoveCursor::Dir(Dir::Right)),
        Filter::Modes(&[Mode::Insert, Mode::Cursor])
    ),
];
