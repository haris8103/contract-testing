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
        
        let info = mock_info(&"loop_staker1".to_string(), &[]);
        let _result = instantiate(deps.as_mut(), mock_env(), info.clone()).unwrap();

        let add_customer = ExecuteMsg::AddFunction {
            first_number : 5,
            second_number : 10,
        };

        let response = execute(mock_env(), add_customer.clone());
        
        if let  Err(StdError::GenericErr { msg,.. }) = response {
            panic!("Error msg received msg: {} ", msg);
        } else {
           let response_result =  response.unwrap();
           let attribute = response_result.attributes.get(0);
            if let Some(attribute) = attribute {
               assert_eq!( attribute.value, "20");
            }
        }
        
    }    
}
