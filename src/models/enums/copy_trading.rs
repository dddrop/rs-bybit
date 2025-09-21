// *************************************************************************************************
// Ref. https://bybit-exchange.github.io/docs/zh-TW/v5/enum#copytrading
// *************************************************************************************************

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum CopyTrading {
    None,
    Both,
    UtaOnly,
    NormalOnly,
}

impl From<CopyTrading> for String {
    fn from(copy_trading: CopyTrading) -> Self {
        match copy_trading {
            CopyTrading::None => "None".to_string(),
            CopyTrading::Both => "Both".to_string(),
            CopyTrading::UtaOnly => "UtaOnly".to_string(),
            CopyTrading::NormalOnly => "NormalOnly".to_string(),
        }
    }
}
