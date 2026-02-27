use crate::model::input::*;

#[derive(Debug, Default)]
pub struct State {
    pub inputs: InputFields,
    pub action_sel: Option<u8>,
}