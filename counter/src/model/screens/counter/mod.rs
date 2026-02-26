pub mod recv_pkg;
pub mod send_pkg;

use crate::{
    prelude::*,
    model::{Popup, input::*}
};

/*#[derive(Debug)]
pub enum Tab {
    Received(recv_pkg::)
}*/

#[derive(Debug)]
pub struct Package {
    pub package_id: i32,
    pub package_desc: String,
    pub package_weight: f64,
    pub package_width: i32,
    pub package_length: i32,
    pub package_height: i32,
}

#[derive(Debug, Default)]
pub struct State {
    pub active_popup: Option<Popup>,
    pub inputs: InputFields,
    pub input_mode: InputMode,
    pub action_sel: Option<u8>,
    pub sidebar_state: ListState,
    pub client: Option<u32>,
    pub packages: Vec<Package>,
    pub package_table_state: TableState,
}