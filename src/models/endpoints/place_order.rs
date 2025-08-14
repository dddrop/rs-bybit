use reqwest::Method;
use serde::Serialize;

use crate::models::{
    ApiRequest, Category, Endpoint, OrderFilter, OrderId, OrderType, PositionIdxString, Side,
    SmpType, TimeInForce, TpslMode, TriggerBy, TriggerDirection,
};

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRequest {
    pub category: Category,
    pub symbol: String,
    pub side: Side,
    pub order_type: OrderType,
    pub qty: String,

    pub is_leverage: Option<bool>,
    pub market_unit: Option<String>,
    pub slippage_to_tolerance_type: Option<String>,
    pub slippage_tolerance: Option<String>,
    pub price: Option<String>,
    pub trigger_direction: Option<TriggerDirection>,
    pub order_filter: Option<OrderFilter>,
    pub trigger_price: Option<String>,
    pub trigger_by: Option<TriggerBy>,
    pub order_iv: Option<String>,
    pub time_in_force: Option<TimeInForce>,
    pub position_idx: Option<PositionIdxString>,
    pub order_link_id: Option<String>,
    pub take_profit: Option<String>,
    pub stop_loss: Option<String>,
    pub tp_trigger_by: Option<TriggerBy>,
    pub sl_trigger_by: Option<TriggerBy>,
    pub reduce_only: Option<bool>,
    pub close_on_trigger: Option<bool>,
    pub smp_type: Option<SmpType>,
    pub mmp: Option<bool>,
    pub tpsl_mode: Option<TpslMode>,
    pub tp_limit_price: Option<String>,
    pub sl_limit_price: Option<String>,
    pub tp_order_type: Option<OrderType>,
    pub sl_order_type: Option<OrderType>,
}

impl ApiRequest for PlaceOrderRequest {
    type ApiResponse = PlaceOrderResponse;

    fn endpoint() -> Endpoint {
        Endpoint {
            path: "/v5/order/create".to_string(),
            method: Method::POST,
            auth: true,
        }
    }
}

pub type PlaceOrderResponse = OrderId;
