use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp, Uint64};

#[cw_serde]
pub struct UserProfile {
  pub username: Option<String>,
  pub description: Option<String>,
  pub location: Option<UserLocation>,
  pub image_url: Option<String>,
  pub cover_image_url: Option<String>,
  pub title: Option<String>,
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub email: Option<String>,
  pub phone: Option<String>,
  pub website: Option<String>,
  pub color: Option<String>,
  pub socials: Option<Vec<SocialMediaId>>,
}

#[cw_serde]
pub struct UserMetadata {
  pub created_at: Timestamp,
  pub updated_at: Timestamp,
}

#[cw_serde]
pub struct UserLocation {
  pub address_line_1: Option<String>,
  pub address_line_2: Option<String>,
  pub postal_code: Option<String>,
  pub country: Option<String>,
  pub region: Option<String>,
  pub city: Option<String>,
}

#[cw_serde]
pub enum SocialMediaId {
  Linkedin(String),
  Twitter(String),
  Instagram(String),
  Telegram(String),
  Discord(String),
  Facebook(String),
  Tiktok(String),
}

#[cw_serde]
pub struct UserConfig {
  pub session_timeout_seconds: Uint64,
}

#[cw_serde]
pub struct UserSession {
  pub user_id: Uint64,
  pub address: Addr,
  pub time: Timestamp,
  pub height: Uint64,
  pub refresh_time: Timestamp,
  pub refresh_height: Uint64,
}

#[cw_serde]
pub struct User {
  pub id: Uint64,
  pub metadata: UserMetadata,
  pub profile: UserProfile,
  pub wallets: Vec<Addr>,
  pub config: UserConfig,
}
