// *************************************************************************************************
// Ref. https://bybit-exchange.github.io/docs/v5/enum#interval
// *************************************************************************************************

use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Interval {
    #[serde(rename = "1")]
    #[default]
    Min1,
    #[serde(rename = "3")]
    Min3,
    #[serde(rename = "5")]
    Min5,
    #[serde(rename = "15")]
    Min15,
    #[serde(rename = "30")]
    Min30,
    #[serde(rename = "60")]
    Min60,
    #[serde(rename = "120")]
    Min120,
    #[serde(rename = "240")]
    Min240,
    #[serde(rename = "360")]
    Min360,
    #[serde(rename = "720")]
    Min720,
    #[serde(rename = "D")]
    Day,
    #[serde(rename = "W")]
    Week,
    #[serde(rename = "M")]
    Month,
}

impl Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let interval_str = match self {
            Interval::Min1 => "1",
            Interval::Min3 => "3",
            Interval::Min5 => "5",
            Interval::Min15 => "15",
            Interval::Min30 => "30",
            Interval::Min60 => "60",
            Interval::Min120 => "120",
            Interval::Min240 => "240",
            Interval::Min360 => "360",
            Interval::Min720 => "720",
            Interval::Day => "D",
            Interval::Week => "W",
            Interval::Month => "M",
        };
        write!(f, "{}", interval_str)
    }
}

impl From<&str> for Interval {
    fn from(s: &str) -> Self {
        match s {
            "1" => Interval::Min1,
            "3" => Interval::Min3,
            "5" => Interval::Min5,
            "15" => Interval::Min15,
            "30" => Interval::Min30,
            "60" => Interval::Min60,
            "120" => Interval::Min120,
            "240" => Interval::Min240,
            "360" => Interval::Min360,
            "720" => Interval::Min720,
            "D" => Interval::Day,
            "W" => Interval::Week,
            "M" => Interval::Month,
            _ => Interval::Min1, // Default value
        }
    }
}

impl From<String> for Interval {
    fn from(s: String) -> Self {
        Interval::from(s.as_str())
    }
}

impl Interval {
    /// Get all available intervals
    pub fn all() -> Vec<Interval> {
        vec![
            Interval::Min1,
            Interval::Min3,
            Interval::Min5,
            Interval::Min15,
            Interval::Min30,
            Interval::Min60,
            Interval::Min120,
            Interval::Min240,
            Interval::Min360,
            Interval::Min720,
            Interval::Day,
            Interval::Week,
            Interval::Month,
        ]
    }

    /// Get interval description
    pub fn description(&self) -> &'static str {
        match self {
            Interval::Min1 => "1 minute",
            Interval::Min3 => "3 minutes",
            Interval::Min5 => "5 minutes",
            Interval::Min15 => "15 minutes",
            Interval::Min30 => "30 minutes",
            Interval::Min60 => "60 minutes",
            Interval::Min120 => "120 minutes",
            Interval::Min240 => "240 minutes",
            Interval::Min360 => "360 minutes",
            Interval::Min720 => "720 minutes",
            Interval::Day => "day",
            Interval::Week => "week",
            Interval::Month => "month",
        }
    }

    /// Check if it's a minute-level interval
    pub fn is_minute_interval(&self) -> bool {
        matches!(
            self,
            Interval::Min1
                | Interval::Min3
                | Interval::Min5
                | Interval::Min15
                | Interval::Min30
                | Interval::Min60
                | Interval::Min120
                | Interval::Min240
                | Interval::Min360
                | Interval::Min720
        )
    }

    /// Get the number of minutes for the interval (only valid for minute-level intervals)
    pub fn minutes(&self) -> Option<u32> {
        match self {
            Interval::Min1 => Some(1),
            Interval::Min3 => Some(3),
            Interval::Min5 => Some(5),
            Interval::Min15 => Some(15),
            Interval::Min30 => Some(30),
            Interval::Min60 => Some(60),
            Interval::Min120 => Some(120),
            Interval::Min240 => Some(240),
            Interval::Min360 => Some(360),
            Interval::Min720 => Some(720),
            _ => None,
        }
    }
}
