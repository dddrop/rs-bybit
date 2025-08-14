use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::models::*;

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PositionInfoRequest {
    pub category: Category,

    pub symbol: Option<String>,
    pub base_coin: Option<String>,
    pub settle_coin: Option<String>,
    pub limit: Option<u8>,
    pub cursor: Option<String>,
}

impl ApiRequest for PositionInfoRequest {
    type ApiResponse = PositionInfoResponse;

    fn endpoint() -> Endpoint {
        Endpoint {
            path: "/v5/position/list".to_string(),
            method: Method::GET,
            auth: true,
        }
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PositionInfoResponse {
    pub category: Category,
    pub next_page_cursor: Option<String>,
    pub list: Vec<Position>,
}
