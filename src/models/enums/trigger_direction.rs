use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default, Clone)]
pub enum TriggerDirection {
    #[serde(rename = "1")]
    #[default]
    UpTo,
    #[serde(rename = "2")]
    DownTo,
}
