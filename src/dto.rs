use crate::domain::Network;
use serde::{
  Deserialize,
  Serialize,
};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct WalletGroupMetadataParam {
  metadata: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
pub struct WalletParam {
  pub encryption_context: String,
  pub network: Network,
  pub wallet_group_id: String,
  pub metadata: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
pub struct RecoveryParam {
  pub wallet_id: String,
  pub encryption_context: String,
  pub recovery_key: String,
}

#[derive(Debug, Serialize)]
pub struct TransSigningParam<T> {
  pub encryption_context: String,
  pub access_key: String,
  pub wallet_id: String,
  pub payload: T,
}

#[derive(Debug, Serialize)]
pub struct EVMPayload {
  pub _type: u16,
  pub chainId: u64,
  pub nonce: u16,
  pub value: String,
  pub gas: u64,
  pub maxFeePerGas: u64,
  pub maxPriorityFeePerGas: u64,
  pub to: String,
}

pub type SolanaPayload = String;

#[derive(Debug, Serialize)]
pub struct BitcoinPayload {
  pub inputs: Vec<AddressInfo>,
  pub outputs: Vec<AddressInfo>,
}

#[derive(Debug, Serialize)]
pub struct AddressInfo {
  pub address: String,
  pub value: u16,
  pub txid: Option<String>,
  pub tx_num: Option<u16>,
}

#[derive(Debug, Serialize)]
pub struct MessageSigningParam {
  pub raw_data_hash: String,
  pub encryption_context: String,
  pub access_key: String,
  pub wallet_id: String,
}

#[derive(Debug, Serialize)]
pub struct SolanaSignMessageParam {
  pub message_base64: String,
  pub encryption_context: String,
  pub access_key: String,
  pub wallet_id: String,
}
