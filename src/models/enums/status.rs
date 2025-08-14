// *************************************************************************************************
// Ref. https://bybit-exchange.github.io/docs/v5/enum#status
// *************************************************************************************************

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase")]
pub enum Status {
    Closed,
    Delivering,
    PreLaunch,
    #[default]
    Trading,
}

impl From<Status> for String {
    fn from(status: Status) -> Self {
        match status {
            Status::Closed => "Closed".to_string(),
            Status::Delivering => "Delivering".to_string(),
            Status::PreLaunch => "PreLaunch".to_string(),
            Status::Trading => "Trading".to_string(),
        }
    }
}
