use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OrderId {
    pub order_id: String,
    pub order_link_id: Option<String>,
}
