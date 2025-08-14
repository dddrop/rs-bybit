// *************************************************************************************************
// Ref. https://bybit-exchange.github.io/docs/v5/enum#exectype
// *************************************************************************************************

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase")]
pub enum ExecType {
    Trade,
    AdlTrade,
    Funding,
    BustTrade,
    Delivery,
    Settle,
    BlockTrade,
    MovePosition,
    #[default]
    #[serde(rename = "UNKNOWN")]
    UNKNOWN,
}
