use crate::model::{Popup, input::*};

#[derive(Debug, Default)]
pub struct State {
    pub active_popup: Option<Popup>,
    pub inputs: InputFields,
    pub input_mode: InputMode,
    pub failed_logins: u8,
    pub action_sel: Option<u8>,
}