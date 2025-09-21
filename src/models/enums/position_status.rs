// *************************************************************************************************
// Ref. https://bybit-exchange.github.io/docs/v5/enum#positionstatus
// *************************************************************************************************

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default, Clone)]
pub enum PositionStatus {
    #[default]
    Normal,
    Liq,
    Adl,
}
