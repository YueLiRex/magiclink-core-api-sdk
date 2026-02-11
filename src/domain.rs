use serde::{
  Deserialize,
  Serialize,
};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletGroup {
  pub uuid: Uuid,
  pub time_created: u64,
  pub time_updated: u64,
  pub metada: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Wallet {
  pub uuid: Uuid,
  pub time_created: u64,
  pub time_updated: u64,
  pub network: Network,
  pub wallet_group_id: Uuid,
  pub key_type: String,
  pub public_address: Option<String>,
  pub recovery_key: String,
  pub access_key: String,
  pub metadata: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Network {
  BtcMainnet,
  BtcTestnet,
  BtcRegtest,

  SolMainnet,
  SolTestnet,

  Evm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Signing {
  pub signed_transaction: String,
  pub transaction_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageSigning {
  pub messageHash: String,
  pub signature: String,
  pub r: String,
  pub s: String,
  pub v: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolanaSignMessage {
  pub signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
  data: T,
}

pub struct ApiError {
  pub status_code: u8,
  pub code: String,
  pub message: String,
  pub details: String,
}
