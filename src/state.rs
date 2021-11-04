use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use cw_storage_plus::Map;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
  //  pub count: u32,
    
    pub name : String,
    pub ssn : String,
    pub address : String,
    pub favouritecolor : String
    //, pub amount : f64
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Owner {
    pub owner: Addr,
}
pub const STATE: Item<Owner> = Item::new("owner");
pub const CUSTOMERS_MAPS : Map<String, State> = Map::new("myMapStorgae");