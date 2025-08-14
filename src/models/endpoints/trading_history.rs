use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::models::*;

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TradingHistoryRequest {
    pub category: Category,

    pub symbol: Option<String>,
    pub order_id: Option<String>,
    pub order_link_id: Option<String>,
    pub base_coin: Option<String>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub exec_type: Option<ExecType>,
    pub limit: Option<u8>,
    pub cursor: Option<String>,
}

impl ApiRequest for TradingHistoryRequest {
    type ApiResponse = TradingHistoryResponse;

    fn endpoint() -> Endpoint {
        Endpoint {
            path: "/v5/execution/list".to_string(),
            method: Method::GET,
            auth: true,
        }
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TradingHistoryResponse {
    pub category: Category,
    pub list: Vec<Execution>,
    pub next_page_cursor: String,
}
