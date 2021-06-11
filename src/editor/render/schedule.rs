use super::Render;
use crate::{
    dom::{timebox::AdjustPolicy, TimeBox, TimeSlotKind},
    schedule::Schedule,
};
use chrono::NaiveTime;
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
        let mut circ_sector = if self.sunrise.is_some() && self.sunset.is_some() {
            CircadianSector::PreDawn
        } else {
            CircadianSector::Day
        };
        stdout.queue(style::SetForegroundColor(circ_sector.color()))?;

        for time_box in self.timeboxes.iter() {
            if self.sunrise.is_some() && self.sunset.is_some() {
                let sunrise = self.sunrise.expect("logic error");
                let sunset = self.sunset.expect("logic error");

                if let Some(time) = &time_box.time {
                    let time = match time {
                        TimeSlotKind::Time(t) => t,
                        TimeSlotKind::Span(t, _) => t,
                    };
                    let time = NaiveTime::from_hms(time.hour as u32, time.min as u32, 0);
                    let next_color = match circ_sector {
                        CircadianSector::PreDawn => time >= sunrise,
                        CircadianSector::Day => time >= sunset,
                        CircadianSector::Dusk => false,
                        CircadianSector::Night => false,
                    };
                    if next_color {
                        circ_sector = circ_sector.next();
                        stdout.queue(style::SetForegroundColor(circ_sector.color()))?;
                    }
                }
            }

            let t_str = match &time_box.time {
                Some(t) => format!("{}", t),
                None => "     ".to_owned(),
            };

            let fmt = format!("{{time:<{}}} {{activity}}", self.time_col_width(),);
            let mut vars = HashMap::new();
            vars.insert("time".to_owned(), t_str);
            vars.insert("activity".to_owned(), format!("{}", time_box.activity));
            let content = strfmt(&fmt, &vars)?;

            let (set_styles, unset_styles): (Vec<_>, Vec<_>) =
                time_box.resolve_styles().into_iter().unzip();

            for set in set_styles {
                stdout.queue(style::SetAttribute(set))?;
            }

            let styled = style(content);
            stdout
                .queue(style::PrintStyledContent(styled))?
                .queue(cursor::MoveToNextLine(1))?;

            for unset in unset_styles {
                stdout.queue(style::SetAttribute(unset))?;
            }
        }
        stdout.queue(style::SetForegroundColor(style::Color::Reset))?;
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

impl TimeBox {
    fn resolve_styles(&self) -> Vec<(style::Attribute, style::Attribute)> {
        let mut styles = vec![];

        // Cross out done items
        if self.done {
            styles.push((
                style::Attribute::CrossedOut,
                style::Attribute::NotCrossedOut,
            ));
        }

        // Cursive fixed items
        if self.adjust_policy == AdjustPolicy::Fixed {
            styles.push((style::Attribute::Bold, style::Attribute::NormalIntensity));
        }

        styles
    }
}

#[derive(Clone, PartialEq, Copy)]
enum CircadianSector {
    PreDawn,
    Day,
    Dusk,
    Night,
}

impl CircadianSector {
    fn next(&self) -> CircadianSector {
        match self {
            CircadianSector::PreDawn => CircadianSector::Day,
            CircadianSector::Day => CircadianSector::Dusk,
            CircadianSector::Dusk => CircadianSector::Night,
            CircadianSector::Night => CircadianSector::PreDawn,
        }
    }
    fn color(&self) -> style::Color {
        match self {
            CircadianSector::PreDawn => style::Color::Rgb {
                r: 235,
                g: 180,
                b: 180,
            },
            CircadianSector::Day => style::Color::Reset,
            CircadianSector::Dusk => style::Color::Rgb {
                r: 215,
                g: 180,
                b: 220,
            },
            CircadianSector::Night => style::Color::Rgb {
                r: 100,
                g: 100,
                b: 255,
            },
        }
    }
}
