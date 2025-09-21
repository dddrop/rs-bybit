// *************************************************************************************************
// Ref. https://bybit-exchange.github.io/docs/v5/enum#stopordertype
// *************************************************************************************************

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum StopOrderType {
    TakeProfit,
    StopLoss,
    TrailingStop,
    Stop,
    PartialTakeProfit,
    PartialStopLoss,
    #[serde(rename = "tpslOrder")]
    TpslOrder,
    OcoOrder,
    MmRateClose,
    BidirectionalTpslOrder,
    #[default]
    #[serde(rename = "UNKNOWN")]
    UNKNOWN,
}
