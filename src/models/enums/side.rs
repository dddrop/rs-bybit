use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum Side {
    #[default]
    Buy,
    Sell,
}
