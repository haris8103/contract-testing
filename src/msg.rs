use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
   // ,  pub amount : f64
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    AddFunction{first_number: u64, second_number: u64},
    //AddCustomer{name : String,  ssn: String, address : String, favouritecolor : String},
    //Reset { count: u32},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetCount {ssn : String},
    // GetCount returns the current count as a json-encoded number
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    
    pub name : String,
    pub ssn : String,
    pub address : String,
    pub favouritecolor : String
    //pub count: u32,
}
