use crate::prelude::*;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    pub id: i32,
    pub desc: Option<String>,
    pub weight: Option<String>,
    pub width: Option<i32>,
    pub length: Option<i32>,
    pub height: Option<i32>,
    pub guide_num: String,
    pub shipment_dt: String,
    pub destination_address: String,
}

#[derive(Debug, Default)]
pub struct State {
    pub packages: Vec<Package>,
    pub package_table_state: TableState,
}