//crates we're using here
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};
//make the Config struct public
//we should make public too
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {   
    pub owner: Addr  
}

// I would change this to u128
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Deposits {
    pub count: i32, 
    pub owner: Addr,
    pub coins: Coin
}
//signed integers alovede negative value, smart contract use unsigned (u128)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Withdraws {
    pub count: u128, 
    pub owner: Addr,
    pub coins: Coin
}

//key is address, denom
//const is immutable variable, the value cannot be changed or shadowed, we're using if we know that's constant 
pub const DEPOSITS: Map<(&str, &str), Deposits> = Map::new("deposits"); 

pub const WITHDRAWS: Map<(&str, &str), Withdraws> = Map::new("withdraws");

pub const CONFIG: Item<Config> = Item::new("config");