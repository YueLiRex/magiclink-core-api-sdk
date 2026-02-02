use crate::domain::Network;
use serde::{
  Deserialize,
  Serialize,
};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
  data: T,
}

#[derive(Debug, Serialize)]
pub struct WalletGroupMetadataParam {
  metadata: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
pub struct WalletParam {
  encryption_context: String,
  network: Network,
  wallet_group_id: String,
  metadata: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
pub struct RecoveryParam {
  wallet_id: String,
  encryption_context: String,
  recovery_key: String,
}

#[derive(Debug, Serialize)]
pub struct TransSigningParam<T> {
  encryption_context: String,
  access_key: String,
  wallet_id: String,
  payload: T,
}

#[derive(Debug, Serialize)]
pub struct EVMPayload {
  _type: u16,
  chainId: u64,
  nonce: u16,
  value: String,
  gas: u64,
  maxFeePerGas: u64,
  maxPriorityFeePerGas: u64,
  to: String,
}

pub type SolanaPayload = String;

#[derive(Debug, Serialize)]
pub struct BitcoinPayload {
  inputs: Vec<AddressInfo>,
  outputs: Vec<AddressInfo>,
}

#[derive(Debug, Serialize)]
pub struct AddressInfo {
  address: String,
  value: u16,
  txid: Option<String>,
  tx_num: Option<u16>,
}

#[derive(Debug, Serialize)]
pub struct MessageSigningParam {
  raw_data_hash: String,
  encryption_context: String,
  access_key: String,
  wallet_id: String,
}

#[derive(Debug, Serialize)]
pub struct SolanaSignMessageParam {
  message_base64: String,
  encryption_context: String,
  access_key: String,
  wallet_id: String,
}
