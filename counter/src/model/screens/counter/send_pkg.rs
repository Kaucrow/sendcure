use crate::{
    prelude::*,
    model::{Popup, input::*},
};

#[derive(Debug, Serialize)]
pub struct SendPackage {
    pub sender_cid: u32,
    pub receiver_cid: u32,
    pub desc: Option<String>,
    pub weight: f64,
    pub width: u32,
    pub length: u32,
    pub height: u32,
    pub destination_address: String,
}

#[derive(Debug, Default)]
pub struct State {
    pub active_popup: Option<Popup>,
    pub inputs: InputFields,
    pub action_sel: Option<u8>,
}