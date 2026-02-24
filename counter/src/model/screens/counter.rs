use crate::{
    prelude::*,
    model::{Popup, input::*}
};

#[derive(Debug, Default)]
pub struct State {
    pub active_popup: Option<Popup>,
    pub inputs: InputFields,
    pub input_mode: InputMode,
    pub action_sel: Option<u8>,
    pub sidebar_state: ListState,
}