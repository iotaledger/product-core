// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::result::Result as StdResult;

use anyhow::{anyhow, Context as _};
use async_trait::async_trait;
use fastcrypto::traits::EncodeDecodeBase64;
use iota_interaction::rpc_types::IotaTransactionBlockEffects;
use iota_interaction::types::crypto::Signature;
use iota_interaction::types::transaction::{ProgrammableTransaction, TransactionData, TransactionDataAPI as _};
use iota_interaction_ts::bindings::{
  WasmIotaTransactionBlockEffects, WasmIotaTransactionBlockResponse, WasmObjectRef, WasmTransactionDataBuilder,
};
use iota_interaction_ts::core_client::{WasmCoreClient, WasmCoreClientReadOnly};
use iota_interaction_ts::error::{Result, WasmResult as _};
use js_sys::JsString;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast as _, JsValue};
use wasm_bindgen_futures::JsFuture;

use crate::bindings::core_client::{WasmManagedCoreClient, WasmManagedCoreClientReadOnly};
use crate::core_client::CoreClientReadOnly;
use crate::transaction::transaction_builder::{MutGasDataRef, Transaction, TransactionBuilder};
use crate::transaction::TransactionOutputInternal;

#[wasm_bindgen]
extern "C" {
  #[derive(Clone)]
  #[wasm_bindgen(typescript_type = "Transaction<unknown>")]
  pub type WasmTransaction;

  #[wasm_bindgen(method, catch, js_name = buildProgrammableTransaction)]
  pub async fn build_programmable_transaction(
    this: &WasmTransaction,
    client: &WasmCoreClientReadOnly,
  ) -> Result<js_sys::Uint8Array>;
  #[wasm_bindgen(method, catch)]
  pub async fn apply(
    this: &WasmTransaction,
    effects: &WasmIotaTransactionBlockEffects,
    client: &WasmCoreClientReadOnly,
  ) -> Result<JsValue>;
}

#[async_trait(?Send)]
impl Transaction for WasmTransaction {
  type Output = JsValue;
  type Error = crate::Error;

  async fn build_programmable_transaction<C>(&self, client: &C) -> StdResult<ProgrammableTransaction, Self::Error>
  where
    C: CoreClientReadOnly,
  {
    let managed_client = WasmManagedCoreClientReadOnly::from_rust(client);
    let core_client = managed_client.into_wasm();
    let pt_bcs = Self::build_programmable_transaction(self, &core_client)
      .await
      .map_err(|e| {
        crate::Error::TransactionBuildingFailed(format!("failed to build programmable transaction from WASM: {e:?}"))
      })?
      .to_vec();

    bcs::from_bytes(&pt_bcs).map_err(|e| crate::Error::TransactionBuildingFailed(e.to_string()))
  }

  async fn apply<C>(self, effects: &mut IotaTransactionBlockEffects, client: &C) -> StdResult<Self::Output, Self::Error>
  where
    C: CoreClientReadOnly,
  {
    let managed_client = WasmManagedCoreClientReadOnly::from_rust(client);
    let core_client = managed_client.into_wasm();
    let wasm_effects = WasmIotaTransactionBlockEffects::from(&*effects);

    Self::apply(&self, &wasm_effects, &core_client)
      .await
      .map_err(|e| crate::Error::Transaction(anyhow!("failed to apply effects from WASM Transaction: {e:?}").into()))
  }
}

#[wasm_bindgen(js_name = TransactionBuilder, skip_typescript)]
pub struct WasmTransactionBuilder(pub(crate) TransactionBuilder<WasmTransaction>);

#[wasm_bindgen(js_class = TransactionBuilder)]
impl WasmTransactionBuilder {
  #[wasm_bindgen(constructor)]
  pub fn new(tx: WasmTransaction) -> Self {
    Self(TransactionBuilder::new(tx))
  }

  #[wasm_bindgen(getter)]
  pub fn transaction(&self) -> WasmTransaction {
    self.0.as_ref().clone()
  }

  #[wasm_bindgen(js_name = withGasPrice)]
  pub fn with_gas_price(mut self, price: u64) -> Self {
    self.0 = self.0.with_gas_price(price);
    self
  }

  #[wasm_bindgen(js_name = withGasBudget)]
  pub fn with_gas_budget(mut self, budget: u64) -> Self {
    self.0 = self.0.with_gas_budget(budget);
    self
  }

  #[wasm_bindgen(js_name = withGasOwner)]
  pub fn with_gas_owner(mut self, owner: &str) -> Result<Self> {
    let owner = owner.parse().wasm_result()?;
    self.0 = self.0.with_gas_owner(owner);
    Ok(self)
  }

  #[wasm_bindgen(js_name = withGasPayment)]
  pub fn with_gas_payment(mut self, payment: Vec<WasmObjectRef>) -> Result<Self> {
    let payment = payment
      .into_iter()
      .map(TryInto::try_into)
      .collect::<anyhow::Result<Vec<_>>>()
      .wasm_result()?;

    self.0 = self.0.with_gas_payment(payment);
    Ok(self)
  }

  #[wasm_bindgen(js_name = withSender)]
  pub fn with_sender(mut self, sender: &str) -> Result<Self> {
    let sender = sender
      .parse()
      .map_err(|e| anyhow!("failed to parse IotaAddress: {e}"))
      .wasm_result()?;
    self.0 = self.0.with_sender(sender);
    Ok(self)
  }

  #[wasm_bindgen(js_name = withSignature)]
  pub async fn with_signature(mut self, client: &WasmCoreClient) -> Result<Self> {
    let managed_client = WasmManagedCoreClient::from_wasm(client)?;
    self.0 = self.0.with_signature(&managed_client).await.wasm_result()?;
    Ok(self)
  }

  #[wasm_bindgen(js_name = withSponsor)]
  pub async fn with_sponsor(
    mut self,
    client: &WasmCoreClientReadOnly,
    #[wasm_bindgen(unchecked_param_type = "SponsorFn")] sponsor_fn: &js_sys::Function,
  ) -> Result<Self> {
    let closure = async |mut tx_data_ref: MutGasDataRef<'_>| -> anyhow::Result<Signature> {
      let tx_data_bcs = bcs::to_bytes(&*tx_data_ref)?;
      let wasm_tx = WasmTransactionDataBuilder::from_bcs_bytes(js_sys::Uint8Array::from(tx_data_bcs.as_slice()))
        .map_err(|_| anyhow!("failed to convert TransactionData into JS IotaTransaction"))?;
      let promise: js_sys::Promise = sponsor_fn
        .call1(&JsValue::NULL, &wasm_tx)
        .and_then(|value| value.dyn_into())
        .map_err(|_| anyhow!("failed to call JS closure"))?;
      let sig_str: JsString = JsFuture::from(promise)
        .await
        .and_then(|value| value.dyn_into())
        .map_err(|_| anyhow!("failed to build a Future from a JS Promise"))?;

      let modified_tx_data_bcs = wasm_tx
        .build()
        .map_err(|_| anyhow!("failed to build JS TransactionDataBuilder"))?
        .to_vec();
      let tx_data = bcs::from_bytes::<TransactionData>(&modified_tx_data_bcs)?;

      *tx_data_ref.gas_data_mut() = tx_data.gas_data().clone();
      let signature = Signature::decode_base64(&String::from(sig_str)).context("failed to decode b64 signature")?;

      Ok(signature)
    };

    let managed_client = WasmManagedCoreClientReadOnly::from_wasm(client)?;
    self.0 = self.0.with_sponsor(&managed_client, closure).await.wasm_result()?;
    Ok(self)
  }

  #[wasm_bindgen(unchecked_return_type = "[Uint8Array, string[], Transaction]")]
  pub async fn build(self, client: &WasmCoreClient) -> Result<JsValue> {
    let managed_client = WasmManagedCoreClient::from_wasm(client)?;
    self
      .0
      .build(&managed_client)
      .await
      .wasm_result()
      .and_then(tx_parts_to_js)
  }

  #[wasm_bindgen(unchecked_return_type = "[Uint8Array, string[], Transaction]")]
  pub async fn build_with_defaults(self, client: &WasmCoreClientReadOnly) -> Result<JsValue> {
    let managed_client = WasmManagedCoreClientReadOnly::from_wasm(client)?;
    self
      .0
      .build_with_defaults(&managed_client)
      .await
      .wasm_result()
      .and_then(tx_parts_to_js)
  }

  #[wasm_bindgen(js_name = buildAndExecute, unchecked_return_type = "TransactionOutput<unknown>")]
  pub async fn build_and_execute(self, client: &WasmCoreClient) -> Result<WasmTransactionOutput> {
    let managed_client = WasmManagedCoreClient::from_wasm(client)?;
    self
      .0
      .build_and_execute(&managed_client)
      .await
      .wasm_result()
      .map(Into::into)
  }
}

#[wasm_bindgen(
  js_name = TransactionOutput,
  skip_typescript,
  inspectable,
  getter_with_clone
)]
pub struct WasmTransactionOutput {
  pub output: JsValue,
  pub response: WasmIotaTransactionBlockResponse,
}

impl From<TransactionOutputInternal<JsValue>> for WasmTransactionOutput {
  fn from(value: TransactionOutputInternal<JsValue>) -> Self {
    Self {
      output: value.output,
      response: value.response.clone_native_response().response(),
    }
  }
}

fn tx_parts_to_js((tx_data, signatures, tx): (TransactionData, Vec<Signature>, WasmTransaction)) -> Result<JsValue> {
  let tx_data_bcs = bcs::to_bytes(&tx_data)
    .wasm_result()
    .map(|bcs_bytes| js_sys::Uint8Array::from(bcs_bytes.as_slice()))?;
  let wasm_signatures = {
    let wasm_signatures = js_sys::Array::new();
    for sig in signatures {
      let b64_sig = sig.encode_base64();
      wasm_signatures.push(&JsValue::from_str(&b64_sig));
    }

    wasm_signatures
  };

  let wasm_triple = js_sys::Array::new();
  wasm_triple.push(&tx_data_bcs);
  wasm_triple.push(wasm_signatures.as_ref());
  wasm_triple.push(tx.as_ref());

  Ok(wasm_triple.into())
}
