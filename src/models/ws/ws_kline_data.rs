use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WSKlineData {
    pub timestamp: u64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
    pub turnover: String,
    pub interval: String,
    pub start: u64,
    pub end: u64,
    pub confirm: bool,
}
