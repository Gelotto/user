use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp, Uint64};

#[cw_serde]
pub struct Profile {
  pub username: Option<String>,
  pub description: Option<String>,
  pub location: Option<Geolocation>,
  pub image_url: Option<String>,
  pub cover_image_url: Option<String>,
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
pub struct Geolocation {
  pub street_address: Option<String>,
  pub unit: Option<String>,
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
pub struct User {
  pub id: Uint64,
  pub metadata: UserMetadata,
  pub profile: Profile,
  pub wallets: Vec<Addr>,
}
