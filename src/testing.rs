use cosmwasm_std::testing::mock_dependencies;
use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::StdError;

use crate::contract::{execute, instantiate};
use crate::msg::ExecuteMsg;

mod tests {
    use super::*;


    #[test]
    fn test_add_number_function() {
        let mut deps = mock_dependencies(&[]);
        //let init_msg = InstantiateMsg { };///
        let info = mock_info(&"loop_staker1".to_string(), &[]);
        let _result = instantiate(deps.as_mut(), mock_env(), info.clone()).unwrap();

        let add_customer = ExecuteMsg::AddFunction {
            first_number : 5,
            second_number : 10,
        };

        let response = execute(deps.as_mut(), mock_env(), add_customer.clone());
        
        if let  Err(StdError::GenericErr { msg,.. }) = response {
            panic!("Error msg received msg: {} ", msg);
        } else {
           let response_result =  response.unwrap();
           let attribute = response_result.attributes.get(0);
            if let Some(attribute) = attribute {
               assert_eq!( attribute.value, "15");
            }
        }
        
    }

    // #[test]
    // fn test_add_customer() {
    //     let mut deps = mock_dependencies(&[]);
    //     //let init_msg = InstantiateMsg { };///
    //     let info = mock_info(&"loop_staker1".to_string(), &[]);
    //     let _result = instantiate(deps.as_mut(), mock_env(), info.clone()).unwrap();
    //     let name = "Muhammad Haris";
    //     let ssn = "352025966101";
    //     let address = "573 B block";
    //     let favouritecolor = "Red";

    //     let add_customer = ExecuteMsg::AddCustomer {
    //         name: name.to_string(),
    //         ssn: ssn.to_string(),
    //         address: address.to_string(),
    //         favouritecolor: favouritecolor.to_string(),
    //     };

    //     let res = execute(deps.as_mut(), mock_env(), add_customer.clone());
        
    //     if let  Err(StdError::GenericErr { msg,.. }) = res {
    //         panic!("Error msg received msg: {} ", msg);
    //     }
    // }    
}
