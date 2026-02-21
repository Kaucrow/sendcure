use crate::model::input::*;

#[derive(Debug, Default)]
pub struct State {
    pub input: InputFields,
    pub input_mode: InputMode,
    pub failed_logins: u8,
    pub action_sel: Option<u8>,
}