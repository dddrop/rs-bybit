// *************************************************************************************************
// Ref. https://bybit-exchange.github.io/docs/v5/enum#smptype
// *************************************************************************************************

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase")]
pub enum SmpType {
    #[default]
    None,
    CancelMaker,
    CancelTaker,
    CancelBoth,
}
