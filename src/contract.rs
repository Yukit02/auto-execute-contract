#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdError};
use cw2::set_contract_version;

use sei_cosmwasm::{SeiQueryWrapper};

use crate::error::ContractError;
use crate::msg::*;
use crate::sei_type::*;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:auto-execute-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut<SeiQueryWrapper>, env: Env, msg: SudoMsg) -> Result<Response, StdError> {
    match msg {
        SudoMsg::Settlement { epoch: _, entries: _ } => Ok(Response::new()),
        SudoMsg::NewBlock { epoch: _ } => Ok(Response::new()),
        SudoMsg::BulkOrderPlacements { orders: _, deposits: _ } => {
            Ok(Response::new())
        }
        SudoMsg::BulkOrderCancellations { ids: _ } => Ok(Response::new()),
        SudoMsg::Liquidation { requests: _ } => Ok(Response::new()),
        SudoMsg::FinalizeBlock { contract_order_results } => sudo_execution::finalize_block(deps, env, contract_order_results)
    }
}

pub mod sudo_execution {
  use super::*;

  pub fn finalize_block(
    _deps: DepsMut<SeiQueryWrapper>,
    env: Env,
    _contract_order_results: Vec<ContractOrderResult>
  ) -> Result<Response, StdError> {
    if env.block.height % 10 == 0 {}

    Ok(Response::new())
  }
}
