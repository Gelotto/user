use crate::models::{User, UserMetadata};
use crate::msg::InstantiateMsg;
use crate::{error::ContractError, models::Profile};
use cosmwasm_std::{Addr, BlockInfo, Deps, DepsMut, Env, MessageInfo, Order, Storage, Uint64};
use cw_acl::client::Acl;
use cw_lib::models::Owner;
use cw_storage_plus::{Item, Map};

pub type LookupTable<'a, T> = Map<'a, T, bool>;

pub const OWNER: Item<Owner> = Item::new("owner");
pub const PROFILES: Map<u64, Profile> = Map::new("profiles");
pub const METADATA: Map<u64, UserMetadata> = Map::new("metadata");
pub const ID_COUNTER: Item<Uint64> = Item::new("id_counter");
pub const ADDRESS_TO_ID: Map<Addr, Uint64> = Map::new("address_to_id");
pub const ID_TO_ADDRESS: Map<u64, Addr> = Map::new("id_to_address");
pub const USER_WALLETS: LookupTable<(u64, Addr)> = Map::new("user_wallets");

/// Initialize contract state data.
pub fn initialize(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  msg: InstantiateMsg,
) -> Result<(), ContractError> {
  ID_COUNTER.save(deps.storage, &Uint64::zero())?;
  OWNER.save(
    deps.storage,
    &msg.owner.unwrap_or(Owner::Address(info.sender.clone())),
  )?;
  Ok(())
}

pub fn require_is_allowed(
  deps: &Deps,
  principal: &Addr,
  action: &str,
) -> Result<(), ContractError> {
  if !match OWNER.load(deps.storage)? {
    Owner::Address(addr) => *principal == addr,
    Owner::Acl(acl_addr) => {
      let acl = Acl::new(&acl_addr);
      acl.is_allowed(&deps.querier, principal, action)?
    },
  } {
    Err(ContractError::NotAuthorized {})
  } else {
    Ok(())
  }
}

pub fn exists_wallet(
  storage: &dyn Storage,
  addr: &Addr,
) -> bool {
  ADDRESS_TO_ID.has(storage, addr.clone())
}

pub fn init_user_id(
  storage: &mut dyn Storage,
  addr: &Addr,
) -> Result<u64, ContractError> {
  if ADDRESS_TO_ID.has(storage, addr.clone()) {
    return Err(ContractError::UserExists);
  }
  let id = ID_COUNTER.update(storage, |n| -> Result<_, ContractError> {
    Ok(n + Uint64::one())
  })?;
  ADDRESS_TO_ID.save(storage, addr.clone(), &id.into())?;
  ID_TO_ADDRESS.save(storage, id.u64(), &addr)?;
  Ok(id.u64())
}

pub fn init_profile(
  storage: &mut dyn Storage,
  id: u64,
  profile: &Profile,
  block: &BlockInfo,
) -> Result<(), ContractError> {
  // TODO: validate profile
  PROFILES.save(storage, id, &profile)?;
  METADATA.save(
    storage,
    id,
    &UserMetadata {
      created_at: block.time,
      updated_at: block.time,
    },
  )?;
  Ok(())
}

pub fn load_wallets(
  storage: &dyn Storage,
  id: u64,
) -> Result<Vec<Addr>, ContractError> {
  Ok(
    USER_WALLETS
      .prefix(id)
      .keys(storage, None, None, Order::Ascending)
      .map(|r| r.unwrap())
      .collect(),
  )
}

pub fn load_user(
  storage: &dyn Storage,
  id: u64,
) -> Result<User, ContractError> {
  Ok(User {
    id: id.into(),
    metadata: METADATA.load(storage, id)?,
    profile: PROFILES.load(storage, id)?,
    wallets: load_wallets(storage, id)?,
  })
}
