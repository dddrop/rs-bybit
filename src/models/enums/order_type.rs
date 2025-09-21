// *************************************************************************************************
// Ref. https://bybit-exchange.github.io/docs/v5/enum#ordertype
// *************************************************************************************************

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum OrderType {
    #[default]
    Market,
    Limit,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}
