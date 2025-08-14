use reqwest::Method;
use serde::Serialize;

use crate::models::{ApiRequest, Category, Endpoint, OrderFilter, OrderId};

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    pub category: Category,
    pub symbol: String,
    pub order_id: Option<String>,
    pub order_link_id: Option<String>,
    pub order_filter: Option<OrderFilter>,
}

impl ApiRequest for CancelOrderRequest {
    type ApiResponse = CancelOrderResponse;

    fn endpoint() -> Endpoint {
        Endpoint {
            path: "/v5/order/cancel".to_string(),
            method: Method::POST,
            auth: true,
        }
    }
}

pub type CancelOrderResponse = OrderId;
