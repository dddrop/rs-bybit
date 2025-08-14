// *************************************************************************************************
// Ref. https://bybit-exchange.github.io/docs/v5/enum#orderstatus
// *************************************************************************************************

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum OrderStatus {
    #[default]
    New,
    PartiallyFilled,
    Untriggered,
    Rejected,
    PartiallyFilledCanceled,
    Filled,
    Cancelled,
    Triggered,
    Deactivated,
}
