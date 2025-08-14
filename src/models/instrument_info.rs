use serde::Deserialize;

use crate::models::{
    enums::{
        auction_phase::AuctionPhase, contract_type::ContractType, copy_trading::CopyTrading,
        margin_trading::MarginTrading, options_type::OptionsType, status::Status,
    },
    filters::{
        LinearLeverageFilter, LinearLotSizeFilter, LinearPriceFilter, OptionLotSizeFilter,
        OptionPriceFilter, SpotLotSizeFilter,
    },
};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum InstrumentInfo {
    Linear(LinearInstrumentInfo),
    Inverse(InverseInstrumentInfo),
    Spot(SpotInstrumentInfo),
    Option(OptionInstrumentInfo),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinearInstrumentInfo {
    pub symbol: String,
    pub contract_type: ContractType,
    pub status: Status,
    pub base_coin: String,
    pub quote_coin: String,
    pub launch_time: String,
    pub delivery_time: String,
    pub delivery_fee_rate: String,
    pub price_scale: String,
    pub leverage_filter: LinearLeverageFilter,
    pub price_filter: LinearPriceFilter,
    pub lot_size_filter: LinearLotSizeFilter,
    pub unified_margin_trade: bool,
    pub funding_interval: i32,
    pub settle_coin: String,
    pub copy_trading: CopyTrading,
    pub upper_funding_rate: String,
    pub lower_funding_rate: String,
    pub risk_parameters: RiskParameters,
    pub is_pre_listing: bool,
    pub pre_listing_info: Option<PreListingInfo>,
}

pub type InverseInstrumentInfo = LinearInstrumentInfo;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotInstrumentInfo {
    pub symbol: String,
    pub base_coin: String,
    pub quote_coin: String,
    pub innovation: String,
    pub status: Status,
    pub margin_trading: MarginTrading,
    pub st_tag: String,
    pub lot_size_filter: SpotLotSizeFilter,
    pub price_filter: LinearPriceFilter,
    pub risk_parameters: RiskParameters,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionInstrumentInfo {
    pub symbol: String,
    pub options_type: OptionsType,
    pub status: Status,
    pub base_coin: String,
    pub quote_coin: String,
    pub settle_coin: String,
    pub launch_time: String,
    pub delivery_time: String,
    pub delivery_fee_rate: String,
    pub price_filter: OptionPriceFilter,
    pub lot_size_filter: OptionLotSizeFilter,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskParameters {
    pub price_limit_ratio_x: String,
    pub price_limit_ratio_y: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreListingInfo {
    pub cur_auction_phase: AuctionPhase,
    pub phases: Vec<Phase>,
    pub auction_fee_info: AuctionFeeInfo,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Phase {
    pub phase: AuctionPhase,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuctionFeeInfo {
    pub auction_fee_rate: String,
    pub taker_fee_rate: String,
    pub maker_fee_rate: String,
}
