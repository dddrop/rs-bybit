use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::models::*;

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BatchCancelOrdersRequest {
    pub category: Category,
    pub request: Vec<SymbolCancel>,
}

impl ApiRequest for BatchCancelOrdersRequest {
    type ApiResponse = BatchCancelOrdersResponse;

    fn endpoint() -> Endpoint {
        Endpoint {
            path: "/v5/order/cancel-batch".to_string(),
            method: Method::POST,
            auth: true,
        }
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BatchCancelOrdersResponse {
    pub list: Vec<OrderId>,
    pub success: String,
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SymbolCancel {
    pub symbol: String,
    pub order_id: Option<String>,
    pub order_link_id: Option<String>,
}
