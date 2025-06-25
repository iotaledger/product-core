// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(target_arch = "wasm32")]
pub mod bindings;
#[cfg(target_arch = "wasm32")]
pub mod common;
#[cfg(target_arch = "wasm32")]
pub mod core_client;
#[cfg(target_arch = "wasm32")]
pub mod error;
#[cfg(target_arch = "wasm32")]
pub mod iota_client_ts_sdk;
#[cfg(target_arch = "wasm32")]
pub mod transaction_builder;

#[cfg(all(target_arch = "wasm32", feature = "keypair-signer"))]
mod keypair_signer;
#[cfg(target_arch = "wasm32")]
pub mod wasm_error;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        #[allow(unused_imports)] pub use error::TsSdkError as AdapterError;
        #[allow(unused_imports)] pub use iota_client_ts_sdk::IotaClientTsSdk as IotaClientAdapter;
        #[allow(unused_imports)] pub use iota_client_ts_sdk::IotaTransactionBlockResponseProvider as IotaTransactionBlockResponseAdapter;
        #[allow(unused_imports)] pub use bindings::WasmIotaTransactionBlockResponseWrapper as NativeTransactionBlockResponse;
        #[allow(unused_imports)] pub use transaction_builder::TransactionBuilderTsSdk as TransactionBuilderAdapter;

        #[allow(unused_imports)] pub use iota_client_ts_sdk::IotaTransactionBlockResponseAdaptedT;
        #[allow(unused_imports)] pub use iota_client_ts_sdk::IotaTransactionBlockResponseAdaptedTraitObj;
        #[allow(unused_imports)] pub use iota_client_ts_sdk::QuorumDriverApiAdaptedT;
        #[allow(unused_imports)] pub use iota_client_ts_sdk::QuorumDriverApiAdaptedTraitObj;
        #[allow(unused_imports)] pub use iota_client_ts_sdk::ReadApiAdaptedT;
        #[allow(unused_imports)] pub use iota_client_ts_sdk::ReadApiAdaptedTraitObj;
        #[allow(unused_imports)] pub use iota_client_ts_sdk::CoinReadApiAdaptedT;
        #[allow(unused_imports)] pub use iota_client_ts_sdk::CoinReadApiAdaptedTraitObj;
        #[allow(unused_imports)] pub use iota_client_ts_sdk::EventApiAdaptedT;
        #[allow(unused_imports)] pub use iota_client_ts_sdk::EventApiAdaptedTraitObj;
        #[allow(unused_imports)] pub use iota_client_ts_sdk::IotaClientAdaptedT;
        #[allow(unused_imports)] pub use iota_client_ts_sdk::IotaClientAdaptedTraitObj;

        #[allow(unused_imports)] pub use bindings::ProgrammableTransaction;
        #[allow(unused_imports)] pub use bindings::WasmPublicKey;
        #[allow(unused_imports)] pub use bindings::Ed25519PublicKey as WasmEd25519PublicKey;
        #[allow(unused_imports)] pub use bindings::Secp256r1PublicKey as WasmSecp256r1PublicKey;
        #[allow(unused_imports)] pub use bindings::Secp256k1PublicKey as WasmSecp256k1PublicKey;
        #[allow(unused_imports)] pub use bindings::WasmIotaSignature;
        #[cfg(feature = "keytool")]
        #[allow(unused_imports)]
        pub use bindings::keytool::*;

        #[cfg(feature = "keypair-signer")]
        pub use keypair_signer::*;

        #[allow(unused_imports)] pub use transaction_builder::NativeTsTransactionBuilderBindingWrapper;
    }
}
