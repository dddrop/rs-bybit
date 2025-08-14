// *************************************************************************************************
// Ref. https://bybit-exchange.github.io/docs/v5/enum#category
// *************************************************************************************************

use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub enum Category {
    Spot,
    #[default]
    Linear,
    Inverse,
    Option,
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
