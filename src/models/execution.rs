use serde::{Deserialize, Serialize};

use crate::{models::*, utilities::deserialize_string_or_bool};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Execution {
    pub category: Option<Category>,
    pub symbol: String,
    #[serde(deserialize_with = "deserialize_string_or_bool")]
    pub is_leverage: Option<bool>,
    pub order_id: String,
    pub order_link_id: String,
    pub side: String,
    pub order_price: String,
    pub order_qty: String,
    pub leaves_qty: String,
    pub create_type: CreateType,
    pub order_type: OrderType,
    pub stop_order_type: StopOrderType,
    pub exec_fee: String,
    pub exec_id: String,
    pub exec_price: String,
    pub exec_qty: String,
    pub exec_pnl: Option<String>,
    pub exec_type: ExecType,
    pub exec_value: String,
    pub exec_time: String,
    pub fee_currency: Option<String>,
    pub is_maker: bool,
    pub fee_rate: String,
    pub trade_iv: String,
    pub mark_iv: String,
    pub mark_price: String,
    pub index_price: String,
    pub underlying_price: String,
    pub block_trade_id: String,
    pub closed_size: String,
    pub seq: u64,
}
