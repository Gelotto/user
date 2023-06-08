use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContractError {
  #[error("{0}")]
  Std(#[from] StdError),

  #[error("NotAuthorized")]
  NotAuthorized,

  #[error("UserExists")]
  UserExists,

  #[error("UserNotFound")]
  UserNotFound,

  #[error("SessionExpired")]
  SessionExpired,

  #[error("SessionNotFound")]
  SessionNotFound,
}
