use std::fmt::Display;

use serde::{Deserialize, Deserializer, Serialize};

use crate::models::ws::ws_kline_data::WSKlineData;
use crate::models::{Execution, Order, Position, WalletBalance, ws::fast_execution::FastExecution};

#[derive(Debug, Serialize, PartialEq)]
pub enum Message {
    Operation(OperationMessage),
    Event(EventMessage),
}

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map = serde_json::Map::deserialize(deserializer)?;

        // Check if this is an operation message (contains "op" field)
        if map.contains_key("op") {
            let operation_msg = serde_json::from_value(serde_json::Value::Object(map))
                .map_err(serde::de::Error::custom)?;
            Ok(Message::Operation(operation_msg))
        } else {
            // Assume it's an event message (contains "topic" field)
            let event_msg = serde_json::from_value(serde_json::Value::Object(map))
                .map_err(serde::de::Error::custom)?;
            Ok(Message::Event(event_msg))
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::Operation(operation) => write!(f, "[WS][OP] {:?}", operation),
            Message::Event(event) => write!(f, "[WS][EVENT] {:?}", event),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct OperationMessage {
    pub op: String,
    pub conn_id: String,
    pub success: Option<bool>,
    pub ret_msg: Option<String>,
    pub req_id: Option<String>,
    pub args: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EventMessage {
    pub id: Option<String>,
    pub creation_time: Option<u64>,
    #[serde(rename = "type")]
    pub message_type: Option<String>,
    pub ts: Option<u64>,
    pub cs: Option<u64>,
    #[serde(flatten)]
    pub data: EventMessageData,
}

impl<'de> Deserialize<'de> for EventMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map = serde_json::Map::deserialize(deserializer)?;

        let topic = map
            .get("topic")
            .and_then(|v| v.as_str())
            .ok_or_else(|| serde::de::Error::custom("missing type field"))?;

        let data = match topic {
            execution if execution.starts_with("execution") => {
                let data = map
                    .get("data")
                    .ok_or_else(|| serde::de::Error::custom("missing data field"))?;
                serde_json::from_value(data.clone()).map(EventMessageData::Execution)
            }
            fast_execution if fast_execution.starts_with("fast_execution") => {
                let data = map
                    .get("data")
                    .ok_or_else(|| serde::de::Error::custom("missing data field"))?;
                serde_json::from_value(data.clone()).map(EventMessageData::FastExecution)
            }
            wallet if wallet.starts_with("wallet") => {
                let data = map
                    .get("data")
                    .ok_or_else(|| serde::de::Error::custom("missing data field"))?;
                serde_json::from_value(data.clone()).map(EventMessageData::Wallet)
            }
            order if order.starts_with("order") => {
                let data = map
                    .get("data")
                    .ok_or_else(|| serde::de::Error::custom("missing data field"))?;
                serde_json::from_value(data.clone()).map(EventMessageData::Order)
            }
            position if position.starts_with("position") => {
                let data = map
                    .get("data")
                    .ok_or_else(|| serde::de::Error::custom("missing data field"))?;
                serde_json::from_value(data.clone()).map(EventMessageData::Position)
            }
            kline if kline.starts_with("kline") => {
                let data = map
                    .get("data")
                    .ok_or_else(|| serde::de::Error::custom("missing data field"))?;
                serde_json::from_value(data.clone()).map(EventMessageData::Kline)
            }
            _ => Err(serde::de::Error::custom(format!("unknown type: {}", topic))),
        };

        match data {
            Ok(data) => Ok(EventMessage {
                id: map
                    .get("id")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                creation_time: map.get("creationTime").and_then(|v| v.as_u64()),
                message_type: map
                    .get("type")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                ts: map.get("ts").and_then(|v| v.as_u64()),
                cs: map.get("cs").and_then(|v| v.as_u64()),
                data,
            }),
            Err(e) => Err(serde::de::Error::custom(e)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "topic", content = "data")]
#[serde(rename_all = "camelCase")]
pub enum EventMessageData {
    Execution(Vec<Execution>),
    FastExecution(Vec<FastExecution>),
    Order(Vec<Order>),
    Position(Vec<Position>),
    Wallet(Vec<WalletBalance>),
    Kline(Vec<WSKlineData>),
}