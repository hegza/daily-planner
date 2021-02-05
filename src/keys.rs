use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    bind, bind_key,
    editor::{
        command::{Binding, Command, Dir, Filter},
        editor::Mode,
    },
};

/*
    'h', 'l' move cursor (and ghost) horizontal
    'j', 'k' move cursor vertical, account for horizontal ghost
    'o' create new empty line below and move cursor to it in insert mode
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
        'j',
        Command::MoveCursor(Dir::Down),
        Filter::Mode(Mode::Cursor)
    ),
    bind_key!(
        'k',
        Command::MoveCursor(Dir::Up),
        Filter::Mode(Mode::Cursor)
    ),
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
    bind_key!('i', Command::InsertMode, Filter::Mode(Mode::Cursor)),
    bind_key!('t', Command::TimeMode, Filter::Mode(Mode::Cursor)),
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
    bind_key!(
        'o',
        Command::InsertTimeBoxBelowAndInsert,
        Filter::Mode(Mode::Cursor)
    ),
    bind!(
        KeyCode::Char('O'),
        KeyModifiers::SHIFT,
        Command::InsertTimeBoxAboveAndInsert,
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
        Command::MoveCursorLeftAndCursorMode,
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
