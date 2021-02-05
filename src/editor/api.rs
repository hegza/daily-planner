use std::{io::stdout, panic};

use super::Editor;
use crate::schedule::Schedule;
use backtrace::Backtrace;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

pub trait EditorLike<Ed>
where
    Ed: Sized,
{
    fn spawn(schedule: Schedule) -> Result<Ed>;
    fn attach(&mut self);
}

impl EditorLike<Editor> for Editor {
    /// Creates the editor, attaches to stdout and sets raw mode
    fn spawn(schedule: Schedule) -> Result<Editor> {
        let stdout = stdout();

        // Enable raw mode and disable it on panic
        enable_raw_mode()?;
        panic::set_hook(Box::new(|panic_info| {
            disable_raw_mode().unwrap();

            let bt = Backtrace::new();
            eprintln!("{:?}", bt);
            eprintln!("{}", panic_info);
        }));

        let editor = Editor::with_stdout(stdout, schedule);

        Ok(editor)
    }

    fn attach(&mut self) {
        // Disable raw mode on error
        let on_error = |err| {
            disable_raw_mode().unwrap();

            let bt = Backtrace::new();
            eprintln!("{:?}", bt);
            eprintln!("{}", err);
        };

        self.run().unwrap_or_else(on_error)
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
    }
}
