use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::models::{AccountType, ApiRequest, Endpoint};

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InterTransferRequest {
    pub transfer_id: String,
    pub coin: String,
    pub amount: String,
    pub from_account_type: AccountType,
    pub to_account_type: AccountType,
}

impl ApiRequest for InterTransferRequest {
    type ApiResponse = InterTransferResponse;

    fn endpoint() -> Endpoint {
        Endpoint {
            path: "/v5/asset/transfer/inter-transfer".to_string(),
            method: Method::POST,
            auth: true,
        }
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InterTransferResponse {
    pub transfer_id: String,
    pub status: String,
}
