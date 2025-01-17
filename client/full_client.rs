// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;

use crate::IotaDID;
use crate::IotaDocument;
use async_trait::async_trait;
use fastcrypto::ed25519::Ed25519PublicKey;
use fastcrypto::traits::ToFromBytes;
use identity_iota_interaction::move_types::language_storage::StructTag;
use identity_iota_interaction::rpc_types::IotaObjectData;
use identity_iota_interaction::rpc_types::IotaObjectDataFilter;
use identity_iota_interaction::rpc_types::IotaObjectResponseQuery;
use identity_iota_interaction::types::base_types::IotaAddress;
use identity_iota_interaction::types::base_types::ObjectRef;
use identity_verification::jwk::Jwk;
use secret_storage::Signer;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::iota_interaction_adapter::IotaTransactionBlockResponseAdaptedTraitObj;
use crate::rebased::assets::AuthenticatedAssetBuilder;
use crate::rebased::migration::Identity;
use crate::rebased::migration::IdentityBuilder;
use crate::rebased::rebased_err;
use crate::rebased::Error;
use identity_iota_interaction::IotaClientTrait;
use identity_iota_interaction::IotaKeySignature;
use identity_iota_interaction::MoveType;
use identity_iota_interaction::ProgrammableTransactionBcs;

use crate::rebased::transaction::TransactionOutputInternal;
cfg_if::cfg_if! {
  if #[cfg(target_arch = "wasm32")] {
    use crate::rebased::transaction::TransactionInternal as TransactionT;
    type TransactionOutputT<T> = TransactionOutputInternal<T>;
  } else {
    use crate::rebased::transaction::TransactionInternal;
    use crate::rebased::transaction::Transaction as TransactionT;
    use crate::rebased::transaction::TransactionOutput as TransactionOutputT;
  }
}

use super::get_object_id_from_did;
use super::IdentityClientReadOnly;

/// Mirrored types from identity_storage::KeyId
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct KeyId(String);

impl KeyId {
  /// Creates a new key identifier from a string.
  pub fn new(id: impl Into<String>) -> Self {
    Self(id.into())
  }

  /// Returns string representation of the key id.
  pub fn as_str(&self) -> &str {
    &self.0
  }
}

impl std::fmt::Display for KeyId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.0)
  }
}

impl From<KeyId> for String {
  fn from(value: KeyId) -> Self {
    value.0
  }
}

/// A client for interacting with the IOTA network.
#[derive(Clone)]
pub struct IdentityClient<S> {
  /// [`IdentityClientReadOnly`] instance, used for read-only operations.
  read_client: IdentityClientReadOnly,
  /// The address of the client.
  address: IotaAddress,
  /// The public key of the client.
  public_key: Vec<u8>,
  /// The signer of the client.
  signer: S,
}

impl<S> Deref for IdentityClient<S> {
  type Target = IdentityClientReadOnly;
  fn deref(&self) -> &Self::Target {
    &self.read_client
  }
}

impl<S> IdentityClient<S>
where
  S: Signer<IotaKeySignature> + Sync,
{
  /// Create a new [`IdentityClient`].
  pub async fn new(client: IdentityClientReadOnly, signer: S) -> Result<Self, Error> {
    let public_key = signer
      .public_key()
      .await
      .map_err(|e| Error::InvalidKey(e.to_string()))?;
    let address = convert_to_address(&public_key)?;

    Ok(Self {
      public_key,
      address,
      read_client: client,
      signer,
    })
  }

  pub(crate) async fn execute_transaction(
    &self,
    tx_bcs: ProgrammableTransactionBcs,
    gas_budget: Option<u64>,
  ) -> Result<IotaTransactionBlockResponseAdaptedTraitObj, Error> {
    // This code looks like we would call execute_transaction() on
    // self.read_client (which is an IdentityClientReadOnly).
    // Actually we call execute_transaction() on self.read_client.iota_client
    // which is an IotaClientAdapter instance now, provided via the Deref trait.
    // TODO: Find a more transparent way to reference the
    //       IotaClientAdapter for readonly.
    self
      .read_client
      .execute_transaction(
        self.sender_address(),
        self.sender_public_key(),
        tx_bcs,
        gas_budget,
        self.signer(),
      )
      .await
      .map_err(rebased_err)
  }
}

impl<S> IdentityClient<S> {
  /// Returns the bytes of the sender's public key.
  pub fn sender_public_key(&self) -> &[u8] {
    &self.public_key
  }

  /// Returns this [`IdentityClient`]'s sender address.
  pub fn sender_address(&self) -> IotaAddress {
    self.address
  }

  /// Returns a reference to this [`IdentityClient`]'s [`Signer`].
  pub fn signer(&self) -> &S {
    &self.signer
  }

  /// Returns a new [`IdentityBuilder`] in order to build a new [`crate::rebased::migration::OnChainIdentity`].
  pub fn create_identity(&self, iota_document: IotaDocument) -> IdentityBuilder {
    IdentityBuilder::new(iota_document)
  }

  /// Returns a new [`IdentityBuilder`] in order to build a new [`crate::rebased::migration::OnChainIdentity`].
  pub fn create_authenticated_asset<T>(&self, content: T) -> AuthenticatedAssetBuilder<T>
  where
    T: MoveType + Serialize + DeserializeOwned,
  {
    AuthenticatedAssetBuilder::new(content)
  }

  /// Query the objects owned by the address wrapped by this client to find the object of type `tag`
  /// and that satisfies `predicate`.
  pub async fn find_owned_ref<P>(&self, tag: StructTag, predicate: P) -> Result<Option<ObjectRef>, Error>
  where
    P: Fn(&IotaObjectData) -> bool,
  {
    let filter = IotaObjectResponseQuery::new_with_filter(IotaObjectDataFilter::StructType(tag));

    let mut cursor = None;
    loop {
      let mut page = self
        .read_api()
        .get_owned_objects(self.sender_address(), Some(filter.clone()), cursor, None)
        .await?;
      let obj_ref = std::mem::take(&mut page.data)
        .into_iter()
        .filter_map(|res| res.data)
        .find(|obj| predicate(obj))
        .map(|obj_data| obj_data.object_ref());
      cursor = page.next_cursor;

      if obj_ref.is_some() {
        return Ok(obj_ref);
      }
      if !page.has_next_page {
        break;
      }
    }

    Ok(None)
  }
}

impl<S> IdentityClient<S>
where
  S: Signer<IotaKeySignature> + Sync,
{
  /// Returns [`Transaction`] [`PublishDidTx`] that - when executed - will publish a new DID Document on chain.
  pub fn publish_did_document(&self, document: IotaDocument) -> PublishDidTx {
    PublishDidTx(document)
  }

  // TODO: define what happens for (legacy|migrated|new) documents
  /// Updates a DID Document.
  pub async fn publish_did_document_update(
    &self,
    document: IotaDocument,
    gas_budget: u64,
  ) -> Result<IotaDocument, Error> {
    let mut oci =
      if let Identity::FullFledged(value) = self.get_identity(get_object_id_from_did(document.id())?).await? {
        value
      } else {
        return Err(Error::Identity("only new identities can be updated".to_string()));
      };

    oci
      .update_did_document(document.clone())
      .finish(self)
      .await?
      .execute_with_gas(gas_budget, self)
      .await?;

    Ok(document)
  }

  /// Deactivates a DID document.
  pub async fn deactivate_did_output(&self, did: &IotaDID, gas_budget: u64) -> Result<(), Error> {
    let mut oci = if let Identity::FullFledged(value) = self.get_identity(get_object_id_from_did(did)?).await? {
      value
    } else {
      return Err(Error::Identity("only new identities can be deactivated".to_string()));
    };

    oci
      .deactivate_did()
      .finish(self)
      .await?
      .execute_with_gas(gas_budget, self)
      .await?;

    Ok(())
  }
}

/// Utility function that returns the key's bytes of a JWK encoded public ed25519 key.
pub fn get_sender_public_key(sender_public_jwk: &Jwk) -> Result<Vec<u8>, Error> {
  let public_key_base_64 = &sender_public_jwk
    .try_okp_params()
    .map_err(|err| Error::InvalidKey(format!("key not of type `Okp`; {err}")))?
    .x;

  identity_jose::jwu::decode_b64(public_key_base_64)
    .map_err(|err| Error::InvalidKey(format!("could not decode base64 public key; {err}")))
}

/// Utility function to convert a public key's bytes into an [`IotaAddress`].
pub fn convert_to_address(sender_public_key: &[u8]) -> Result<IotaAddress, Error> {
  let public_key = Ed25519PublicKey::from_bytes(sender_public_key)
    .map_err(|err| Error::InvalidKey(format!("could not parse public key to Ed25519 public key; {err}")))?;

  Ok(IotaAddress::from(&public_key))
}

/// Publishes a new DID Document on-chain. An [`crate::rebased::migration::OnChainIdentity`] will be created to contain
/// the provided document.
#[derive(Debug)]
pub struct PublishDidTx(IotaDocument);

impl PublishDidTx {
  async fn execute_publish_did_tx_with_opt_gas<S>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S>,
  ) -> Result<TransactionOutputInternal<IotaDocument>, Error>
  where
    S: Signer<IotaKeySignature> + Sync,
  {
    let TransactionOutputInternal {
      output: identity,
      response,
    } = client
      .create_identity(self.0)
      .finish()
      .execute_with_opt_gas_internal(gas_budget, client)
      .await?;

    Ok(TransactionOutputInternal {
      output: identity.did_doc,
      response,
    })
  }
}

// #[cfg(not(target_arch = "wasm32"))]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl TransactionT for PublishDidTx {
  type Output = IotaDocument;

  #[cfg(not(target_arch = "wasm32"))]
  async fn execute_with_opt_gas<S>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S>,
  ) -> Result<TransactionOutputT<Self::Output>, Error>
  where
    S: Signer<IotaKeySignature> + Sync,
  {
    Ok(
      self
        .execute_publish_did_tx_with_opt_gas(gas_budget, client)
        .await?
        .into(),
    )
  }

  #[cfg(target_arch = "wasm32")]
  async fn execute_with_opt_gas_internal<S>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S>,
  ) -> Result<TransactionOutputT<Self::Output>, Error>
  where
    S: Signer<IotaKeySignature> + Sync,
  {
    self.execute_publish_did_tx_with_opt_gas(gas_budget, client).await
  }
}
