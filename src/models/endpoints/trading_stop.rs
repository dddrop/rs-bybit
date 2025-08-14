// *************************************************************************************************
// Ref. https://bybit-exchange.github.io/docs/v5/position/trading-stop
// *************************************************************************************************

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::models::*;

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TradingStopRequest {
    pub category: Category,
    pub symbol: String,
    pub position_idx: PositionIdxInt,

    pub take_profit: Option<String>,
    pub stop_loss: Option<String>,
    pub trailing_stop: Option<String>,
    pub tp_trigger_by: Option<TriggerBy>,
    pub sl_trigger_by: Option<TriggerBy>,
    pub active_price: Option<String>,
    pub tpsl_mode: Option<TpslMode>,
    pub tp_size: Option<String>,
    pub sl_size: Option<String>,
    pub tp_limit_price: Option<String>,
    pub sl_limit_price: Option<String>,
    pub tp_order_type: Option<OrderType>,
    pub sl_order_type: Option<OrderType>,
}

impl ApiRequest for TradingStopRequest {
    type ApiResponse = TradingStopResponse;

    fn endpoint() -> Endpoint {
        Endpoint {
            path: "/v5/position/trading-stop".to_string(),
            method: Method::POST,
            auth: true,
        }
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TradingStopResponse {}
