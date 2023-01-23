use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

use crate::sei_type::*;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SudoMsg {
    Settlement {
        epoch: i64,
        entries: Vec<SettlementEntry>,
    },
    NewBlock {
        epoch: i64,
    },
    BulkOrderPlacements {
        orders: Vec<Order>,
        deposits: Vec<DepositInfo>,
    },
    BulkOrderCancellations {
        ids: Vec<u64>,
    },
    Liquidation {
        requests: Vec<LiquidationRequest>,
    },
    FinalizeBlock {
        contract_order_results: Vec<ContractOrderResult>,
    },
}
