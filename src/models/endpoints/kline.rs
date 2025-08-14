use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::models::{
    ApiRequest, Endpoint,
    enums::{category::Category, interval::Interval},
    kline_data::KlineData,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KlineRequest {
    pub category: Category,
    pub symbol: String,
    pub interval: Interval,
    pub start: Option<i64>,
    pub end: Option<i64>,
    pub limit: Option<i32>,
}

impl Default for KlineRequest {
    fn default() -> Self {
        Self {
            category: Category::Linear,
            symbol: String::new(),
            interval: Interval::Min1,
            start: None,
            end: None,
            limit: Some(1000),
        }
    }
}

impl ApiRequest for KlineRequest {
    type ApiResponse = KlineResponse;

    fn endpoint() -> Endpoint {
        Endpoint {
            path: "/v5/market/kline".to_string(),
            method: Method::GET,
            auth: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KlineResponse {
    pub symbol: String,
    pub category: String,
    pub list: Vec<KlineData>,
}

impl Default for KlineResponse {
    fn default() -> Self {
        Self {
            symbol: String::new(),
            category: String::new(),
            list: Vec::new(),
        }
    }
}
