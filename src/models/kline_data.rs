use std::fmt;

use serde::{
    Deserialize, Deserializer, Serialize,
    de::{self, SeqAccess, Visitor},
};

#[derive(Debug, Clone, Serialize)]
pub struct KlineData {
    pub timestamp: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
    pub turnover: String,
}

impl<'de> Deserialize<'de> for KlineData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct KlineDataVisitor;

        impl<'de> Visitor<'de> for KlineDataVisitor {
            type Value = KlineData;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an array of 7 strings representing kline data")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<KlineData, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let timestamp = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let open = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let high = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let low = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                let close = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(4, &self))?;
                let volume = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(5, &self))?;
                let turnover = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(6, &self))?;

                Ok(KlineData {
                    timestamp,
                    open,
                    high,
                    low,
                    close,
                    volume,
                    turnover,
                })
            }
        }

        deserializer.deserialize_seq(KlineDataVisitor)
    }
}
