use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::models::{ApiRequest, Endpoint};

#[derive(Debug, Deserialize, Serialize, Default)]
pub(crate) struct ServerTimeRequest {}

impl ApiRequest for ServerTimeRequest {
    type ApiResponse = ServerTimeResponse;

    fn endpoint() -> Endpoint {
        Endpoint {
            path: "/v5/market/time".to_string(),
            method: Method::GET,
            auth: false,
        }
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServerTimeResponse {
    pub time_second: String,
    pub time_nano: String,
}
