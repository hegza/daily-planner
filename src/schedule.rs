use crate::{activity::TimeBox, template::Template};

/// Main data structure
#[derive(Clone, Debug)]
pub struct Schedule(pub Vec<TimeBox>);
