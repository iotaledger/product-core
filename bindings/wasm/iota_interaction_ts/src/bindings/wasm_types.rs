// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use std::str::FromStr;

use fastcrypto::encoding::{Base64, Encoding};
use fastcrypto::traits::EncodeDecodeBase64 as _;
use iota_interaction::rpc_types::{IotaTransactionBlockEffects, OwnedObjectRef};
use iota_interaction::types::base_types::{IotaAddress, ObjectID, ObjectRef, SequenceNumber};
use iota_interaction::types::crypto::{PublicKey, Signature};
use iota_interaction::types::digests::TransactionDigest;
use iota_interaction::types::execution_status::CommandArgumentError;
use iota_interaction::types::object::Owner;
use iota_interaction::types::transaction::TransactionData;
use iota_interaction::ProgrammableTransactionBcs;
use js_sys::{Promise, Uint8Array};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsError, JsValue};
use wasm_bindgen_futures::JsFuture;

use crate::bindings::WasmIotaClient;
use crate::console_log;
use crate::error::{TsSdkError, WasmError};

// TODO: fix/add signer or remove functions relying on it
type WasmStorageSigner = ();

#[wasm_bindgen(typescript_custom_section)]
const TS_SDK_TYPES: &str = r#"
  import {
    Balance,
    ExecuteTransactionBlockParams,
    GetCoinsParams,
    GetDynamicFieldObjectParams,
    GetObjectParams,
    GetOwnedObjectsParams,
    GetTransactionBlockParams,
    IotaClient,
    IotaObjectData,
    IotaObjectResponse,
    IotaTransactionBlockResponse,
    IotaTransactionBlockResponseOptions,
    ObjectRead,
    PaginatedCoins,
    PaginatedEvents,
    PaginatedObjectsResponse,
    QueryEventsParams,
    TryGetPastObjectParams,
  } from "@iota/iota-sdk/client";
  import { bcs } from "@iota/iota-sdk/bcs";
  import {
    executeTransaction,
    WasmIotaTransactionBlockResponseWrapper,
  } from "./iota_client_helpers"
"#;

#[wasm_bindgen(module = "@iota/iota-sdk/client")]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<Balance>")]
  pub type PromiseBalance;

  #[wasm_bindgen(typescript_type = "TransactionArgument")]
  pub type WasmTransactionArgument;

  #[wasm_bindgen(typescript_type = "IotaObjectData")]
  pub type WasmIotaObjectData;

  #[wasm_bindgen(typescript_type = "ExecuteTransactionBlockParams")]
  #[derive(Clone)]
  pub type WasmExecuteTransactionBlockParams;

  #[wasm_bindgen(typescript_type = "IotaTransactionBlockResponseOptions")]
  #[derive(Clone)]
  pub type WasmIotaTransactionBlockResponseOptions;

  #[wasm_bindgen(typescript_type = "IotaTransactionBlockResponse")]
  #[derive(Clone)]
  pub type WasmIotaTransactionBlockResponse;

  #[wasm_bindgen(typescript_type = "Promise<DryRunTransactionBlockResponse>")]
  pub type PromiseDryRunTransactionBlockResponse;

  #[wasm_bindgen(typescript_type = "DryRunTransactionBlockResponse")]
  #[derive(Clone)]
  pub type WasmDryRunTransactionBlockResponse;

  #[wasm_bindgen(typescript_type = "DryRunTransactionBlockParams")]
  #[derive(Clone)]
  pub type WasmDryRunTransactionBlockParams;

  #[derive(Clone)]
  #[wasm_bindgen(
    typescript_type = "TransactionEffects",
    extends = js_sys::Object,
  )]
  pub type WasmIotaTransactionBlockEffects;

  #[wasm_bindgen(typescript_type = "GetDynamicFieldObjectParams")]
  #[derive(Clone)]
  pub type WasmGetDynamicFieldObjectParams;

  #[wasm_bindgen(typescript_type = "GetObjectParams")]
  #[derive(Clone)]
  pub type WasmGetObjectParams;

  #[wasm_bindgen(typescript_type = "Promise<IotaTransactionBlockResponse>")]
  #[derive(Clone)]
  pub type PromiseIotaTransactionBlockResponse;

  #[wasm_bindgen(typescript_type = "Promise<IotaObjectResponse>")]
  #[derive(Clone)]
  pub type PromiseIotaObjectResponse;

  #[wasm_bindgen(typescript_type = "GetOwnedObjectsParams")]
  #[derive(Clone)]
  pub type WasmGetOwnedObjectsParams;

  #[wasm_bindgen(typescript_type = "GetTransactionBlockParams")]
  #[derive(Clone)]
  pub type WasmGetTransactionBlockParams;

  #[wasm_bindgen(typescript_type = "Promise<PaginatedObjectsResponse>")]
  #[derive(Clone)]
  pub type PromisePaginatedObjectsResponse;

  #[wasm_bindgen(typescript_type = "TryGetPastObjectParams")]
  #[derive(Clone)]
  pub type WasmTryGetPastObjectParams;

  #[wasm_bindgen(typescript_type = "Promise<ObjectRead>")]
  #[derive(Clone)]
  pub type PromiseObjectRead;

  #[wasm_bindgen(typescript_type = "ExecutionStatus")]
  #[derive(Clone)]
  pub type WasmExecutionStatus;

  #[wasm_bindgen(typescript_type = "IotaObjectRef")]
  #[derive(Clone)]
  pub type WasmObjectRef;

  #[wasm_bindgen(method, getter, js_name = objectId)]
  pub fn object_id(this: &WasmObjectRef) -> String;

  #[wasm_bindgen(method, getter, js_name = digest)]
  pub fn digest(this: &WasmObjectRef) -> String;

  #[wasm_bindgen(method, getter, js_name = version)]
  pub fn version(this: &WasmObjectRef) -> String;

  #[wasm_bindgen(typescript_type = "SharedObjectRef")]
  #[derive(Clone)]
  pub type WasmSharedObjectRef;

  #[wasm_bindgen(typescript_type = "OwnedObjectRef")]
  #[derive(Clone)]
  pub type WasmOwnedObjectRef;

  #[wasm_bindgen(typescript_type = "QueryEventsParams")]
  #[derive(Clone)]
  pub type WasmQueryEventsParams;

  #[wasm_bindgen(typescript_type = "Promise<PaginatedEvents>")]
  #[derive(Clone)]
  pub type PromisePaginatedEvents;

  #[wasm_bindgen(typescript_type = "GetCoinsParams")]
  #[derive(Clone)]
  pub type WasmGetCoinsParams;

  #[wasm_bindgen(typescript_type = "Promise<PaginatedCoins>")]
  #[derive(Clone)]
  pub type PromisePaginatedCoins;

  #[wasm_bindgen(typescript_type = "Promise<WasmIotaTransactionBlockResponseWrapper>")]
  #[derive(Clone)]
  pub type PromiseIotaTransactionBlockResponseWrapper;

  #[wasm_bindgen(typescript_type = "Signature")]
  pub type WasmIotaSignature;

  #[wasm_bindgen(typescript_type = "Parameters<IotaClient['waitForTransaction']>")]
  #[derive(Clone, Debug)]
  pub type WasmWaitForTransactionParams;
}

impl From<WasmIotaTransactionBlockEffects> for IotaTransactionBlockEffects {
  fn from(value: WasmIotaTransactionBlockEffects) -> Self {
    serde_wasm_bindgen::from_value(value.into()).expect("have the same repr")
  }
}

impl From<&'_ IotaTransactionBlockEffects> for WasmIotaTransactionBlockEffects {
  fn from(value: &'_ IotaTransactionBlockEffects) -> Self {
    value
      .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
      .expect("same representation")
      .unchecked_into()
  }
}

#[derive(Serialize, Deserialize)]
enum IotaSignatureHelper {
  Ed25519IotaSignature(String),
  Secp256k1IotaSignature(String),
  Secp256r1IotaSignature(String),
}

impl TryFrom<Signature> for WasmIotaSignature {
  type Error = JsValue;
  fn try_from(sig: Signature) -> Result<Self, Self::Error> {
    let base64sig = Base64::encode(&sig);
    let json_signature = match sig {
      Signature::Ed25519IotaSignature(_) => IotaSignatureHelper::Ed25519IotaSignature(base64sig),
      Signature::Secp256r1IotaSignature(_) => IotaSignatureHelper::Secp256r1IotaSignature(base64sig),
      Signature::Secp256k1IotaSignature(_) => IotaSignatureHelper::Secp256k1IotaSignature(base64sig),
    };

    json_signature
      .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
      .map(JsCast::unchecked_into)
      .map_err(|e| e.into())
  }
}

impl TryFrom<WasmIotaSignature> for Signature {
  type Error = JsValue;
  fn try_from(sig: WasmIotaSignature) -> Result<Self, Self::Error> {
    let sig_helper = serde_wasm_bindgen::from_value(sig.into())?;
    let base64sig = match sig_helper {
      IotaSignatureHelper::Ed25519IotaSignature(s) => s,
      IotaSignatureHelper::Secp256k1IotaSignature(s) => s,
      IotaSignatureHelper::Secp256r1IotaSignature(s) => s,
    };

    base64sig
      .parse()
      .map_err(|e: eyre::Report| JsError::new(&e.to_string()).into())
  }
}

#[wasm_bindgen(module = "@iota/iota-sdk/transactions")]
extern "C" {
  #[derive(Clone)]
  #[wasm_bindgen(typescript_type = "Transaction")]
  pub type WasmTransaction;

  #[wasm_bindgen(js_name = "from", js_class = "Transaction", static_method_of = WasmTransaction, catch)]
  pub fn from_bcs_bytes(bytes: Uint8Array) -> Result<WasmTransaction, JsValue>;

  #[wasm_bindgen(method, structural, catch)]
  pub async fn build(
    this: &WasmTransaction,
    options: Option<WasmBuildTransactionOptions>,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(typescript_type = BuildTransactionOptions)]
  pub type WasmBuildTransactionOptions;

  #[derive(Clone)]
  #[wasm_bindgen(typescript_type = "TransactionData")]
  pub type WasmTransactionData;

  #[wasm_bindgen(typescript_type = "Transaction")]
  pub type WasmTransactionBuilder;

  #[wasm_bindgen(js_name = "from", js_class = "Transaction", static_method_of = WasmTransactionBuilder, catch)]
  pub fn from_bcs_bytes(bytes: Uint8Array) -> Result<WasmTransactionBuilder, JsValue>;

  #[wasm_bindgen(method, structural, catch)]
  pub async fn build(this: &WasmTransactionBuilder) -> Result<Uint8Array, JsValue>;

  #[derive(Clone)]
  #[wasm_bindgen(typescript_type = TransactionDataBuilder)]
  pub type WasmTransactionDataBuilder;

  #[wasm_bindgen(
    js_name = fromBytes,
    js_class = TransactionDataBuilder,
    static_method_of = WasmTransactionDataBuilder,
    catch
  )]
  pub fn from_bcs_bytes(bytes: Uint8Array) -> Result<WasmTransactionDataBuilder, JsValue>;

  #[wasm_bindgen(
    js_name = fromKindBytes,
    js_class = TransactionDataBuilder,
    static_method_of = WasmTransactionDataBuilder,
    catch
  )]
  pub fn from_kind_bcs_bytes(bytes: Uint8Array) -> Result<WasmTransactionDataBuilder, JsValue>;

  #[wasm_bindgen(method, catch)]
  pub fn build(this: &WasmTransactionDataBuilder) -> Result<Uint8Array, JsValue>;
  // TODO: decide if we need the following functions: "yagni" or not?

  // #[wasm_bindgen(js_name = "setSender", method, catch)]
  // pub fn set_sender(this: &WasmTransactionBuilder, address: String) -> Result<(), JsValue>;

  // #[wasm_bindgen(js_name = "setGasOwner", method, catch)]
  // pub fn set_gas_owner(this: &WasmTransactionBuilder, address: String) -> Result<(), JsValue>;

  // #[wasm_bindgen(js_name = "setGasPrice", method, catch)]
  // pub fn set_gas_price(this: &WasmTransactionBuilder, price: u64) -> Result<(), JsValue>;

  // #[wasm_bindgen(js_name = "setGasPayment", method, catch)]
  // pub fn set_gas_payment(this: &WasmTransactionBuilder, payments: Vec<WasmObjectRef>) -> Result<(), JsValue>;

  // #[wasm_bindgen(js_name = "setGasBudget", method, catch)]
  // pub fn set_gas_budget(this: &WasmTransactionBuilder, budget: u64) -> Result<(), JsValue>;

  // #[wasm_bindgen(js_name = "getData", method, catch)]
  // pub fn get_data(this: &WasmTransactionBuilder) -> Result<JsValue, JsValue>;
}

impl TryFrom<TransactionData> for WasmTransactionData {
  type Error = serde_wasm_bindgen::Error;
  fn try_from(value: TransactionData) -> Result<Self, Self::Error> {
    let js_value = serde_wasm_bindgen::to_value(&value)?;
    Ok(js_value.unchecked_into())
  }
}

impl TryFrom<WasmTransactionData> for TransactionData {
  type Error = serde_wasm_bindgen::Error;
  fn try_from(value: WasmTransactionData) -> Result<Self, Self::Error> {
    serde_wasm_bindgen::from_value(value.into())
  }
}

#[wasm_bindgen(module = "@iota/iota-sdk/cryptography")]
extern "C" {
  #[derive(Clone)]
  #[wasm_bindgen(typescript_type = PublicKey)]
  pub type WasmPublicKey;

  #[wasm_bindgen(js_name = toIotaPublicKey, method)]
  pub fn to_iota_public_key(this: &WasmPublicKey) -> String;

  #[wasm_bindgen(js_name = toRawBytes, method)]
  pub fn to_raw_bytes(this: &WasmPublicKey) -> Vec<u8>;

  #[wasm_bindgen(js_name = toIotaAddress, method)]
  pub fn to_iota_address(this: &WasmPublicKey) -> String;

  #[wasm_bindgen(method)]
  pub fn flag(this: &WasmPublicKey) -> u8;
}

#[wasm_bindgen(module = "@iota/iota-sdk/keypairs/ed25519")]
extern "C" {
  #[wasm_bindgen(extends = WasmPublicKey)]
  pub type Ed25519PublicKey;

  #[wasm_bindgen(constructor, catch)]
  pub fn new_ed25519_pk(bytes: &[u8]) -> Result<Ed25519PublicKey, JsValue>;
}

#[wasm_bindgen(module = "@iota/iota-sdk/keypairs/secp256r1")]
extern "C" {
  #[wasm_bindgen(extends = WasmPublicKey)]
  pub type Secp256r1PublicKey;

  #[wasm_bindgen(constructor, catch)]
  pub fn new_secp256r1_pk(bytes: &[u8]) -> Result<Secp256r1PublicKey, JsValue>;
}

#[wasm_bindgen(module = "@iota/iota-sdk/keypairs/secp256k1")]
extern "C" {
  #[wasm_bindgen(extends = WasmPublicKey)]
  pub type Secp256k1PublicKey;

  #[wasm_bindgen(constructor, catch)]
  pub fn new_secp256k1_pk(bytes: &[u8]) -> Result<Secp256k1PublicKey, JsValue>;
}

impl TryFrom<&'_ PublicKey> for WasmPublicKey {
  type Error = JsValue;
  fn try_from(pk: &PublicKey) -> Result<Self, Self::Error> {
    let pk_bytes = pk.as_ref();
    let wasm_pk: WasmPublicKey = match pk {
      PublicKey::Ed25519(_) => Ed25519PublicKey::new_ed25519_pk(pk_bytes)?.into(),
      PublicKey::Secp256r1(_) => Secp256r1PublicKey::new_secp256r1_pk(pk_bytes)?.into(),
      PublicKey::Secp256k1(_) => Secp256k1PublicKey::new_secp256k1_pk(pk_bytes)?.into(),
      _ => return Err(JsError::new("unsupported PublicKey type").into()),
    };

    assert_eq!(pk_bytes, &wasm_pk.to_raw_bytes());
    assert_eq!(
      IotaAddress::from(pk),
      wasm_pk.to_iota_address().parse().expect("valid iota address")
    );

    Ok(wasm_pk)
  }
}

impl TryFrom<WasmPublicKey> for PublicKey {
  type Error = JsValue;
  fn try_from(wasm_pk: WasmPublicKey) -> Result<Self, Self::Error> {
    let pk = PublicKey::decode_base64(&wasm_pk.to_iota_public_key())
      .map_err(|_| JsError::new("failed to decode base64 JS PublicKey"))?;

    assert_eq!(&wasm_pk.to_raw_bytes(), pk.as_ref());
    assert_eq!(
      IotaAddress::from(&pk),
      wasm_pk.to_iota_address().parse().expect("valid iota address")
    );

    Ok(pk)
  }
}

impl TryFrom<WasmObjectRef> for ObjectRef {
  type Error = anyhow::Error;
  fn try_from(value: WasmObjectRef) -> Result<Self, Self::Error> {
    let digest = serde_json::from_value(serde_json::Value::String(value.digest()))?;
    let version = {
      let version_number = serde_json::Number::from_str(&value.version())?;
      serde_json::from_value(serde_json::Value::Number(version_number))?
    };
    let object_id = value.object_id().parse()?;

    Ok((object_id, version, digest))
  }
}

impl From<ObjectRef> for WasmObjectRef {
  fn from(value: ObjectRef) -> Self {
    let json_obj = serde_json::json!({
      "objectId": value.0,
      "version": value.1,
      "digest": value.2,
    });

    json_obj
      .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
      .expect("a JSON object is a JS value")
      // safety: `json_obj` was constructed following TS ObjectRef's interface.
      .unchecked_into()
  }
}

impl From<(ObjectID, SequenceNumber, bool)> for WasmSharedObjectRef {
  fn from(value: (ObjectID, SequenceNumber, bool)) -> Self {
    let json_obj = serde_json::json!({
      "objectId": value.0,
      "initialSharedVersion": value.1,
      "mutable": value.2,
    });

    json_obj
      .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
      .expect("a JSON object is a JS value")
      // safety: `json_obj` was constructed following TS SharedObjectRef's interface.
      .unchecked_into()
  }
}

impl TryFrom<OwnedObjectRef> for WasmSharedObjectRef {
  type Error = TsSdkError;
  fn try_from(value: OwnedObjectRef) -> Result<Self, Self::Error> {
    let Owner::Shared { initial_shared_version } = value.owner else {
      return Err(TsSdkError::CommandArgumentError(CommandArgumentError::TypeMismatch));
    };
    let obj_id = value.object_id();

    Ok((obj_id, initial_shared_version, true).into())
  }
}

impl WasmSharedObjectRef {
  #[allow(dead_code)]
  pub(crate) fn immutable(self) -> Self {
    const JS_FALSE: JsValue = JsValue::from_bool(false);

    let _ = js_sys::Reflect::set(&self, &JsValue::from_str("mutable"), &JS_FALSE);
    self
  }
}

#[wasm_bindgen(module = "@iota/iota-interaction-ts/iota_client_helpers")]
extern "C" {
  // Please note: For unclear reasons the `typescript_type` name and the `pub type` name defined
  // in wasm_bindgen extern "C" scopes must be equal. Otherwise, the JS constructor will not be
  // found in the generated js code.
  #[wasm_bindgen(typescript_type = "WasmIotaTransactionBlockResponseWrapper")]
  #[derive(Clone)]
  pub type WasmIotaTransactionBlockResponseWrapper;

  #[wasm_bindgen(constructor)]
  pub fn new(response: WasmIotaTransactionBlockResponse) -> WasmIotaTransactionBlockResponseWrapper;

  #[wasm_bindgen(method, js_name = get_effects)]
  pub fn effects(this: &WasmIotaTransactionBlockResponseWrapper) -> Option<WasmIotaTransactionBlockEffects>;

  #[wasm_bindgen(method)]
  pub fn to_string(this: &WasmIotaTransactionBlockResponseWrapper) -> String;

  #[wasm_bindgen(method, js_name = "get_digest")]
  fn digest_inner(this: &WasmIotaTransactionBlockResponseWrapper) -> String;

  #[wasm_bindgen(method, js_name = "get_response")]
  pub fn response(this: &WasmIotaTransactionBlockResponseWrapper) -> WasmIotaTransactionBlockResponse;

  #[wasm_bindgen(js_name = executeTransaction)]
  fn execute_transaction_inner(
    iota_client: &WasmIotaClient, // --> TypeScript: IotaClient
    sender_address: String,       // --> TypeScript: string
    tx_bcs: Vec<u8>,              // --> TypeScript: Uint8Array,
    signer: WasmStorageSigner,    // --> TypeScript: Signer (iota_client_helpers module)
    gas_budget: Option<u64>,      // --> TypeScript: optional bigint
  ) -> PromiseIotaTransactionBlockResponseWrapper;

  #[wasm_bindgen(js_name = "sleep")]
  fn sleep_inner(ms: i32) -> Promise;
}

///  Helper function to pause execution.
pub async fn sleep(duration_ms: i32) -> Result<(), JsValue> {
  let promise = sleep_inner(duration_ms);
  let js_fut = JsFuture::from(promise);
  js_fut.await?;
  Ok(())
}

impl WasmIotaTransactionBlockResponseWrapper {
  pub fn digest(&self) -> Result<TransactionDigest, TsSdkError> {
    TransactionDigest::from_str(&self.digest_inner())
      .map_err(|err| TsSdkError::WasmError("Failed to parse transaction block digest".to_string(), err.to_string()))
  }
}

pub async fn execute_transaction(
  iota_client: &WasmIotaClient,       // --> Binding: WasmIotaClient
  sender_address: IotaAddress,        // --> Binding: String
  tx_bcs: ProgrammableTransactionBcs, // --> Binding: Vec<u8>
  signer: WasmStorageSigner,          // --> Binding: WasmStorageSigner
  gas_budget: Option<u64>,            // --> Binding: Option<u64>,
) -> Result<WasmIotaTransactionBlockResponseWrapper, TsSdkError> {
  let promise: Promise = Promise::resolve(&execute_transaction_inner(
    iota_client,
    sender_address.to_string(),
    tx_bcs,
    signer,
    gas_budget,
  ));
  let result: JsValue = JsFuture::from(promise).await.map_err(|e| {
    let message = "Error executing JsFuture::from(promise) for `execute_transaction`";
    let details = format!("{e:?}");
    console_log!("{message}; {details}");
    TsSdkError::WasmError(message.to_string(), details.to_string())
  })?;

  Ok(WasmIotaTransactionBlockResponseWrapper::new(result.into()))
}

#[derive(Deserialize)]
#[serde(try_from = "Vec<u8>")]
pub struct ProgrammableTransaction(#[allow(dead_code)] pub(crate) WasmTransactionBuilder);
impl TryFrom<Vec<u8>> for ProgrammableTransaction {
  type Error = TsSdkError;
  fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
    let uint8array: Uint8Array = value.as_slice().into();
    WasmTransactionBuilder::from_bcs_bytes(uint8array)
      .map(Self)
      .map_err(WasmError::from)
      .map_err(TsSdkError::from)
  }
}
