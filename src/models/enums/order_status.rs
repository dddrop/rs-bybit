// *************************************************************************************************
// Ref. https://bybit-exchange.github.io/docs/v5/enum#orderstatus
// *************************************************************************************************

use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default, Clone)]
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

impl Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_status_display() {
        assert_eq!(OrderStatus::New.to_string(), "New");
    }
}
