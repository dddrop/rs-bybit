use reqwest::Method;
use serde::Serialize;

use crate::models::*;

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderRequest {
    pub category: Category,
    pub symbol: String,

    pub order_id: Option<String>,
    pub order_link_id: Option<String>,
    pub order_iv: Option<String>,
    pub trigger_price: Option<String>,
    pub qty: Option<String>,
    pub price: Option<String>,
    pub tpsl_mode: Option<TpslMode>,
    pub take_profit: Option<String>,
    pub stop_loss: Option<String>,
    pub tp_trigger_by: Option<TriggerBy>,
    pub sl_trigger_by: Option<TriggerBy>,
    pub trigger_by: Option<TriggerBy>,
    pub tp_limit_price: Option<String>,
    pub sl_limit_price: Option<String>,
}

impl ApiRequest for AmendOrderRequest {
    type ApiResponse = AmendOrderResponse;

    fn endpoint() -> Endpoint {
        Endpoint {
            path: "/v5/order/amend".to_string(),
            method: Method::POST,
            auth: true,
        }
    }
}

pub type AmendOrderResponse = OrderId;
