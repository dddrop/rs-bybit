// *************************************************************************************************
// Ref. https://bybit-exchange.github.io/docs/zh-TW/v5/enum#curauctionphase
// *************************************************************************************************

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum AuctionPhase {
    NotStarted,
    Finished,
    CallAuction,
    CallAuctionNoCancel,
    CrossMatching,
    ContinuousTrading,
}
