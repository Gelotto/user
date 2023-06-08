use crate::{
  error::ContractError,
  models::UserProfile,
  state::{init_user, init_user_id},
};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};

pub fn register(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  profile: UserProfile,
) -> Result<Response, ContractError> {
  let user_id = init_user_id(deps.storage, &info.sender)?;
  init_user(deps.storage, user_id, &profile, &env.block)?;
  Ok(Response::new().add_attributes(vec![
    attr("action", "register"),
    attr("user_id", user_id.to_string()),
  ]))
}
