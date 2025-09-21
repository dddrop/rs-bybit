// *************************************************************************************************
// Ref. https://bybit-exchange.github.io/docs/v5/enum#accounttype
// *************************************************************************************************

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum AccountType {
    #[default]
    Unified,
    Fund,
    Contract,
    Spot,
}
