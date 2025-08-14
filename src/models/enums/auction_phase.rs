// *************************************************************************************************
// Ref. https://bybit-exchange.github.io/docs/zh-TW/v5/enum#curauctionphase
// *************************************************************************************************

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum AuctionPhase {
    NotStarted,
    Finished,
    CallAuction,
    CallAuctionNoCancel,
    CrossMatching,
    ContinuousTrading,
}
