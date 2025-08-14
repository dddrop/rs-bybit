use serde::{Deserialize, Serialize};

use crate::models::*;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FastExecution {
    pub category: Category,
    pub symbol: String,
    pub order_id: String,
    pub is_maker: bool,
    pub order_link_id: String,
    pub exec_id: String,
    pub exec_price: String,
    pub exec_qty: String,
    pub side: String,
    pub exec_time: String,
    pub seq: u64,
}
