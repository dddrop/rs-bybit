use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::models::*;

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpenAndClosedOrdersRequest {
    pub category: Category,

    pub symbol: Option<String>,
    pub base_coin: Option<String>,
    pub settle_coin: Option<String>,
    pub order_id: Option<String>,
    pub order_link_id: Option<String>,
    pub open_only: Option<u8>,
    pub order_filter: Option<OrderFilter>,
    pub limit: Option<u8>,
    pub cursor: Option<String>,
}

impl ApiRequest for OpenAndClosedOrdersRequest {
    type ApiResponse = OpenAndClosedOrdersResponse;

    fn endpoint() -> Endpoint {
        Endpoint {
            path: "/v5/order/realtime".to_string(),
            method: Method::GET,
            auth: true,
        }
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpenAndClosedOrdersResponse {
    pub category: Category,
    pub next_page_cursor: Option<String>,
    pub list: Vec<Order>,
}
