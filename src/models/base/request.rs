use std::collections::HashMap;

use anyhow::Result;
use serde::{Serialize, de::DeserializeOwned};

use crate::models::Endpoint;

pub(crate) trait ApiRequest
where
    Self: Serialize,
{
    type ApiResponse: DeserializeOwned;

    fn endpoint() -> Endpoint;

    fn to_query_map(&self) -> Result<HashMap<String, String>> {
        match serde_json::to_value(self)?.as_object() {
            None => Ok(HashMap::new()),
            Some(v) => Ok(v
                .iter()
                .filter_map(|(k, v)| {
                    if v.is_null() {
                        None
                    } else {
                        Some((k.clone(), v.as_str().unwrap_or(&v.to_string()).to_string()))
                    }
                })
                .collect()),
        }
    }

    fn to_query_string(&self) -> Result<String> {
        let mut map_data = self
            .to_query_map()?
            .iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<String>>();
        map_data.sort();
        Ok(map_data.join("&"))
    }
}
