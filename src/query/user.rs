use crate::{
  error::ContractError,
  models::User,
  msg::UserQueryTarget,
  state::{load_user, ADDRESS_TO_ID},
};
use cosmwasm_std::Deps;

pub fn user(
  deps: Deps,
  target: UserQueryTarget,
) -> Result<User, ContractError> {
  Ok(match target {
    UserQueryTarget::Id(id) => load_user(deps.storage, id.into())?,
    UserQueryTarget::Address(addr) => {
      if let Some(id) = ADDRESS_TO_ID.may_load(deps.storage, addr)? {
        load_user(deps.storage, id.into())?
      } else {
        return Err(ContractError::UserNotFound);
      }
    },
  })
}
