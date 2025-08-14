use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinearLeverageFilter {
    pub min_leverage: String,
    pub max_leverage: String,
    pub leverage_step: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinearLotSizeFilter {
    pub max_order_qty: String,
    pub min_order_qty: String,
    pub qty_step: String,
    pub min_notional_value: String,
    pub max_mkt_order_qty: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinearPriceFilter {
    pub min_price: String,
    pub max_price: String,
    pub tick_size: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionLotSizeFilter {
    pub max_order_qty: String,
    pub min_order_qty: String,
    pub qty_step: String,
}

pub type OptionPriceFilter = LinearPriceFilter;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotLotSizeFilter {
    pub base_precision: String,
    pub quote_precision: String,
    pub min_order_qty: String,
    pub max_order_qty: String,
    pub max_order_amt: String,
    pub min_order_amt: String,
}

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct SpotPriceFilter {
//     pub tick_size: String,
// }
