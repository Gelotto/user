use crate::{
  error::ContractError,
  models::UserSession,
  state::{build_session_key, load_user_id, require_valid_session},
};
use cosmwasm_std::{Addr, Deps, Env};

pub fn session(
  deps: Deps,
  env: Env,
  address: Addr,
  seed: String,
) -> Result<Option<UserSession>, ContractError> {
  let user_id = load_user_id(deps.storage, &address)?;
  let key = build_session_key(&address, user_id, &seed);
  Ok(require_valid_session(deps.storage, &env.block, user_id, &key).ok())
}
