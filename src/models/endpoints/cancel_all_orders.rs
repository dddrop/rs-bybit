use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::models::*;

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrdersRequest {
    pub category: Category,
    pub symbol: Option<String>,
    pub base_coin: Option<String>,
    pub settle_coin: Option<String>,
    pub order_filter: Option<OrderFilter>,
    pub stop_order_type: Option<StopOrderType>,
}

impl ApiRequest for CancelAllOrdersRequest {
    type ApiResponse = CancelAllOrdersResponse;

    fn endpoint() -> Endpoint {
        Endpoint {
            path: "/v5/order/cancel-all".to_string(),
            method: Method::POST,
            auth: true,
        }
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrdersResponse {
    pub list: Vec<OrderId>,
    pub success: String,
}
