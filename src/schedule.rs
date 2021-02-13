use std::borrow::BorrowMut;

use crate::activity::TimeBox;

/// Main data structure
#[derive(Clone, Debug)]
pub struct Schedule(pub Vec<TimeBox>);
