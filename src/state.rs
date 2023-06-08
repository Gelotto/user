use crate::models::{User, UserConfig, UserMetadata, UserSession};
use crate::msg::InstantiateMsg;
use crate::{error::ContractError, models::UserProfile};
use cosmwasm_std::{Addr, BlockInfo, Deps, DepsMut, Env, MessageInfo, Order, Storage, Uint64};
use cw_acl::client::Acl;
use cw_lib::models::Owner;
use cw_storage_plus::{Item, Map};
use sha2::{Digest, Sha512};

pub type LookupTable<'a, T> = Map<'a, T, bool>;
pub type Hash = [u8; 64];

pub const DEFAULT_SESSION_TIMEOUT_SECONDS: u64 = (60 * 60 * 48) as u64;

pub const OWNER: Item<Owner> = Item::new("owner");
pub const USER_PROFILES: Map<u64, UserProfile> = Map::new("profiles");
pub const USER_METADATA: Map<u64, UserMetadata> = Map::new("metadata");
pub const USER_SESSION_TIMEOUTS: Map<u64, Uint64> = Map::new("session_timeouts");
pub const USER_SESSIONS: Map<String, UserSession> = Map::new("sessions");
pub const ADDRESS_TO_ID: Map<Addr, Uint64> = Map::new("address_to_id");
pub const ID_2_ADDRESS: LookupTable<(u64, Addr)> = Map::new("id_2_addr");
pub const ID_COUNTER: Item<Uint64> = Item::new("id_counter");
pub const USER_COUNT: Item<u32> = Item::new("user_count");

/// Initialize contract state data.
pub fn initialize(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  msg: InstantiateMsg,
) -> Result<(), ContractError> {
  ID_COUNTER.save(deps.storage, &Uint64::zero())?;
  USER_COUNT.save(deps.storage, &0)?;
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

pub fn load_user_id(
  storage: &dyn Storage,
  addr: &Addr,
) -> Result<u64, ContractError> {
  if let Some(id) = ADDRESS_TO_ID.may_load(storage, addr.clone())? {
    Ok(id.u64())
  } else {
    Err(ContractError::UserNotFound)
  }
}

pub fn exists_wallet(
  storage: &dyn Storage,
  addr: &Addr,
) -> bool {
  ADDRESS_TO_ID.has(storage, addr.clone())
}

pub fn require_valid_session(
  storage: &dyn Storage,
  block: &BlockInfo,
  user_id: u64,
  key: &String,
) -> Result<UserSession, ContractError> {
  if let Some(session) = USER_SESSIONS.may_load(storage, key.clone())? {
    let interval = USER_SESSION_TIMEOUTS.load(storage, user_id)?;
    let expires_at = session.refresh_time.seconds() + interval.u64();
    if block.time.seconds() > expires_at {
      return Err(ContractError::SessionExpired);
    } else {
      return Ok(session);
    }
  }
  Err(ContractError::SessionNotFound)
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
  ID_2_ADDRESS.save(storage, (id.u64(), addr.clone()), &true)?;
  USER_COUNT.update(storage, |n| -> Result<_, ContractError> { Ok(n + 1) })?;
  Ok(id.u64())
}

pub fn init_user(
  storage: &mut dyn Storage,
  id: u64,
  profile: &UserProfile,
  block: &BlockInfo,
) -> Result<(), ContractError> {
  // TODO: validate profile
  USER_PROFILES.save(storage, id, &profile)?;
  USER_SESSION_TIMEOUTS.save(storage, id, &DEFAULT_SESSION_TIMEOUT_SECONDS.into())?;
  USER_METADATA.save(
    storage,
    id,
    &UserMetadata {
      created_at: block.time,
      updated_at: block.time,
    },
  )?;
  Ok(())
}

pub fn build_session_key(
  addr: &Addr,
  user_id: u64,
  seed: &String,
) -> String {
  let mut hasher = Sha512::new();
  hasher.update(user_id.to_le_bytes());
  hasher.update(addr.as_bytes());
  hasher.update(seed.as_bytes());
  hex::encode(hasher.finalize().to_ascii_uppercase().as_slice())
}

pub fn init_session(
  storage: &mut dyn Storage,
  block: &BlockInfo,
  address: &Addr,
  user_id: u64,
  key: &String,
) -> Result<UserSession, ContractError> {
  Ok(USER_SESSIONS.update(
    storage,
    key.clone(),
    |maybe_session| -> Result<_, ContractError> {
      if maybe_session.is_some() {
        Err(ContractError::NotAuthorized)
      } else {
        Ok(UserSession {
          user_id: user_id.into(),
          address: address.clone(),
          time: block.time,
          height: block.height.into(),
          refresh_time: block.time,
          refresh_height: block.height.into(),
        })
      }
    },
  )?)
}

pub fn end_session(
  storage: &mut dyn Storage,
  key: &String,
) -> Result<(), ContractError> {
  USER_SESSIONS.remove(storage, key.clone());
  Ok(())
}

pub fn load_wallets(
  storage: &dyn Storage,
  id: u64,
) -> Result<Vec<Addr>, ContractError> {
  Ok(
    ID_2_ADDRESS
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
    metadata: USER_METADATA.load(storage, id)?,
    profile: USER_PROFILES.load(storage, id)?,
    wallets: load_wallets(storage, id)?,
    config: UserConfig {
      session_timeout_seconds: USER_SESSION_TIMEOUTS.load(storage, id)?,
    },
  })
}
