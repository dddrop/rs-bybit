use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default, Clone)]
pub enum OrderFilter {
    #[default]
    Order,
    #[serde(rename = "tpslOrder")]
    TpslOrder,
    StopOrder,
    OcoOrder,
    BidirectionalTpslOrder,
}
