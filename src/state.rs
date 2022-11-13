use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Deposits {
    pub count: i32, // I would change this to u128
    pub owner: Addr,
    pub coins: Coin
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Withdraws {
    pub count: u128, //signed integers alovede negative value, smart contract use unsigned (u128)
    pub owner: Addr,
    pub coins: Coin
}

//key is address, denom
pub const DEPOSITS: Map<(&str, &str), Deposits> = Map::new("deposits");

pub const WITHDRAWS: Map<(&str, &str), Withdraws> = Map::new("withdraws");

pub const CONFIG: Item<Config> = Item::new("config");