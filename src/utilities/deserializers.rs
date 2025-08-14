use serde::{Deserialize, Deserializer, de, de::DeserializeOwned};

pub(crate) fn deserialize_empty_object_as_none<'de, D, T>(d: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    let value = serde_json::Value::deserialize(d)?;

    match value {
        serde_json::Value::Null => Ok(None),
        serde_json::Value::Object(ref obj) if obj.is_empty() => Ok(None),
        serde_json::Value::String(ref s) if s.is_empty() => Ok(None),
        _ => T::deserialize(value).map(Some).map_err(de::Error::custom),
    }
}

pub(crate) fn deserialize_string_or_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    // Try to deserialize as string
    let value = serde_json::Value::deserialize(deserializer)?;

    match value {
        serde_json::Value::String(s) => match s.as_str() {
            "0" => Ok(Some(false)),
            "1" => Ok(Some(true)),
            "" => Ok(None),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid boolean string: {}",
                s
            ))),
        },
        serde_json::Value::Bool(b) => Ok(Some(b)),
        serde_json::Value::Null => Ok(None),
        _ => Err(serde::de::Error::custom("Expected boolean or string")),
    }
}
