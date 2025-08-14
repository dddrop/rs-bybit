// *************************************************************************************************
// Ref. https://bybit-exchange.github.io/docs/v5/enum#timeinforce
// *************************************************************************************************

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum TimeInForce {
    #[default]
    GTC,
    IOC,
    FOK,
    PostOnly,
    RPI,
}
