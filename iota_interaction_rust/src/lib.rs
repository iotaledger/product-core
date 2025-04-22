// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod iota_client_rust_sdk;
pub mod transaction_builder;
pub mod error;
mod utils;

#[allow(unused_imports)]
pub use iota_client_rust_sdk::IotaClientRustSdk as IotaClientAdapter;
#[allow(unused_imports)]
pub use transaction_builder::TransactionBuilderRustSdk as TransactionBuilderAdapter;

#[allow(unused_imports)]
pub use error::Error as AdapterError;
#[allow(unused_imports)]
pub use iota_client_rust_sdk::CoinReadApiAdaptedT;
#[allow(unused_imports)]
pub use iota_client_rust_sdk::CoinReadApiAdaptedTraitObj;
#[allow(unused_imports)]
pub use iota_client_rust_sdk::EventApiAdaptedT;
#[allow(unused_imports)]
pub use iota_client_rust_sdk::EventApiAdaptedTraitObj;
#[allow(unused_imports)]
pub use iota_client_rust_sdk::IotaClientAdaptedT;
#[allow(unused_imports)]
pub use iota_client_rust_sdk::IotaClientAdaptedTraitObj;
#[allow(unused_imports)]
pub use iota_client_rust_sdk::IotaTransactionBlockResponseAdaptedT;
#[allow(unused_imports)]
pub use iota_client_rust_sdk::IotaTransactionBlockResponseAdaptedTraitObj;
#[allow(unused_imports)]
pub use iota_client_rust_sdk::QuorumDriverApiAdaptedT;
#[allow(unused_imports)]
pub use iota_client_rust_sdk::QuorumDriverApiAdaptedTraitObj;
#[allow(unused_imports)]
pub use iota_client_rust_sdk::ReadApiAdaptedT;
#[allow(unused_imports)]
pub use iota_client_rust_sdk::ReadApiAdaptedTraitObj;
