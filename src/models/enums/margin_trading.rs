// *************************************************************************************************
// Ref. https://bybit-exchange.github.io/docs/zh-TW/v5/enum#margintrading
// *************************************************************************************************

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum MarginTrading {
    None,
    Both,
    UtaOnly,
    NormalSpotOnly,
}
