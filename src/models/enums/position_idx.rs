// *************************************************************************************************
// Ref. https://bybit-exchange.github.io/docs/v5/enum#positionidx
// *************************************************************************************************

use std::fmt::Display;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error as DeError, Unexpected},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PositionIdx {
    #[default]
    OneWay,
    HedgeBuySide,
    HedgeSellSide,
}

impl PositionIdx {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(PositionIdx::OneWay),
            1 => Some(PositionIdx::HedgeBuySide),
            2 => Some(PositionIdx::HedgeSellSide),
            _ => None,
        }
    }

    fn to_u8(self) -> u8 {
        match self {
            PositionIdx::OneWay => 0,
            PositionIdx::HedgeBuySide => 1,
            PositionIdx::HedgeSellSide => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PositionIdxInt(pub PositionIdx);

impl Serialize for PositionIdxInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.0.to_u8())
    }
}

impl<'de> Deserialize<'de> for PositionIdxInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        PositionIdx::from_u8(value)
            .map(PositionIdxInt)
            .ok_or_else(|| {
                DeError::invalid_value(Unexpected::Unsigned(value as u64), &"0, 1, or 2")
            })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PositionIdxString(pub PositionIdx);

impl Serialize for PositionIdxString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_u8().to_string())
    }
}

impl<'de> Deserialize<'de> for PositionIdxString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let parsed: u8 = s.parse().map_err(|_| {
            DeError::invalid_value(Unexpected::Str(&s), &"string representing 0, 1, or 2")
        })?;
        PositionIdx::from_u8(parsed)
            .map(PositionIdxString)
            .ok_or_else(|| DeError::invalid_value(Unexpected::Str(&s), &"\"0\", \"1\", or \"2\""))
    }
}

impl Display for PositionIdx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// TEST

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_position_idx_from_u8() {
        assert_eq!(PositionIdx::from_u8(0), Some(PositionIdx::OneWay));
        assert_eq!(PositionIdx::from_u8(1), Some(PositionIdx::HedgeBuySide));
        assert_eq!(PositionIdx::from_u8(2), Some(PositionIdx::HedgeSellSide));
        assert_eq!(PositionIdx::from_u8(3), None);
    }

    #[test]
    fn test_position_idx_to_u8() {
        assert_eq!(PositionIdx::OneWay.to_u8(), 0);
        assert_eq!(PositionIdx::HedgeBuySide.to_u8(), 1);
        assert_eq!(PositionIdx::HedgeSellSide.to_u8(), 2);
    }

    #[test]
    fn test_position_idx_int_serialize() {
        let one_way = PositionIdxInt(PositionIdx::OneWay);
        let buy_side = PositionIdxInt(PositionIdx::HedgeBuySide);
        let sell_side = PositionIdxInt(PositionIdx::HedgeSellSide);

        assert_eq!(serde_json::to_string(&one_way).unwrap(), "0");
        assert_eq!(serde_json::to_string(&buy_side).unwrap(), "1");
        assert_eq!(serde_json::to_string(&sell_side).unwrap(), "2");
    }

    #[test]
    fn test_position_idx_int_deserialize() {
        let one_way: PositionIdxInt = serde_json::from_value(json!(0)).unwrap();
        let buy_side: PositionIdxInt = serde_json::from_value(json!(1)).unwrap();
        let sell_side: PositionIdxInt = serde_json::from_value(json!(2)).unwrap();

        assert_eq!(one_way.0, PositionIdx::OneWay);
        assert_eq!(buy_side.0, PositionIdx::HedgeBuySide);
        assert_eq!(sell_side.0, PositionIdx::HedgeSellSide);
    }

    #[test]
    fn test_position_idx_string_serialize() {
        let one_way = PositionIdxString(PositionIdx::OneWay);
        let buy_side = PositionIdxString(PositionIdx::HedgeBuySide);
        let sell_side = PositionIdxString(PositionIdx::HedgeSellSide);

        assert_eq!(serde_json::to_string(&one_way).unwrap(), "\"0\"");
        assert_eq!(serde_json::to_string(&buy_side).unwrap(), "\"1\"");
        assert_eq!(serde_json::to_string(&sell_side).unwrap(), "\"2\"");
    }

    #[test]
    fn test_position_idx_string_deserialize() {
        let one_way: PositionIdxString = serde_json::from_value(json!("0")).unwrap();
        let buy_side: PositionIdxString = serde_json::from_value(json!("1")).unwrap();
        let sell_side: PositionIdxString = serde_json::from_value(json!("2")).unwrap();

        assert_eq!(one_way.0, PositionIdx::OneWay);
        assert_eq!(buy_side.0, PositionIdx::HedgeBuySide);
        assert_eq!(sell_side.0, PositionIdx::HedgeSellSide);
    }

    #[test]
    fn test_position_idx_invalid_deserialization() {
        let result: Result<PositionIdxInt, _> = serde_json::from_value(json!(3));
        assert!(result.is_err());

        let result: Result<PositionIdxString, _> = serde_json::from_value(json!("3"));
        assert!(result.is_err());

        let result: Result<PositionIdxString, _> = serde_json::from_value(json!("invalid"));
        assert!(result.is_err());
    }

    #[test]
    fn test_position_idx_display() {
        assert_eq!(PositionIdx::OneWay.to_string(), "OneWay");
        assert_eq!(PositionIdx::HedgeBuySide.to_string(), "HedgeBuySide");
        assert_eq!(PositionIdx::HedgeSellSide.to_string(), "HedgeSellSide");
    }
}
