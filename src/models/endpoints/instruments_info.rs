use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::models::{
    ApiRequest, Endpoint,
    enums::{category::Category, status::Status},
    instrument_info::InstrumentInfo,
};

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentsInfoRequest {
    pub category: Category,
    pub symbol: Option<String>,
    pub status: Option<Status>,
    pub base_coin: Option<String>,
    pub quote_coin: Option<String>,
    pub limit: Option<u8>,
    pub cursor: Option<String>,
}

impl ApiRequest for InstrumentsInfoRequest {
    type ApiResponse = InstrumentsInfoResponse;

    fn endpoint() -> Endpoint {
        Endpoint {
            path: "/v5/market/instruments-info".to_string(),
            method: Method::GET,
            auth: false,
        }
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentsInfoResponse {
    pub category: Category,
    pub list: Vec<InstrumentInfo>,
    pub next_page_cursor: Option<String>,
}
