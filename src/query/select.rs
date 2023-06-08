use crate::{
  error::ContractError,
  models::ContractMetadata,
  msg::SelectResponse,
  state::{load_user, ADDRESS_TO_ID, OWNER, USER_COUNT},
};
use cosmwasm_std::{Addr, Deps};
use cw_repository::client::Repository;

pub fn select(
  deps: Deps,
  maybe_fields: Option<Vec<String>>,
  maybe_account: Option<Addr>,
) -> Result<SelectResponse, ContractError> {
  let loader = Repository::loader(deps.storage, &maybe_fields, &maybe_account);

  Ok(SelectResponse {
    owner: loader.get("owner", &OWNER)?,

    metadata: loader.view("metadata", |_| {
      Ok(Some(ContractMetadata {
        n_users: USER_COUNT.load(deps.storage)?,
      }))
    })?,

    user: loader.view("user", |maybe_wallet| {
      if let Some(wallet) = maybe_wallet {
        if let Some(user_id) = ADDRESS_TO_ID.may_load(deps.storage, wallet.clone())? {
          return Ok(load_user(deps.storage, user_id.u64()).ok());
        }
      }
      Ok(None)
    })?,
  })
}
