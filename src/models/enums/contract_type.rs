// *************************************************************************************************
// Ref. https://bybit-exchange.github.io/docs/v5/enum#contracttype
// *************************************************************************************************

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ContractType {
    InverseFutures,
    InversePerpetual,
    LinearFutures,
    LinearPerpetual,
}

impl From<ContractType> for String {
    fn from(contract_type: ContractType) -> Self {
        match contract_type {
            ContractType::InverseFutures => "InverseFutures".to_string(),
            ContractType::InversePerpetual => "InversePerpetual".to_string(),
            ContractType::LinearFutures => "LinearFutures".to_string(),
            ContractType::LinearPerpetual => "LinearPerpetual".to_string(),
        }
    }
}
