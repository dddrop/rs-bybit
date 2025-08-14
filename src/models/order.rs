use serde::{Deserialize, Serialize};

use crate::{
    models::{
        CancelType, CreateType, OrderStatus, OrderType, RejectReason, SmpType, StopOrderType,
        TimeInForce, TriggerBy,
    },
    utilities::deserialize_empty_object_as_none,
};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub order_id: String,
    pub order_link_id: String,
    pub block_trade_id: String,
    pub symbol: String,
    pub price: String,
    pub qty: String,
    pub side: String,
    pub is_leverage: String,
    pub position_idx: i32,
    pub order_status: OrderStatus,
    #[serde(deserialize_with = "deserialize_empty_object_as_none")]
    pub create_type: Option<CreateType>,
    #[serde(deserialize_with = "deserialize_empty_object_as_none")]
    pub cancel_type: Option<CancelType>,
    #[serde(deserialize_with = "deserialize_empty_object_as_none")]
    pub reject_reason: Option<RejectReason>,
    pub avg_price: String,
    pub leaves_qty: String,
    pub leaves_value: String,
    pub cum_exec_qty: String,
    pub cum_exec_value: String,
    pub cum_exec_fee: String,
    #[serde(deserialize_with = "deserialize_empty_object_as_none")]
    pub time_in_force: Option<TimeInForce>,
    #[serde(deserialize_with = "deserialize_empty_object_as_none")]
    pub order_type: Option<OrderType>,
    #[serde(deserialize_with = "deserialize_empty_object_as_none")]
    pub stop_order_type: Option<StopOrderType>,
    pub order_iv: String,
    pub market_unit: String,
    pub trigger_price: String,
    pub take_profit: String,
    pub stop_loss: String,
    pub tpsl_mode: String,
    pub oco_trigger_by: Option<String>,
    pub tp_limit_price: String,
    pub sl_limit_price: String,
    #[serde(deserialize_with = "deserialize_empty_object_as_none")]
    pub tp_trigger_by: Option<TriggerBy>,
    #[serde(deserialize_with = "deserialize_empty_object_as_none")]
    pub sl_trigger_by: Option<TriggerBy>,
    pub trigger_direction: i32,
    #[serde(deserialize_with = "deserialize_empty_object_as_none")]
    pub trigger_by: Option<TriggerBy>,
    pub last_price_on_created: String,
    pub base_price: Option<String>,
    pub reduce_only: bool,
    pub close_on_trigger: bool,
    pub place_type: String,
    #[serde(deserialize_with = "deserialize_empty_object_as_none")]
    pub smp_type: Option<SmpType>,
    pub smp_group: i32,
    pub smp_order_id: String,
    pub created_time: String,
    pub updated_time: String,
    // Web Socket Only
    pub category: Option<String>,
    pub fee_currency: Option<String>,
    pub closed_pnl: Option<String>,
}
