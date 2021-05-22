use std::{io::Stdout, panic};

use super::State;
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

impl EditorLike<State> for State {
    /// Creates the editor, attaches to stdout and sets raw mode
    fn try_from_schedule(schedule: Schedule, stdout: Stdout) -> Result<State> {
        // Enable raw mode and disable it on panic
        enable_raw_mode()?;
        panic::set_hook(Box::new(|panic_info| {
            disable_raw_mode().unwrap();

            let bt = Backtrace::new();
            eprintln!("{:?}", bt);
            eprintln!("{}", panic_info);
        }));

        let editor = State::with_stdout(stdout, schedule);

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

        self.run().unwrap_or_else(on_error);

        disable_raw_mode().unwrap();
    }
}

impl Drop for State {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
    }
}
