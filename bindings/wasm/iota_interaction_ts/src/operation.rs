// Copyright 2020-2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    graphql_client::Client as IotaClient, transaction_builder::TransactionBuilder,
    types::Transaction,
};
use js_sys::Error as JsError;
use product_core::{operation::Operation, product_client::ProductClient};
use wasm_bindgen::{JsCast, JsValue, prelude::wasm_bindgen};

use crate::{
    bindings::{iota_client::TsIotaClient, transaction::WasmTransaction},
    product_client::{AbstractProductClient, WasmProductClient},
};

#[wasm_bindgen(module = "@iota/iota-interaction-ts/operation")]
extern "C" {
    #[wasm_bindgen(typescript_type = Operation)]
    #[derive(Clone)]
    pub type WasmOperation;

    #[wasm_bindgen(method, catch, js_name = "toTransaction")]
    async fn _to_transaction(
        this: &WasmOperation,
        client: &WasmProductClient,
        tx_builder: WasmTransaction,
    ) -> Result<WasmTransaction, JsError>;

    #[wasm_bindgen(method, catch, js_name = "applyEffects")]
    async fn _apply_effects(
        this: &WasmOperation,
        client: &WasmProductClient,
        effects: &JsValue,
    ) -> Result<JsValue, JsError>;
}

impl Operation for WasmOperation {
    type Output = JsValue;
    type Error = WasmOperationError;

    async fn to_transaction(
        &self,
        client: &impl ProductClient,
        tx_builder: TransactionBuilder<IotaClient>,
    ) -> Result<TransactionBuilder<IotaClient>, Self::Error> {
        let abstract_client = AbstractProductClient::new(client);
        let abstract_client_js = JsValue::from(abstract_client.clone());
        let iota_client = tx_builder.get_client().clone();
        let tx = tx_builder
            .finish()
            .await
            .map_err(|e| WasmOperationError(Box::new(e)))?;
        let wasm_tx = {
            let prev = WasmTransaction::from_bcs_bytes(&tx.to_bcs())?;
            self._to_transaction(abstract_client_js.unchecked_ref(), prev)
                .await?
        };

        let ts_client = TsIotaClient::from(abstract_client);
        let tx_bytes = wasm_tx.build(ts_client).await?;
        let tx = Transaction::from_bcs(&tx_bytes).map_err(|e| WasmOperationError(Box::new(e)))?;

        let tx_builder = TransactionBuilder::try_from(tx)
            .map_err(|e| WasmOperationError(Box::new(e)))?
            .with_client(iota_client);
        Ok(tx_builder)
    }

    async fn apply_effects(
        self,
        client: &impl ProductClient,
        tx_effects: &mut iota_sdk::types::TransactionEffects,
    ) -> Result<Self::Output, Self::Error> {
        let abstract_client = JsValue::from(AbstractProductClient::new(client));
        let ts_effects = serde_wasm_bindgen::to_value(tx_effects)
            .map_err(|e| WasmOperationError(Box::new(e)))?
            .unchecked_into();
        let output = self
            ._apply_effects(abstract_client.unchecked_ref(), &ts_effects)
            .await?;
        let unused_effects = serde_wasm_bindgen::from_value(ts_effects)
            .map_err(|e| WasmOperationError(Box::new(e)))?;
        *tx_effects = unused_effects;
        Ok(output)
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct WasmOperationError(Box<dyn std::error::Error + Send + Sync>);

impl From<js_sys::Error> for WasmOperationError {
    fn from(value: js_sys::Error) -> Self {
        let msg = String::from(value.message().to_string());
        Self(msg.into())
    }
}
