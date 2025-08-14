use serde::{Deserialize, de::DeserializeOwned};

use crate::utilities::deserialize_empty_object_as_none;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ResponseWrapper<T>
where
    T: DeserializeOwned,
{
    pub(crate) ret_code: i32,
    pub(crate) ret_msg: String,
    #[serde(deserialize_with = "deserialize_empty_object_as_none")]
    pub(crate) result: Option<T>,
    pub(crate) time: Option<i64>,
}
