// *************************************************************************************************
// Ref. https://bybit-exchange.github.io/docs/v5/enum#rejectreason
// *************************************************************************************************

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default, Clone)]
pub enum RejectReason {
    #[default]
    #[serde(rename = "EC_NoError")]
    ErrorCodeNoError,
    #[serde(rename = "EC_Others")]
    ErrorCodeOthers,
    #[serde(rename = "EC_UnknownMessageType")]
    ErrorCodeUnknownMessageType,
    #[serde(rename = "EC_MissingClOrdID")]
    ErrorCodeMissingClOrdID,
    #[serde(rename = "EC_MissingOrigClOrdID")]
    ErrorCodeMissingOrigClOrdID,
    #[serde(rename = "EC_ClOrdIDOrigClOrdIDAreTheSame")]
    ErrorCodeClOrdIDOrigClOrdIDAreTheSame,
    #[serde(rename = "EC_DuplicatedClOrdID")]
    ErrorCodeDuplicatedClOrdID,
    #[serde(rename = "EC_OrigClOrdIDDoesNotExist")]
    ErrorCodeOrigClOrdIDDoesNotExist,
    #[serde(rename = "EC_TooLateToCancel")]
    ErrorCodeTooLateToCancel,
    #[serde(rename = "EC_UnknownOrderType")]
    ErrorCodeUnknownOrderType,
    #[serde(rename = "EC_UnknownSide")]
    ErrorCodeUnknownSide,
    #[serde(rename = "EC_UnknownTimeInForce")]
    ErrorCodeUnknownTimeInForce,
    #[serde(rename = "EC_WronglyRouted")]
    ErrorCodeWronglyRouted,
    #[serde(rename = "EC_MarketOrderPriceIsNotZero")]
    ErrorCodeMarketOrderPriceIsNotZero,
    #[serde(rename = "EC_LimitOrderInvalidPrice")]
    ErrorCodeLimitOrderInvalidPrice,
    #[serde(rename = "EC_NoEnoughQtyToFill")]
    ErrorCodeNoEnoughQtyToFill,
    #[serde(rename = "EC_NoImmediateQtyToFill")]
    ErrorCodeNoImmediateQtyToFill,
    #[serde(rename = "EC_PerCancelRequest")]
    ErrorCodePerCancelRequest,
    #[serde(rename = "EC_MarketOrderCannotBePostOnly")]
    ErrorCodeMarketOrderCannotBePostOnly,
    #[serde(rename = "EC_PostOnlyWillTakeLiquidity")]
    ErrorCodePostOnlyWillTakeLiquidity,
    #[serde(rename = "EC_CancelReplaceOrder")]
    ErrorCodeCancelReplaceOrder,
    #[serde(rename = "EC_InvalidSymbolStatus")]
    ErrorCodeInvalidSymbolStatus,
    #[serde(rename = "EC_CancelForNoFullFill")]
    ErrorCodeCancelForNoFullFill,
    #[serde(rename = "EC_BySelfMatch")]
    ErrorCodeBySelfMatch,
    #[serde(rename = "EC_InCallAuctionStatus")]
    ErrorCodeInCallAuctionStatus,
    #[serde(rename = "EC_QtyCannotBeZero")]
    ErrorCodeQtyCannotBeZero,
    #[serde(rename = "EC_MarketOrderNoSupportTIF")]
    ErrorCodeMarketOrderNoSupportTIF,
    #[serde(rename = "EC_ReachMaxTradeNum")]
    ErrorCodeReachMaxTradeNum,
    #[serde(rename = "EC_InvalidPriceScale")]
    ErrorCodeInvalidPriceScale,
    #[serde(rename = "EC_BitIndexInvalid")]
    ErrorCodeBitIndexInvalid,
    #[serde(rename = "EC_StopBySelfMatch")]
    ErrorCodeStopBySelfMatch,
    #[serde(rename = "EC_InvalidSmpType")]
    ErrorCodeInvalidSmpType,
    #[serde(rename = "EC_CancelByMMP")]
    ErrorCodeCancelByMMP,
    #[serde(rename = "EC_InvalidUserType")]
    ErrorCodeInvalidUserType,
    #[serde(rename = "EC_InvalidMirrorOid")]
    ErrorCodeInvalidMirrorOid,
    #[serde(rename = "EC_InvalidMirrorUid")]
    ErrorCodeInvalidMirrorUid,
    #[serde(rename = "EC_EcInvalidQty")]
    ErrorCodeEcInvalidQty,
    #[serde(rename = "EC_InvalidAmount")]
    ErrorCodeInvalidAmount,
    #[serde(rename = "EC_LoadOrderCancel")]
    ErrorCodeLoadOrderCancel,
    #[serde(rename = "EC_MarketQuoteNoSuppSell")]
    ErrorCodeMarketQuoteNoSuppSell,
    #[serde(rename = "EC_DisorderOrderID")]
    ErrorCodeDisorderOrderID,
    #[serde(rename = "EC_InvalidBaseValue")]
    ErrorCodeInvalidBaseValue,
    #[serde(rename = "EC_LoadOrderCanMatch")]
    ErrorCodeLoadOrderCanMatch,
    #[serde(rename = "EC_SecurityStatusFail")]
    ErrorCodeSecurityStatusFail,
    #[serde(rename = "EC_ReachRiskPriceLimit")]
    ErrorCodeReachRiskPriceLimit,
    #[serde(rename = "EC_OrderNotExist")]
    ErrorCodeOrderNotExist,
    #[serde(rename = "EC_CancelByOrderValueZero")]
    ErrorCodeCancelByOrderValueZero,
    #[serde(rename = "EC_CancelByMatchValueZero")]
    ErrorCodeCancelByMatchValueZero,
    #[serde(rename = "EC_ReachMarketPriceLimit")]
    ErrorCodeReachMarketPriceLimit,
}
