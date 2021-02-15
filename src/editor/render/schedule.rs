use super::Render;
use crate::schedule::Schedule;
use crossterm::{
    cursor,
    style::{self, style},
    QueueableCommand,
};
use std::{
    collections::HashMap,
    io::{Stdout, Write},
};
use strfmt::strfmt;

impl Render for Schedule {
    fn render(&self, stdout: &mut Stdout) -> crate::editor::Result<()> {
        for time_box in self.timeboxes.iter() {
            let t_str = match &time_box.time {
                Some(t) => format!("{}", t),
                None => "     ".to_owned(),
            };

            let fmt = format!("{{time:<{}}} {{activity}}", self.time_col_width(),);
            let mut vars = HashMap::new();
            vars.insert("time".to_owned(), t_str);
            vars.insert("activity".to_owned(), format!("{}", time_box.activity));
            let content = strfmt(&fmt, &vars)?;

            if time_box.done {
                stdout.queue(style::SetAttribute(style::Attribute::CrossedOut))?;
            }

            let styled = style(content);
            stdout
                .queue(style::PrintStyledContent(styled))?
                .queue(cursor::MoveToNextLine(1))?;

            if time_box.done {
                stdout.queue(style::SetAttribute(style::Attribute::NotCrossedOut))?;
            }
        }
        stdout.flush()?;

        Ok(())
    }
}

impl Schedule {
    pub fn time_col_width(&self) -> usize {
        self.timeboxes
            .iter()
            .filter_map(|x| x.time.as_ref())
            .map(|x| format!("{}", x).len())
            .max_by(|x, y| x.cmp(y))
            .unwrap_or(0)
    }
}
