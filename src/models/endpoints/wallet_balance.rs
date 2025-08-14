use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::models::*;

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WalletBalanceRequest {
    pub account_type: AccountType,
    pub coin: Option<String>,
}

impl ApiRequest for WalletBalanceRequest {
    type ApiResponse = WalletBalanceResponse;

    fn endpoint() -> Endpoint {
        Endpoint {
            path: "/v5/account/wallet-balance".to_string(),
            method: Method::GET,
            auth: true,
        }
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WalletBalanceResponse {
    pub list: Vec<WalletBalance>,
}
