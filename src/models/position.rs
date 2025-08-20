use serde::{Deserialize, Serialize};

use crate::{models::*, utilities::*};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub position_idx: PositionIdxInt,
    pub risk_id: i32,
    pub risk_limit_value: String,
    pub symbol: String,
    #[serde(deserialize_with = "deserialize_empty_object_as_none")]
    pub side: Option<Side>,
    pub size: String,
    pub avg_price: Option<String>,
    pub position_value: String,
    pub trade_mode: i32,
    pub auto_add_margin: i32,
    pub position_status: PositionStatus,
    pub leverage: String,
    pub mark_price: String,
    pub liq_price: String,
    pub bust_price: String,
    #[serde(rename = "positionIm", alias = "positionIM")]
    pub position_im: Option<String>,
    #[serde(rename = "positionMm", alias = "positionMM")]
    pub position_mm: Option<String>,
    pub position_balance: String,
    pub take_profit: String,
    pub stop_loss: String,
    pub trailing_stop: String,
    pub session_avg_price: String,
    pub delta: Option<String>,
    pub gamma: Option<String>,
    pub vega: Option<String>,
    pub theta: Option<String>,
    pub unrealised_pnl: String,
    pub cur_realised_pnl: String,
    pub cum_realised_pnl: String,
    pub adl_rank_indicator: i32,
    pub created_time: String,
    pub updated_time: String,
    pub seq: i64,
    pub is_reduce_only: bool,
    #[serde(deserialize_with = "deserialize_empty_object_as_none")]
    pub mmr_sys_updated_time: Option<String>,
    #[serde(deserialize_with = "deserialize_empty_object_as_none")]
    pub leverage_sys_updated_time: Option<String>,
    pub tpsl_mode: String,
    // Web Socket Only
    pub entry_price: Option<String>,
    pub category: Option<String>,
}
