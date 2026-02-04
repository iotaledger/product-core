// Copyright 2020-2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

use http::HeaderMap;
use iota_sdk::{
    graphql_client::{Client as IotaClient, WaitForTx},
    transaction_builder::{TransactionBuilder, error::Error as TxError},
    types::{Address, Transaction, TransactionEffects, TransactionExpiration},
};
use secret_storage::iota::TransactionSigner;
use url::Url;

pub trait Operation: Send + Sync {
    type Output;
    type Error: 'static + std::error::Error + Send + Sync;

    fn to_transaction(
        &self,
        client: &IotaClient,
        tx_builder: TransactionBuilder<IotaClient>,
    ) -> impl Future<Output = Result<TransactionBuilder<IotaClient>, Self::Error>>;
    fn apply_effects(
        self,
        client: &IotaClient,
        tx_effects: &mut TransactionEffects,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>>;
}

#[derive(Debug)]
pub struct OperationBuilder<O> {
    operation: O,
    gas_budget: Option<u64>,
    gas_price: Option<u64>,
    sponsor: Option<Address>,
    expiration: TransactionExpiration,
}

impl<O> OperationBuilder<O> {
    pub fn new(operation: O) -> Self {
        Self {
            operation,
            gas_budget: None,
            gas_price: None,
            sponsor: None,
            expiration: TransactionExpiration::None,
        }
    }

    pub fn gas_budget(mut self, gas_budget: u64) -> Self {
        self.gas_budget = Some(gas_budget);
        self
    }

    pub fn gas_price(mut self, gas_price: u64) -> Self {
        self.gas_price = Some(gas_price);
        self
    }

    pub fn sponsor(mut self, sponsor: Address) -> Self {
        self.sponsor = Some(sponsor);
        self
    }

    pub fn expiration(mut self, epoch: u64) -> Self {
        self.expiration = TransactionExpiration::Epoch(epoch);
        self
    }
}

impl<O: Operation> OperationBuilder<O> {
    pub async fn build(
        self,
        sender_signer: &impl TransactionSigner,
        client: &IotaClient,
    ) -> Result<(O, Transaction), OperationError> {
        let tx_builder = self
            .operation
            .to_transaction(client, self.initialize_tx_builder(sender_signer, client))
            .await
            .map_err(|e| OperationError::Build(e.into()))?;

        let tx = tx_builder
            .finish()
            .await
            .map_err(|e| OperationError::Build(e.into()))?;

        Ok((self.operation, tx))
    }

    pub async fn execute(
        self,
        signer: &impl TransactionSigner,
        client: &IotaClient,
    ) -> Result<OperationOutput<O::Output>, Box<dyn std::error::Error + Send + Sync>> {
        let tx_builder = self
            .operation
            .to_transaction(client, self.initialize_tx_builder(signer, client))
            .await?;

        let mut effects = tx_builder.execute(signer, WaitForTx::Finalized).await?;

        let output = self.operation.apply_effects(client, &mut effects).await?;
        Ok(OperationOutput {
            output,
            remaining_effects: effects,
        })
    }

    pub async fn execute_with_sponsor(
        self,
        sender_signer: &impl TransactionSigner,
        sponsor_signer: &impl TransactionSigner,
        client: &IotaClient,
    ) -> Result<OperationOutput<O::Output>, Box<dyn std::error::Error + Send + Sync>> {
        let tx_builder = self
            .operation
            .to_transaction(client, self.initialize_tx_builder(sender_signer, client))
            .await?;

        let mut effects = tx_builder
            .execute_with_sponsor(sender_signer, sponsor_signer, WaitForTx::Finalized)
            .await?;

        let output = self.operation.apply_effects(client, &mut effects).await?;
        Ok(OperationOutput {
            output,
            remaining_effects: effects,
        })
    }

    pub async fn execute_with_gas_station(
        self,
        gas_station_options: GasStationOptions,
        signer: &impl TransactionSigner,
        client: &IotaClient,
    ) -> Result<OperationOutput<O::Output>, Box<dyn std::error::Error + Send + Sync>> {
        let mut tx_builder = self
            .operation
            .to_transaction(client, self.initialize_tx_builder(signer, client))
            .await?;
        {
            let tx_builder_gas_station = tx_builder.gas_station_sponsor(gas_station_options.url);

            if let Some(duration) = gas_station_options.gas_reserve_duration {
                tx_builder_gas_station.gas_reservation_duration(duration);
            }
            for (name, value) in gas_station_options.headers {
                if name.is_none() {
                    continue;
                }

                tx_builder_gas_station.add_gas_station_header(name.expect("is some"), value);
            }
        }

        let mut effects = tx_builder.execute(signer, WaitForTx::Finalized).await?;
        let output = self.operation.apply_effects(client, &mut effects).await?;
        Ok(OperationOutput {
            output,
            remaining_effects: effects,
        })
    }

    fn initialize_tx_builder(
        &self,
        signer: &impl TransactionSigner,
        client: &IotaClient,
    ) -> TransactionBuilder<IotaClient> {
        let mut tx_builder =
            TransactionBuilder::new(signer.address()).with_client((*client).clone());
        if let Some(gas_budget) = self.gas_budget {
            tx_builder.gas_budget(gas_budget);
        }
        if let Some(gas_price) = self.gas_price {
            tx_builder.gas_price(gas_price);
        }
        if let Some(sponsor) = self.sponsor {
            tx_builder.sponsor(sponsor);
        }
        if let TransactionExpiration::Epoch(epoch) = self.expiration {
            tx_builder.expiration(epoch);
        }

        tx_builder
    }
}

#[derive(Debug)]
pub struct OperationOutput<T> {
    pub output: T,
    pub remaining_effects: TransactionEffects,
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum OperationError {
    #[error("failed to build transaction")]
    Build(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("failed to execute this operation's transaction")]
    Execute(#[source] TxError),
}

#[derive(Debug)]
#[non_exhaustive]
pub struct GasStationOptions {
    pub url: Url,
    pub gas_reserve_duration: Option<Duration>,
    pub headers: HeaderMap,
}

impl GasStationOptions {
    pub fn new(url: Url) -> Self {
        Self {
            url,
            gas_reserve_duration: None,
            headers: HeaderMap::default(),
        }
    }
}
