use crate::prelude::*;

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
    pub packages: Vec<Package>,
    pub package_table_state: TableState,
}