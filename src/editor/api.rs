use std::{io::Stdout, panic};

use crate::editor;
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
    fn try_from_schedule(schedule: Schedule, stdout: Stdout) -> Result<Ed>;
    fn attach(&mut self);
}

impl EditorLike<editor::State> for editor::State {
    /// Creates the editor, attaches to stdout and sets raw mode
    fn try_from_schedule(schedule: Schedule, stdout: Stdout) -> Result<editor::State> {
        // Enable raw mode and disable it on panic
        enable_raw_mode()?;
        panic::set_hook(Box::new(|panic_info| {
            let disable_result = disable_raw_mode();
            if disable_result.is_err() {
                eprintln!("could not disable raw mode");
            }

            let bt = Backtrace::new();
            eprintln!("{:?}", bt);
            eprintln!("{}", panic_info);
        }));

        let editor = editor::State::with_stdout(stdout, schedule);

        Ok(editor)
    }

    fn attach(&mut self) {
        // Disable raw mode on error
        let on_error = |err| {
            let disable_result = disable_raw_mode();
            if disable_result.is_err() {
                eprintln!("could not disable raw mode");
            }

            let bt = Backtrace::new();
            eprintln!("{:?}", bt);
            eprintln!("{}", err);
        };

        self.run().unwrap_or_else(on_error);

        let disable_result = disable_raw_mode();
        if disable_result.is_err() {
            eprintln!("could not disable raw mode");
        }
    }
}

impl Drop for editor::State {
    fn drop(&mut self) {
        let disable_result = disable_raw_mode();
        if disable_result.is_err() {
            eprintln!("could not disable raw mode");
        }
    }
}
