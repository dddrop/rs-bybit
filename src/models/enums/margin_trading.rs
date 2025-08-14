// *************************************************************************************************
// Ref. https://bybit-exchange.github.io/docs/zh-TW/v5/enum#margintrading
// *************************************************************************************************

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MarginTrading {
    None,
    Both,
    UtaOnly,
    NormalSpotOnly,
}
