use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    bind, bind_key,
    editor::{
        command::{Binding, ColumnKind, Command, Dir, Filter},
        editor::Mode,
    },
};

/*
    Globals:
    - ctrl + q: quit
    Cursor mode:
    - h, l, left arrow, right arrow: move cursor (and ghost) horizontal
    - j, k, down arrow, up arrow: move cursor vertical, account for horizontal ghost
    - i: insert mode
    - t: time mode
    - o: create line below and move in insert mode

    'g' modifier
        'g' move cursor to begin of file (first column)
        'G' move cursor to end of file (first column)
    'd' modifier 'delete'
        'd' delete row
    '/' search, on enter: move cursor to first match
    'ctrl + a' toggle item complete
*/

pub const BINDINGS: &[Binding] = &[
    // Global keys
    bind!(
        KeyCode::Char('q'),
        KeyModifiers::CONTROL,
        Command::Quit,
        Filter::Global
    ),
    // Cursor mode
    bind_key!(
        'h',
        Command::MoveCursor(Dir::Left),
        Filter::Mode(Mode::Cursor)
    ),
    bind_key!(
        'l',
        Command::MoveCursor(Dir::Right),
        Filter::Mode(Mode::Cursor)
    ),
    bind_key!(
        'j',
        Command::MoveCursor(Dir::Down),
        Filter::Mode(Mode::Cursor)
    ),
    bind_key!(
        'k',
        Command::MoveCursor(Dir::Up),
        Filter::Mode(Mode::Cursor)
    ),
    bind!(
        KeyCode::Down,
        KeyModifiers::NONE,
        Command::MoveCursor(Dir::Down),
        Filter::Mode(Mode::Cursor)
    ),
    bind!(
        KeyCode::Up,
        KeyModifiers::NONE,
        Command::MoveCursor(Dir::Up),
        Filter::Mode(Mode::Cursor)
    ),
    bind!(
        KeyCode::Left,
        KeyModifiers::NONE,
        Command::MoveCursor(Dir::Left),
        Filter::Mode(Mode::Cursor)
    ),
    bind!(
        KeyCode::Right,
        KeyModifiers::NONE,
        Command::MoveCursor(Dir::Right),
        Filter::Mode(Mode::Cursor)
    ),
    bind_key!('i', Command::InsertMode, Filter::Mode(Mode::Cursor)),
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
    // Time-mode
    bind_key!('i', Command::InsertMode, Filter::Mode(Mode::Time)),
    bind!(
        KeyCode::Esc,
        KeyModifiers::NONE,
        Command::CursorMode,
        Filter::Mode(Mode::Time)
    ),
    // Insert-mode
    bind!(
        KeyCode::Esc,
        KeyModifiers::NONE,
        Command::Multi(&[Command::MoveCursor(Dir::Left), Command::CursorMode]),
        Filter::Mode(Mode::Insert)
    ),
    bind!(
        KeyCode::Down,
        KeyModifiers::NONE,
        Command::MoveCursor(Dir::Down),
        Filter::Mode(Mode::Insert)
    ),
    bind!(
        KeyCode::Up,
        KeyModifiers::NONE,
        Command::MoveCursor(Dir::Up),
        Filter::Mode(Mode::Insert)
    ),
    bind!(
        KeyCode::Left,
        KeyModifiers::NONE,
        Command::MoveCursor(Dir::Left),
        Filter::Mode(Mode::Insert)
    ),
    bind!(
        KeyCode::Right,
        KeyModifiers::NONE,
        Command::MoveCursor(Dir::Right),
        Filter::Mode(Mode::Insert)
    ),
];
