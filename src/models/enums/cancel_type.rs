// *************************************************************************************************
// Ref. https://bybit-exchange.github.io/docs/v5/enum#canceltype
// *************************************************************************************************

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CancelType {
    #[default]
    #[serde(rename = "UNKNOWN")]
    Unknown,
    CancelByUser,
    CancelByReduceOnly,
    CancelByPrepareLiq,
    CancelByPrepareAdl,
    CancelByAdmin,
    CancelBySettle,
    CancelByTpSlTsClear,
    CancelBySmp,
}
