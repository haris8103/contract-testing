#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,StdError};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{CountResponse, ExecuteMsg, QueryMsg};
use crate::state::{State, Owner, STATE, CUSTOMERS_MAPS};

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
    deps: DepsMut,
    _env: Env,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        
        ExecuteMsg::AddFunction {first_number, second_number} => add_function(first_number, second_number),
        //ExecuteMsg::AddCustomer {name , ssn, address, favouritecolor} => add_customer(deps, name , ssn, address, favouritecolor),
        //ExecuteMsg::Reset { count } => try_reset(deps, info, count),
    }
}



pub fn add_function(first_number: u64, second_number: u64) -> StdResult<Response> {
    let result = first_number + second_number;
    Ok(Response::new()
    .add_attribute("result", result.to_string()))
}


// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
//     match msg {
//         QueryMsg::GetCount {ssn} => to_binary(&query_count(deps, ssn)?),
//     }
// }

// fn query_count(deps: Deps, ssn : String) -> StdResult<CountResponse> {
//     let option = CUSTOMERS_MAPS.may_load(deps.storage, ssn)?;
//     if let Some(state) = option{

//         return Ok(CountResponse {  name : state.name, ssn: state.ssn, address : state.address, favouritecolor : state.favouritecolor });
//     }
//     else{
//         return Ok(CountResponse {name : "Not Found".to_string(),  ssn : "Not Found".to_string(), address : "Not Found".to_string(), favouritecolor : "Not Found".to_string()});
//     }
    
// }


// pub fn try_reset(deps: DepsMut, info: MessageInfo, count: u32) -> Result<Response, ContractError> {
//     STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
//         if info.sender != state.owner {
//             return Err(ContractError::Unauthorized {});
//         }
//         state.count = count;
//         Ok(state)
//     })?;
//     Ok(Response::new().add_attribute("method", "reset"))
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
//     use cosmwasm_std::{coins, from_binary};

//     #[test]
//     fn proper_initialization() {
//         let mut deps = mock_dependencies(&[]);

//         let msg = InstantiateMsg { count: 17 };
//         let info = mock_info("creator", &coins(1000, "earth"));

//         // we can just call .unwrap() to assert this was a success
//         let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
//         assert_eq!(0, res.messages.len());

//         // it worked, let's query the state
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(17, value.count);
//     }

    // #[test]
    // fn increment() {
    //     let mut deps = mock_dependencies(&coins(2, "token"));

    //     let msg = InstantiateMsg { count: 17 };
    //     let info = mock_info("creator", &coins(2, "token"));
    //     let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     // beneficiary can release it
    //     let info = mock_info("anyone", &coins(2, "token"));
    //     let msg = ExecuteMsg::Increment {};
    //     let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     // should increase counter by 1
    //     let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    //     let value: CountResponse = from_binary(&res).unwrap();
    //     assert_eq!(18, value.count);
    // }

    // #[test]
    // fn reset() {
    //     let mut deps = mock_dependencies(&coins(2, "token"));

    //     let msg = InstantiateMsg { count: 17 };
    //     let info = mock_info("creator", &coins(2, "token"));
    //     let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     // beneficiary can release it
    //     let unauth_info = mock_info("anyone", &coins(2, "token"));
    //     let msg = ExecuteMsg::Reset { count: 5 };
    //     let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
    //     match res {
    //         Err(ContractError::Unauthorized {}) => {}
    //         _ => panic!("Must return unauthorized error"),
    //     }

    //     // only the original creator can reset the counter
    //     let auth_info = mock_info("creator", &coins(2, "token"));
    //     let msg = ExecuteMsg::Reset { count: 5 };
    //     let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

    //     // should now be 5
    //     let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    //     let value: CountResponse = from_binary(&res).unwrap();
    //     assert_eq!(5, value.count);
    // }
    // pub fn add_customer(deps: DepsMut, name : String, ssn : String, address : String, favouritecolor : String) -> StdResult<Response> {
//     let result = CUSTOMERS_MAPS.may_load(deps.storage, ssn.to_string());
//     if let Ok(Some(_)) = result {
//        return Err(StdError::generic_err("customer already exists")) 
//     } 
//     let state = State {
        
//         name : name,
//         ssn: ssn, 
//         address : address, 
//         favouritecolor : favouritecolor
//     };
//     CUSTOMERS_MAPS.save(deps.storage, state.ssn.to_string(), &state)?;
//     Ok(Response::new()
//     .add_attribute("method", "try_increment")
//     .add_attribute("name", state.name)
//     .add_attribute("address", state.address)
//     .add_attribute("ssn", state.ssn)
//     .add_attribute("favouritecolor", state.favouritecolor))
// }

//}
