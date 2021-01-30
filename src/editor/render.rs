mod schedule;
mod status_bar;

use std::io::Stdout;

use super::Result;

pub trait Render {
    fn render(&self, stdout: &mut Stdout) -> Result<()>;
}
