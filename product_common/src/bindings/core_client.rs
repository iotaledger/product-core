// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_interaction::types::base_types::{IotaAddress, ObjectID, ObjectIDParseError};
use iota_interaction::types::crypto::PublicKey;
use iota_interaction_ts::bindings::{WasmIotaClient, WasmTransactionSigner};
use iota_interaction_ts::core_client::{WasmCoreClient, WasmCoreClientReadOnly};
use iota_interaction_ts::error::{Result, WasmResult};
use iota_interaction_ts::{IotaClientAdapter, WasmPublicKey};
use wasm_bindgen::prelude::*;

use crate::core_client::{CoreClient, CoreClientReadOnly};
use crate::network_name::NetworkName;

#[derive(Clone)]
#[wasm_bindgen]
pub struct WasmManagedCoreClientReadOnly {
  package_id: ObjectID,
  network: NetworkName,
  iota_client_adapter: IotaClientAdapter,
}

impl WasmManagedCoreClientReadOnly {
  pub fn from_wasm(wasm_core_client: &WasmCoreClientReadOnly) -> Result<Self> {
    let package_id = wasm_core_client
      .package_id()
      .parse()
      .map_err(|e: ObjectIDParseError| JsError::new(&e.to_string()))?;
    let network = wasm_core_client.network().parse().wasm_result()?;
    let iota_client_adapter = IotaClientAdapter::new(wasm_core_client.iota_client());

    Ok(Self {
      package_id,
      network,
      iota_client_adapter,
    })
  }

  pub fn into_wasm(self) -> WasmCoreClientReadOnly {
    JsValue::from(self).unchecked_into()
  }

  pub fn from_rust<C>(core_client: &C) -> Self
  where
    C: CoreClientReadOnly,
  {
    let package_id = core_client.package_id();
    let network = core_client.network_name().clone();
    let iota_client_adapter = core_client.client_adapter().clone();

    Self {
      package_id,
      network,
      iota_client_adapter,
    }
  }
}

#[wasm_bindgen]
impl WasmManagedCoreClientReadOnly {
  // Ensure the TS CoreClientReadOnly interface is exposed.

  #[wasm_bindgen(js_name = packageId)]
  pub fn package_id(&self) -> String {
    self.package_id.to_string()
  }

  #[wasm_bindgen]
  pub fn network(&self) -> String {
    self.network.to_string()
  }

  #[wasm_bindgen(js_name = iotaClient)]
  pub fn iota_client(&self) -> WasmIotaClient {
    self.iota_client_adapter.clone().into_inner()
  }
}

impl CoreClientReadOnly for WasmManagedCoreClientReadOnly {
  fn package_id(&self) -> ObjectID {
    self.package_id
  }

  fn network_name(&self) -> &NetworkName {
    &self.network
  }

  fn client_adapter(&self) -> &IotaClientAdapter {
    &self.iota_client_adapter
  }
}

#[wasm_bindgen]
pub struct WasmManagedCoreClient {
  signer: WasmTransactionSigner,
  sender_address: IotaAddress,
  public_key: PublicKey,
  read_only: WasmManagedCoreClientReadOnly,
}

impl AsRef<WasmManagedCoreClientReadOnly> for WasmManagedCoreClient {
  fn as_ref(&self) -> &WasmManagedCoreClientReadOnly {
    &self.read_only
  }
}

impl WasmManagedCoreClient {
  pub fn from_wasm(wasm_core_client: &WasmCoreClient) -> Result<Self> {
    let signer = wasm_core_client.signer();
    let sender_address = wasm_core_client.sender_address().parse().wasm_result()?;
    let public_key = wasm_core_client.sender_public_key().try_into()?;
    let read_only = WasmManagedCoreClientReadOnly::from_wasm(wasm_core_client.as_ref())?;

    Ok(Self {
      read_only,
      signer,
      sender_address,
      public_key,
    })
  }

  // Note: we don't have any use for this, but will be needed when a duck typed interface will
  // require a CoreClient<S>.
  #[allow(dead_code)]
  pub fn from_rust<C>(core_client: &C) -> Self
  where
    C: CoreClient<WasmTransactionSigner>,
  {
    let read_only = WasmManagedCoreClientReadOnly::from_rust(core_client);
    let signer = core_client.signer().clone();
    let sender_address = core_client.sender_address();
    let public_key = core_client.sender_public_key().clone();

    Self {
      read_only,
      signer,
      sender_address,
      public_key,
    }
  }
}

#[wasm_bindgen]
impl WasmManagedCoreClient {
  // Ensure TS CoreClientReadOnly interface is exposed.

  #[wasm_bindgen(js_name = packageId)]
  pub fn package_id(&self) -> String {
    self.read_only.package_id.to_string()
  }

  #[wasm_bindgen]
  pub fn network(&self) -> String {
    self.read_only.network.to_string()
  }

  #[wasm_bindgen(js_name = iotaClient)]
  pub fn iota_client(&self) -> WasmIotaClient {
    self.read_only.iota_client_adapter.clone().into_inner()
  }

  // Ensure TS CoreClient interface is exposed.

  #[wasm_bindgen]
  pub fn signer(&self) -> WasmTransactionSigner {
    self.signer.clone()
  }

  #[wasm_bindgen(js_name = senderAddress)]
  pub fn sender_address(&self) -> String {
    self.sender_address.to_string()
  }

  #[wasm_bindgen(js_name = senderPublicKey)]
  pub fn sender_public_key(&self) -> Result<WasmPublicKey> {
    WasmPublicKey::try_from(&self.public_key)
  }
}

impl CoreClientReadOnly for WasmManagedCoreClient {
  fn package_id(&self) -> ObjectID {
    self.read_only.package_id
  }

  fn network_name(&self) -> &NetworkName {
    &self.read_only.network
  }

  fn client_adapter(&self) -> &IotaClientAdapter {
    &self.read_only.iota_client_adapter
  }
}

impl CoreClient<WasmTransactionSigner> for WasmManagedCoreClient {
  fn sender_address(&self) -> IotaAddress {
    self.sender_address
  }

  fn sender_public_key(&self) -> &PublicKey {
    &self.public_key
  }

  fn signer(&self) -> &WasmTransactionSigner {
    &self.signer
  }
}
