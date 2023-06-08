use crate::{
  error::ContractError,
  state::{build_session_key, init_session, load_user_id},
};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};

pub fn start(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  seed: String,
) -> Result<Response, ContractError> {
  let user_id = load_user_id(deps.storage, &info.sender)?;
  let key = build_session_key(&info.sender, user_id, &seed);

  init_session(deps.storage, &env.block, &info.sender, user_id, &key)?;

  Ok(Response::new().add_attributes(vec![
    attr("action", "session_start"),
    attr("session_key", key.clone()),
  ]))
}
