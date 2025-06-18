// Copyright 2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;
use anyhow::anyhow;
use js_sys::Object;

use wasm_bindgen::{JsCast, JsValue};
use iota_interaction_ts::error::{Result, WasmResult};
use iota_interaction_ts::bindings::WasmIotaTransactionBlockEvents;
use iota_interaction_ts::bindings::WasmIotaTransactionBlockEffects;
use iota_interaction_ts::core_client::WasmCoreClientReadOnly;
use iota_interaction::types::base_types::ObjectID;
use iota_interaction::types::base_types::IotaAddress;

use crate::transaction::transaction_builder::Transaction;
use super::{WasmObjectID, WasmIotaAddress};
use super::core_client::WasmManagedCoreClientReadOnly;
use super::transaction::WasmTransactionBuilder;

/// Parses a `WasmObjectID` into an `ObjectID`.
pub fn parse_wasm_object_id(wasm_object_id: &WasmObjectID) -> Result<ObjectID> {
    ObjectID::from_str(wasm_object_id)
        .map_err(|e| anyhow!("Could not parse WasmObjectID: {}", e.to_string()))
        .wasm_result()
}

/// Parses a `WasmIotaAddress` into an `IotaAddress`.
pub fn parse_wasm_iota_address(wasm_iota_address: &WasmIotaAddress) -> Result<IotaAddress> {
    IotaAddress::from_str(wasm_iota_address)
        .map_err(|e| anyhow!("Could not parse WasmIotaAddress: {}", e.to_string()))
        .wasm_result()
}

/// Applies a transaction with events using the provided effects and events.
/// The client is converted into a `WasmManagedCoreClientReadOnly` and is used
/// to call `Transaction::apply_with_events` on the provided `tx`.
/// Returns the `Transaction>::Output` as WASM compatible instance.
/// Example from the notarization repository using WasmOnChainNotarization
/// [original source](https://github.com/iotaledger/notarization/blob/main/bindings/wasm/notarization_wasm/src/wasm_notarization.rs):
/// 
/// ```rust
///     use product_common::bindings::utils::apply_with_events;
///     use notarization::core::notarization::CreateNotarization;
///     use notarization::core::builder::Locked;
/// 
///     #[wasm_bindgen(js_name = CreateNotarizationLocked, inspectable)]
///     pub struct WasmCreateNotarizationLocked(pub(crate) CreateNotarization<Locked>);
///  
///     #[wasm_bindgen(js_class = CreateNotarizationLocked)]
///     impl WasmCreateNotarizationLocked {
/// 
///         ...
/// 
///         #[wasm_bindgen(js_name = applyWithEvents)]
///         pub async fn apply_with_events(
///             self,
///             wasm_effects: &WasmIotaTransactionBlockEffects,
///             wasm_events: &WasmIotaTransactionBlockEvents,
///             client: &WasmCoreClientReadOnly,
///         ) -> Result<WasmOnChainNotarization> {
///             apply_with_events(self.0, wasm_effects, wasm_events, client).await
///         } 
///     }
/// ```
pub async fn apply_with_events<T, O>(
    tx: T,
    wasm_effects: &WasmIotaTransactionBlockEffects,
    wasm_events: &WasmIotaTransactionBlockEvents,
    client: &WasmCoreClientReadOnly,
) -> Result<O>
where
    T: Transaction,
    <T as Transaction>::Error: for<'a> Into<iota_interaction_ts::error::WasmError<'a>>,
    O: From<<T as Transaction>::Output>,
{
    let managed_client = WasmManagedCoreClientReadOnly::from_wasm(client)?;
    let mut effects = wasm_effects.clone().into();
    let mut events = wasm_events.clone().into();
    let apply_result = tx.apply_with_events(&mut effects, &mut events, &managed_client).await;
    let rem_wasm_effects = WasmIotaTransactionBlockEffects::from(&effects);
    Object::assign(wasm_effects, &rem_wasm_effects);
    let rem_wasm_events = WasmIotaTransactionBlockEvents::from(&events);
    Object::assign(wasm_events, &rem_wasm_events);
    apply_result.wasm_result().map(|output: <T as Transaction>::Output| O::from(output))
}

/// Converts the client into a `WasmManagedCoreClientReadOnly` and calls
/// `Transaction::build_programmable_transaction` on the provided transaction using it.
/// The resulting programmable transaction is serialized to bytes using BCS
/// and returned as a `Vec<u8>`.
/// Example from the notarization repository
/// [original source](https://github.com/iotaledger/notarization/blob/main/bindings/wasm/notarization_wasm/src/wasm_notarization.rs):
///
/// ```rust
///     use product_common::bindings::utils::build_programmable_transaction;
///     use notarization::core::notarization::CreateNotarization;
///     use notarization::core::builder::Locked;
///
///     #[wasm_bindgen(js_name = CreateNotarizationLocked, inspectable)]
///     pub struct WasmCreateNotarizationLocked(pub(crate) CreateNotarization<Locked>);
///  
///     #[wasm_bindgen(js_class = CreateNotarizationLocked)]
///     impl WasmCreateNotarizationLocked {
///
///         ...
///
///         #[wasm_bindgen(js_name = buildProgrammableTransaction)]
///         pub async fn build_programmable_transaction(&self, client: &WasmCoreClientReadOnly) -> Result<Vec<u8>> {
///             build_programmable_transaction(&self.0, client).await
///         }
///     }
/// ```
pub async fn build_programmable_transaction<T>(
    tx: &T,
    client: &WasmCoreClientReadOnly
) -> Result<Vec<u8>> 
where
    T: Transaction,
    <T as Transaction>::Error: for<'a> Into<iota_interaction_ts::error::WasmError<'a>>
{
    let managed_client = WasmManagedCoreClientReadOnly::from_wasm(client)?;
    let pt = tx
        .build_programmable_transaction(&managed_client)
        .await
        .wasm_result()?;
    bcs::to_bytes(&pt).wasm_result()
}

/// Converts a wasm transaction (example WasmUpdateState in the notarization repository) into a `WasmTransactionBuilder`.
pub fn into_transaction_builder<T>(tx: T) -> WasmTransactionBuilder
where
    wasm_bindgen::JsValue: From<T>
{
    let js_tx = JsValue::from(tx);
    WasmTransactionBuilder::new(js_tx.unchecked_into())
}
