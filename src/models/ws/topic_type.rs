use serde::{Deserialize, Deserializer, Serialize};

use crate::models::Category;

#[derive(Debug, Eq, PartialEq)]
pub enum TopicType {
    Position(Option<Category>),
    Execution(Option<Category>),
    FastExecution(Option<Category>),
    Order(Option<Category>),
    Wallet,
    Ticker(String),
}

impl Serialize for TopicType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self {
            TopicType::Position(category) => match category {
                Some(category) => {
                    serializer.serialize_str(&format!("position.{}", category).to_lowercase())
                }
                None => serializer.serialize_str("position"),
            },
            TopicType::Execution(category) => match category {
                Some(category) => {
                    serializer.serialize_str(&format!("execution.{}", category).to_lowercase())
                }
                None => serializer.serialize_str("execution"),
            },
            TopicType::Order(category) => match category {
                Some(category) => {
                    serializer.serialize_str(&format!("order.{}", category).to_lowercase())
                }
                None => serializer.serialize_str("order"),
            },
            TopicType::FastExecution(category) => match category {
                Some(category) => {
                    serializer.serialize_str(&format!("execution.fast.{}", category).to_lowercase())
                }
                None => serializer.serialize_str("execution.fast"),
            },
            TopicType::Wallet => serializer.serialize_str("wallet"),
            TopicType::Ticker(symbol) => serializer.serialize_str(&format!("tickers.{}", symbol)),
        }
    }
}

impl<'de> Deserialize<'de> for TopicType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;

        match s {
            "position" => Ok(TopicType::Position(None)),
            "execution" => Ok(TopicType::Execution(None)),
            "order" => Ok(TopicType::Order(None)),
            "execution.fast" => Ok(TopicType::FastExecution(None)),
            "wallet" => Ok(TopicType::Wallet),
            _ => {
                let parts: Vec<&str> = s.split('.').collect();
                if parts.len() != 2 {
                    return Err(serde::de::Error::custom("invalid topic"));
                }
                let category =
                    serde_json::from_str::<Category>(parts[1]).map_err(serde::de::Error::custom)?;
                match parts[0] {
                    "position" => Ok(TopicType::Position(category.into())),
                    "execution" => Ok(TopicType::Execution(category.into())),
                    "order" => Ok(TopicType::Order(category.into())),
                    "execution.fast" => Ok(TopicType::FastExecution(category.into())),
                    "tickers" => Ok(TopicType::Ticker(parts[1].to_string())),
                    _ => Err(serde::de::Error::custom("invalid topic")),
                }
            }
        }
    }
}
