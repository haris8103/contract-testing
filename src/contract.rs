#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg};
use crate::state::{Owner, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:my-first-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let owner = Owner {
        owner : info.sender
    };
    
    STATE.save(deps.storage, &owner )?;
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", owner.owner))
        //.add_attribute("amount", state.amount.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    //deps: DepsMut,
    _env: Env,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        
        ExecuteMsg::AddFunction {first_number, second_number} => add_function(first_number, second_number),
        
    }
}



pub fn add_function(first_number: u64, second_number: u64) -> StdResult<Response> {
    let result = first_number + second_number;
    println!("Answer is {} ", result.to_string());
    Ok(Response::new()
    .add_attribute("result", result.to_string()))
}