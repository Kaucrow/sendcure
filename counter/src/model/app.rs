use crate::{
    prelude::*,
    model::{
        common::{TimeoutType, Timer},
        Screen,
        Popup,
    },
};

#[derive(Default)]
pub struct App {
    pub active_screen: Screen,
    pub data: AppData,
}

#[derive(Default)]
pub struct AppData {
    pub timeout: HashMap<TimeoutType, Timer>,
    pub should_clear_screen: bool,
    pub should_quit: bool,
}