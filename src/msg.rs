use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint64};
use cw_lib::models::Owner;

use crate::models::UserProfile;

#[cw_serde]
pub struct InstantiateMsg {
  pub owner: Option<Owner>,
}

#[cw_serde]
pub enum SessionMsg {
  Start { seed: String },
  End { seed: String },
  Refresh { old_seed: String, new_seed: String },
}

#[cw_serde]
pub enum ExecuteMsg {
  Register { profile: UserProfile },
  Session(SessionMsg),
}

#[cw_serde]
pub enum QueryMsg {
  Select {
    fields: Option<Vec<String>>,
    wallet: Option<Addr>,
  },
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct SelectResponse {
  pub owner: Option<Owner>,
}

#[cw_serde]
pub enum UserQueryTarget {
  Id(Uint64),
  Address(Addr),
}
