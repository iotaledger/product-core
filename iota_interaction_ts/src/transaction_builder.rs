// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;
use std::ops::DerefMut;

use crate::bindings::WasmTransactionBuilder;
use crate::error::TsSdkError;
use identity_iota_interaction::ProgrammableTransactionBcs;
use identity_iota_interaction::TransactionBuilderT;

pub type NativeTsTransactionBuilderBindingWrapper = WasmTransactionBuilder;

pub struct TransactionBuilderTsSdk {
  pub(crate) builder: NativeTsTransactionBuilderBindingWrapper,
}

impl TransactionBuilderTsSdk {
  pub fn new(builder: NativeTsTransactionBuilderBindingWrapper) -> Self {
    TransactionBuilderTsSdk { builder }
  }
}

impl TransactionBuilderT for TransactionBuilderTsSdk {
  type Error = TsSdkError;
  type NativeTxBuilder = NativeTsTransactionBuilderBindingWrapper;

  fn finish(self) -> Result<ProgrammableTransactionBcs, TsSdkError> {
    unimplemented!();
  }

  fn as_native_tx_builder(&mut self) -> &mut Self::NativeTxBuilder {
    todo!()
  }

  fn into_native_tx_builder(self) -> Self::NativeTxBuilder {
    todo!()
  }
}

impl Default for TransactionBuilderTsSdk {
  fn default() -> Self {
    unimplemented!();
  }
}

impl Deref for TransactionBuilderTsSdk {
  type Target = NativeTsTransactionBuilderBindingWrapper;

  fn deref(&self) -> &Self::Target {
    &self.builder
  }
}

impl DerefMut for TransactionBuilderTsSdk {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.builder
  }
}
