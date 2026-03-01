use crate::prelude::*;

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub shipment_id: u32,
    pub guide_num: String,
}