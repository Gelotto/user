use crate::{
  error::ContractError,
  state::{build_session_key, end_session, load_user_id},
};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};

pub fn end(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  seed: String,
) -> Result<Response, ContractError> {
  let user_id = load_user_id(deps.storage, &info.sender)?;
  let key = build_session_key(&info.sender, user_id, &seed);

  end_session(deps.storage, &key)?;

  Ok(Response::new().add_attributes(vec![
    attr("action", "session_end"),
    attr("session_key", key.clone()),
  ]))
}
