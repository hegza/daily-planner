use crate::{activity::TimeBox, template::Template};

#[derive(Clone, Debug)]
pub struct Schedule(pub Vec<TimeBox>);
