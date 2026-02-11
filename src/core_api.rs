use crate::{
  domain::{
    MessageSigning,
    Signing,
    SolanaSignMessage,
    Wallet,
    WalletGroup,
  },
  dto::{
    ApiResponse,
    BitcoinPayload,
    EVMPayload,
    MessageSigningParam,
    RecoveryParam,
    SolanaPayload,
    SolanaSignMessageParam,
    TransSigningParam,
    WalletGroupMetadataParam,
    WalletParam,
  },
};
use reqwest::header::{
  HeaderMap,
  HeaderValue,
};
use reqwest_middleware::{
  ClientBuilder,
  ClientWithMiddleware,
  Error,
};
use reqwest_tracing::TracingMiddleware;
use std::{
  env,
  future::Future,
};

static BASE_URI: &str = "https://tee.magiclabs.com/v1/api";

pub struct CoreApi {
  client: ClientWithMiddleware,
  api_secret: String,
}

impl Default for CoreApi {
  fn default() -> Self {
    dotenv::dotenv().ok();

    let client: ClientWithMiddleware = ClientBuilder::new(reqwest::Client::new())
      .with(TracingMiddleware::default())
      .build();

    let api_secret = env::var("MAGIC_LINK_API_SECRET").expect("MAGIC_LINK_API_SECRET is missing");

    Self { client, api_secret }
  }
}

impl CoreApi {
  pub fn new(client: ClientWithMiddleware, api_secret: String) -> Self {
    Self { client, api_secret }
  }

  fn default_headers(&self) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.append(
      "content-type",
      HeaderValue::from_str("application/json").expect("Expect applicaton/json as content-type"),
    );
    headers.append(
      "x-magic-secret-key",
      HeaderValue::from_str(self.api_secret.as_str()).expect("Set api secret to header"),
    );
    headers
  }
}

pub trait WalletApi {
  fn create_wallet_group(
    &self,
    metadata: &WalletGroupMetadataParam,
  ) -> impl Future<Output = Result<ApiResponse<WalletGroup>, Error>>;
  fn wallet_group_list(&self) -> impl Future<Output = Result<ApiResponse<Vec<WalletGroup>>, Error>>;
  fn create_wallet(&self, wallet_param: &WalletParam) -> impl Future<Output = Result<ApiResponse<Wallet>, Error>>;
  fn recover_wallet(&self, recovery_param: &RecoveryParam) -> impl Future<Output = Result<ApiResponse<Wallet>, Error>>;
}

pub trait SigningApi {
  fn evm_transaction_signing(
    &self,
    trans_sign_param: &TransSigningParam<EVMPayload>,
  ) -> impl Future<Output = Result<ApiResponse<Signing>, Error>>;
  fn solana_transaction_signing(
    &self,
    trans_sign_param: &TransSigningParam<SolanaPayload>,
  ) -> impl Future<Output = Result<ApiResponse<Signing>, Error>>;
  fn bitcoin_transaction_signing(
    &self,
    trans_sign_param: &TransSigningParam<BitcoinPayload>,
  ) -> impl Future<Output = Result<ApiResponse<Signing>, Error>>;

  fn message_signing(
    &self,
    message_sign_param: &MessageSigningParam,
  ) -> impl Future<Output = Result<ApiResponse<MessageSigning>, Error>>;
  fn solana_message_signing(
    &self,
    solana_sign_message_param: &SolanaSignMessageParam,
  ) -> impl Future<Output = Result<ApiResponse<SolanaSignMessage>, Error>>;
}

impl WalletApi for CoreApi {
  async fn create_wallet_group(&self, metadata: &WalletGroupMetadataParam) -> Result<ApiResponse<WalletGroup>, Error> {
    let url: String = format!("{}/wallet_group", BASE_URI);
    self
      .client
      .post(url)
      .headers(self.default_headers())
      .json(metadata)
      .send()
      .await?
      .json::<ApiResponse<WalletGroup>>()
      .await
      .map_err(|e| Error::Reqwest(e))
  }

  async fn wallet_group_list(&self) -> Result<ApiResponse<Vec<WalletGroup>>, Error> {
    let url: String = format!("{}/wallet_groups", BASE_URI);
    self
      .client
      .get(url)
      .headers(self.default_headers())
      .send()
      .await?
      .json::<ApiResponse<Vec<WalletGroup>>>()
      .await
      .map_err(|e| Error::Reqwest(e))
  }

  async fn create_wallet(&self, wallet_param: &WalletParam) -> Result<ApiResponse<Wallet>, Error> {
    let url: String = format!("{}/wallet", BASE_URI);
    self
      .client
      .post(url)
      .headers(self.default_headers())
      .json(wallet_param)
      .send()
      .await?
      .json::<ApiResponse<Wallet>>()
      .await
      .map_err(|e| Error::Reqwest(e))
  }

  async fn recover_wallet(&self, recovery_param: &RecoveryParam) -> Result<ApiResponse<Wallet>, Error> {
    let url: String = format!("{}/wallet/recovery/confirm", BASE_URI);
    self
      .client
      .post(url)
      .headers(self.default_headers())
      .json(recovery_param)
      .send()
      .await?
      .json::<ApiResponse<Wallet>>()
      .await
      .map_err(|e| Error::Reqwest(e))
  }
}

impl SigningApi for CoreApi {
  async fn evm_transaction_signing(
    &self,
    evm_sign_param: &TransSigningParam<EVMPayload>,
  ) -> Result<ApiResponse<Signing>, Error> {
    let url = format!("{}/wallet/sign_transaction", BASE_URI);
    self
      .client
      .post(url)
      .headers(self.default_headers())
      .json(evm_sign_param)
      .send()
      .await?
      .json::<ApiResponse<Signing>>()
      .await
      .map_err(|e| Error::Reqwest(e))
  }

  async fn solana_transaction_signing(
    &self,
    solana_sign_param: &TransSigningParam<SolanaPayload>,
  ) -> Result<ApiResponse<Signing>, Error> {
    let url = format!("{}/wallet/sign_transaction", BASE_URI);
    self
      .client
      .post(url)
      .headers(self.default_headers())
      .json(solana_sign_param)
      .send()
      .await?
      .json::<ApiResponse<Signing>>()
      .await
      .map_err(|e| Error::Reqwest(e))
  }

  async fn bitcoin_transaction_signing(
    &self,
    bitcoin_sign_param: &TransSigningParam<BitcoinPayload>,
  ) -> Result<ApiResponse<Signing>, Error> {
    let url = format!("{}/wallet/sign_transaction", BASE_URI);
    self
      .client
      .post(url)
      .headers(self.default_headers())
      .json(bitcoin_sign_param)
      .send()
      .await?
      .json::<ApiResponse<Signing>>()
      .await
      .map_err(|e| Error::Reqwest(e))
  }

  async fn message_signing(
    &self,
    message_sign_param: &MessageSigningParam,
  ) -> Result<ApiResponse<MessageSigning>, Error> {
    let url = format!("{}/wallet/sign_data", BASE_URI);
    self
      .client
      .post(url)
      .headers(self.default_headers())
      .json(message_sign_param)
      .send()
      .await?
      .json::<ApiResponse<MessageSigning>>()
      .await
      .map_err(|e| Error::Reqwest(e))
  }

  async fn solana_message_signing(
    &self,
    solana_sign_message_param: &SolanaSignMessageParam,
  ) -> Result<ApiResponse<SolanaSignMessage>, Error> {
    let url = format!("{}/wallet/svm/sign_message", BASE_URI);
    self
      .client
      .post(url)
      .headers(self.default_headers())
      .json(solana_sign_message_param)
      .send()
      .await?
      .json::<ApiResponse<SolanaSignMessage>>()
      .await
      .map_err(|e| Error::Reqwest(e))
  }
}
