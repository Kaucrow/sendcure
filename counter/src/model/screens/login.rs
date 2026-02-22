use crate::model::input::*;

#[derive(Debug, Default)]
pub struct State {
    pub inputs: InputFields,
    pub input_mode: InputMode,
    pub failed_logins: u8,
    pub action_sel: Option<u8>,
}