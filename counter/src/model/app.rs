use crate::{
    prelude::*,
    model::{
        common::{TimeoutType, Timer},
        Screen,
    },
};

#[derive(Default)]
pub struct App {
    pub active_screen: Option<Screen>,
    pub timeout: HashMap<TimeoutType, Timer>,
    pub should_clear_screen: bool,
    pub should_quit: bool,
}