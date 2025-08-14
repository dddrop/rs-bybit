// *************************************************************************************************
// Ref. https://bybit-exchange.github.io/docs/v5/enum#triggerby
// *************************************************************************************************

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase")]
pub enum TriggerBy {
    #[default]
    LastPrice,
    IndexPrice,
    MarkPrice,
}
