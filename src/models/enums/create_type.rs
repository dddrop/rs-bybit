// *************************************************************************************************
// Ref. https://bybit-exchange.github.io/docs/v5/enum#createtype
// *************************************************************************************************

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default, Clone)]
pub enum CreateType {
    CreateByUser,
    CreateByAdminClosing,
    CreateBySettle,
    CreateByStopOrder,
    CreateByTakeProfit,
    CreateByPartialTakeProfit,
    CreateByStopLoss,
    CreateByPartialStopLoss,
    CreateByTrailingStop,
    CreateByLiq,
    #[serde(rename = "CreateByTakeOver_PassThrough")]
    CreateByTakeOverPassThrough,
    #[serde(rename = "CreateByAdl_PassThrough")]
    CreateByAdlPassThrough,
    #[serde(rename = "CreateByBlock_PassThrough")]
    CreateByBlockPassThrough,
    #[serde(rename = "CreateByBlockTradeMovePosition_PassThrough")]
    CreateByBlockTradeMovePositionPassThrough,
    CreateByClosing,
    CreateByFGridBot,
    CloseByFGridBot,
    CreateByTWAP,
    CreateByTVSignal,
    CreateByMmRateClose,
    CreateByMartingaleBot,
    CloseByMartingaleBot,
    CreateByIceBerg,
    CreateByArbitrage,
    CreateByDdh,
    #[default]
    #[serde(rename = "UNKNOWN", alias = "")]
    UNKNOWN,
}
