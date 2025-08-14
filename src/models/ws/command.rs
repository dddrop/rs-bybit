use core::fmt;

use serde::{Deserialize, Serialize, ser::SerializeSeq};
use serde_json::Value;

use crate::models::ws::topic_type::TopicType;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Command {
    pub conn_id: Option<String>,
    #[serde(flatten)]
    pub command_type: CommandType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "op", content = "args", rename_all = "lowercase")]
pub enum CommandType {
    Auth(Option<AuthOperation>),
    Ping,
    Subscribe(Option<SubscribeArgs>),
    Unsubscribe(Option<UnsubscribeArgs>),
    Other(Option<Value>),
}

impl fmt::Display for CommandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandType::Auth(args) => write!(f, "[WS][OP] auth: {:?}", args),
            CommandType::Ping => write!(f, "[WS][OP] ping"),
            CommandType::Subscribe(args) => write!(f, "[WS][OP] subscribe: {:?}", args),
            CommandType::Unsubscribe(args) => write!(f, "[WS][OP] unsubscribe: {:?}", args),
            CommandType::Other(args) => write!(f, "other: {:?}", args),
        }
    }
}

// MARK: Auth
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct AuthOperation {
    pub key: String,
    pub timestamp: u128,
    pub signature: String,
}

impl Serialize for AuthOperation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(3))?;
        seq.serialize_element(&self.key)?;
        seq.serialize_element(&self.timestamp)?;
        seq.serialize_element(&self.signature)?;
        seq.end()
    }
}

// MARK: Subscribe
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct SubscribeArgs {
    pub topics: Vec<TopicType>,
}

impl Serialize for SubscribeArgs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(1))?;
        let _ = &self.topics.iter().for_each(|topic| {
            seq.serialize_element(topic).unwrap();
        });
        seq.end()
    }
}

// MARK: Unsubscribe
pub type UnsubscribeArgs = SubscribeArgs;
