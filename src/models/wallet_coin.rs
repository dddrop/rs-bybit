use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WalletCoin {
    pub coin: String,
    pub equity: String,
    pub usd_value: String,
    pub wallet_balance: String,
    pub free: Option<String>,
    pub locked: String,
    pub spot_hedging_qty: String,
    pub borrow_amount: String,
    pub available_to_withdraw: String,
    pub accrued_interest: String,
    #[serde(rename = "totalOrderIM")]
    pub total_order_im: String,
    #[serde(rename = "totalPositionIM")]
    pub total_position_im: String,
    #[serde(rename = "totalPositionMM")]
    pub total_position_mm: String,
    pub unrealised_pnl: String,
    pub cum_realised_pnl: String,
    pub bonus: String,
    pub margin_collateral: bool,
    pub collateral_switch: bool,
    pub available_to_borrow: String,
}
