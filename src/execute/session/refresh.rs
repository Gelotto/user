use crate::{
  error::ContractError,
  state::{build_session_key, end_session, init_session, load_user_id, require_valid_session},
};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};

pub fn refresh(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  old_seed: String,
  new_seed: String,
) -> Result<Response, ContractError> {
  let user_id = load_user_id(deps.storage, &info.sender)?;
  let old_key = build_session_key(&info.sender, user_id, &old_seed);
  let new_key = build_session_key(&info.sender, user_id, &new_seed);

  require_valid_session(deps.storage, &env.block, user_id, &old_key)?;
  end_session(deps.storage, &old_key)?;
  init_session(deps.storage, &env.block, &info.sender, user_id, &new_key)?;

  Ok(Response::new().add_attributes(vec![
    attr("action", "session_refresh"),
    attr("session_key", new_key.clone()),
  ]))
}
