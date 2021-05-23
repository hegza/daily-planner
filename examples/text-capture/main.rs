use backtrace::Backtrace;
use crossterm::{
    cursor,
    event::Event,
    terminal::{self, disable_raw_mode, enable_raw_mode},
    QueueableCommand,
};
use daily_planner::editor::{
    text_capture::{InputSource, TextCapture},
    Draw,
};
use std::{cell::RefCell, io::Stdout};
use std::{io, io::Write, rc::Rc};

fn main() -> crossterm::Result<()> {
    // Disable raw mode on error
    let on_error = |err| {
        disable_raw_mode().unwrap();

        let bt = Backtrace::new();
        eprintln!("{:?}", bt);
        eprintln!("{}", err);
    };
    event_loop().unwrap_or_else(on_error);

    Ok(())
}

// Capsulate to contain raw mode
fn event_loop() -> crossterm::Result<()> {
    let mut buffer = Rc::new(RefCell::<String>::new(String::new()));

    let mut cap_ext = TextCapture::capture(buffer, 0, InputSource::Stdio(None));
    let mut cap_int = TextCapture::owned(InputSource::Stdio(None));

    cap_ext.set_text("External buffer".to_string());
    cap_int.set_text("Internal buffer".to_string());

    let mut cursor_pos = 0;
    // Cursor has two positions: 0 and 1. This func switches it.
    let cur_move = |cur_pos: &mut _, mut stdout: Stdout| -> Result<(), crossterm::ErrorKind> {
        if *cur_pos == 0 {
            *cur_pos = 1;
        } else {
            *cur_pos = 0;
        }
        stdout.queue(cursor::MoveTo(0, *cur_pos))?.flush()?;
        Ok(())
    };

    let mut run = true;
    let mut editing = None;

    enable_raw_mode()?;
    // Clear screen and move cursor to top-left
    io::stdout()
        .queue(terminal::Clear(terminal::ClearType::All))?
        .queue(cursor::MoveTo(0, 0))?
        .flush()?;

    cap_int.draw_at(0, 0, io::stdout())?;
    cap_ext.draw_at(0, 1, io::stdout())?;
    format!(
        "Press {:?} to edit, {:?} / {:?} to exit",
        crossterm::event::KeyCode::Enter,
        crossterm::event::KeyCode::Esc,
        crossterm::event::KeyEvent {
            code: crossterm::event::KeyCode::Char('q'),
            modifiers: crossterm::event::KeyModifiers::CONTROL,
        }
    )
    .as_str()
    .draw_at(0, 2, io::stdout())?;

    while run {
        let input = crossterm::event::read()?;
        if let Event::Key(key) = input {
            if let Some(edit_idx) = editing {
                let buf = match edit_idx {
                    0 => &mut cap_ext,
                    1 => &mut cap_int,
                    _ => unreachable!(),
                };

                let (redraw, cur_delta) = buf.input(&key);
                /*if cur_delta > 0 {
                    io::stdout().queue(cursor::MoveRight)?;
                } else if cur_delta < 0 {
                    io::stdout().queue(cursor::MoveLeft)?;
                }*/
                if redraw {
                    io::stdout().flush()?;
                    buf.draw_at(0, edit_idx, io::stdout())?;
                }
            }
            use crossterm::event::KeyCode;
            match key.code {
                KeyCode::Up | KeyCode::Down => cur_move(&mut cursor_pos, io::stdout())?,
                KeyCode::Enter => {
                    // Enable edit mode if not editing
                    if editing.is_none() {
                        // Move cursor to end of line under edit
                        let y = cursor_pos;
                        let x = match cursor_pos {
                            0 => cap_int.cursor(),
                            1 => cap_ext.cursor(),
                            _ => unreachable!(),
                        } as u16;
                        io::stdout().queue(cursor::MoveTo(x, y))?.flush()?;
                        // Set as editing
                        editing = Some(cursor_pos);
                    }
                    // Disable edit mode if editing
                    else {
                        io::stdout().queue(cursor::MoveTo(0, cursor_pos))?.flush()?;
                        editing = None;
                    }
                }
                KeyCode::Esc | KeyCode::Char('q') => {
                    run = false;
                }
                _ => {}
            }
        }
    }
    disable_raw_mode()?;

    Ok(())
}
