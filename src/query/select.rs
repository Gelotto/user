use crate::{error::ContractError, msg::SelectResponse, state::OWNER};
use cosmwasm_std::{Addr, Deps};

pub fn select(
  deps: Deps,
  _fields: Option<Vec<String>>,
  _account: Option<Addr>,
) -> Result<SelectResponse, ContractError> {
  Ok(SelectResponse {
    owner: OWNER.may_load(deps.storage)?,
  })
}
