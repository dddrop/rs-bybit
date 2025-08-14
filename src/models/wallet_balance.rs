use serde::{Deserialize, Serialize};

use crate::models::*;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WalletBalance {
    pub account_type: AccountType,
    #[serde(rename = "accountLTV")]
    pub account_ltv: String,
    #[serde(rename = "accountIMRate")]
    pub account_im_rate: String,
    #[serde(rename = "accountMMRate")]
    pub account_mm_rate: String,
    pub total_equity: String,
    pub total_wallet_balance: String,
    pub total_margin_balance: String,
    pub total_available_balance: String,
    #[serde(rename = "totalPerpUpl")]
    pub total_perp_upl: Option<String>,
    pub total_initial_margin: String,
    pub total_maintenance_margin: String,
    pub coin: Vec<WalletCoin>,
}
