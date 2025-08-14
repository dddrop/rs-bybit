use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::models::{ApiRequest, Category, Endpoint};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLeverageRequest {
    pub category: Category,
    pub symbol: String,
    pub buy_leverage: String,
    pub sell_leverage: String,
}

impl ApiRequest for SetLeverageRequest {
    type ApiResponse = SetLeverageResponse;

    fn endpoint() -> Endpoint {
        Endpoint {
            path: "/v5/position/set-leverage".to_string(),
            method: Method::POST,
            auth: true,
        }
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetLeverageResponse {}
