// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use identity::core::decode_b58;
use identity::core::FromJson;
use identity::crypto::merkle_key::MerkleDigestTag;
use identity::crypto::merkle_key::MerkleKey;
use identity::crypto::merkle_key::Sha256;
use identity::crypto::merkle_tree::Proof;
use identity::crypto::PublicKey;
use identity::crypto::SecretKey;
use identity::did::verifiable;
use identity::did::MethodScope;
use identity::iota::Error;
use identity::iota::IotaDID;
use identity::iota::IotaDocument;
use identity::iota::IotaVerificationMethod;
use identity::iota::MessageId;
use identity::iota::{DocumentDiff, TangleRef};
use wasm_bindgen::prelude::*;

use crate::common::WasmTimestamp;
use crate::credential::VerifiableCredential;
use crate::credential::VerifiablePresentation;
use crate::crypto::KeyPair;
use crate::crypto::KeyType;
use crate::did::WasmVerificationMethod;
use crate::did::{WasmDID, WasmDocumentDiff};
use crate::error::{Result, WasmResult};
use crate::service::Service;

#[wasm_bindgen(inspectable)]
pub struct NewDocument {
  key: KeyPair,
  doc: WasmDocument,
}

#[wasm_bindgen]
impl NewDocument {
  #[wasm_bindgen(getter)]
  pub fn key(&self) -> KeyPair {
    self.key.clone()
  }

  #[wasm_bindgen(getter)]
  pub fn doc(&self) -> WasmDocument {
    self.doc.clone()
  }
}

// =============================================================================
// =============================================================================

#[wasm_bindgen(js_name = Document, inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct WasmDocument(pub(crate) IotaDocument);

#[wasm_bindgen(js_class = Document)]
impl WasmDocument {
  /// Creates a new DID Document from the given KeyPair.
  #[wasm_bindgen(constructor)]
  #[allow(clippy::new_ret_no_self)]
  pub fn new(type_: KeyType, network: Option<String>, tag: Option<String>) -> Result<NewDocument> {
    let keypair: KeyPair = KeyPair::new(type_)?;
    let public: &PublicKey = keypair.0.public();

    let did: IotaDID = if let Some(network) = network.as_deref() {
      IotaDID::with_network(public.as_ref(), network).wasm_result()?
    } else {
      IotaDID::new(public.as_ref()).wasm_result()?
    };

    let method: IotaVerificationMethod =
      IotaVerificationMethod::from_did(did, &keypair.0, tag.as_deref()).wasm_result()?;
    let document: IotaDocument = IotaDocument::from_authentication(method).wasm_result()?;

    Ok(NewDocument {
      key: keypair,
      doc: Self(document),
    })
  }

  /// Creates a new DID Document from the given KeyPair and optional network.
  ///
  /// If unspecified, network defaults to the mainnet.
  #[wasm_bindgen(js_name = fromKeyPair)]
  pub fn from_keypair(key: &KeyPair, network: Option<String>) -> Result<WasmDocument> {
    let doc = match network {
      Some(net) => IotaDocument::from_keypair_with_network(&key.0, &net),
      None => IotaDocument::from_keypair(&key.0),
    };
    doc.map(Self).wasm_result()
  }

  /// Creates a new DID Document from the given verification [`method`][`Method`].
  #[wasm_bindgen(js_name = fromAuthentication)]
  pub fn from_authentication(method: &WasmVerificationMethod) -> Result<WasmDocument> {
    IotaDocument::from_authentication(method.0.clone())
      .map(Self)
      .wasm_result()
  }

  // ===========================================================================
  // Properties
  // ===========================================================================

  /// Returns the DID Document `id`.
  #[wasm_bindgen(getter)]
  pub fn id(&self) -> WasmDID {
    WasmDID(self.0.id().clone())
  }

  /// Returns the timestamp of when the DID document was created.
  #[wasm_bindgen(getter)]
  pub fn created(&self) -> WasmTimestamp {
    WasmTimestamp::from(self.0.created())
  }

  /// Sets the timestamp of when the DID document was created.
  #[wasm_bindgen(setter = created)]
  pub fn set_created(&mut self, timestamp: WasmTimestamp) {
    self.0.set_created(timestamp.0)
  }

  /// Returns the timestamp of the last DID document update.
  #[wasm_bindgen(getter)]
  pub fn updated(&self) -> WasmTimestamp {
    WasmTimestamp::from(self.0.updated())
  }

  /// Sets the timestamp of the last DID document update.
  #[wasm_bindgen(setter = updated)]
  pub fn set_updated(&mut self, timestamp: WasmTimestamp) {
    self.0.set_updated(timestamp.0)
  }

  /// Returns the DID Document `proof` object.
  #[wasm_bindgen(getter)]
  pub fn proof(&self) -> Result<JsValue> {
    match self.0.proof() {
      Some(proof) => JsValue::from_serde(proof).wasm_result(),
      None => Ok(JsValue::NULL),
    }
  }

  /// Returns the default Verification Method of the DID Document.
  #[wasm_bindgen]
  pub fn authentication(&self) -> WasmVerificationMethod {
    WasmVerificationMethod(self.0.authentication().clone())
  }

  #[wasm_bindgen(getter = messageId)]
  pub fn message_id(&self) -> String {
    self.0.message_id().to_string()
  }

  #[wasm_bindgen(getter = previousMessageId)]
  pub fn previous_message_id(&self) -> String {
    self.0.previous_message_id().to_string()
  }

  #[wasm_bindgen(setter = previousMessageId)]
  pub fn set_previous_message_id(&mut self, value: &str) -> Result<()> {
    let message: MessageId = MessageId::from_str(value).wasm_result()?;

    self.0.set_previous_message_id(message);

    Ok(())
  }

  // ===========================================================================
  // Verification Methods
  // ===========================================================================

  #[wasm_bindgen(js_name = insertMethod)]
  pub fn insert_method(&mut self, method: &WasmVerificationMethod, scope: Option<String>) -> Result<bool> {
    let scope: MethodScope = scope.unwrap_or_default().parse().wasm_result()?;

    Ok(self.0.insert_method(scope, method.0.clone()))
  }

  #[wasm_bindgen(js_name = removeMethod)]
  pub fn remove_method(&mut self, did: &WasmDID) -> Result<()> {
    self.0.remove_method(&did.0).wasm_result()
  }

  #[wasm_bindgen(js_name = insertService)]
  pub fn insert_service(&mut self, service: &Service) -> Result<bool> {
    Ok(self.0.insert_service(service.0.clone()))
  }

  #[wasm_bindgen(js_name = removeService)]
  pub fn remove_service(&mut self, did: &WasmDID) -> Result<()> {
    self.0.remove_service(&did.0).wasm_result()
  }

  // ===========================================================================
  // Signatures
  // ===========================================================================

  /// Signs the DID Document with the default authentication method.
  #[wasm_bindgen]
  pub fn sign(&mut self, key: &KeyPair) -> Result<()> {
    self.0.sign(key.0.secret()).wasm_result()
  }

  /// Verify the signature with the authentication_key
  #[wasm_bindgen]
  pub fn verify(&self) -> bool {
    self.0.verify().is_ok()
  }

  #[wasm_bindgen(js_name = signCredential)]
  pub fn sign_credential(&self, data: &JsValue, args: &JsValue) -> Result<VerifiableCredential> {
    let json: JsValue = self.sign_data(data, args)?;
    let data: VerifiableCredential = VerifiableCredential::from_json(&json)?;

    Ok(data)
  }

  #[wasm_bindgen(js_name = signPresentation)]
  pub fn sign_presentation(&self, data: &JsValue, args: &JsValue) -> Result<VerifiablePresentation> {
    let json: JsValue = self.sign_data(data, args)?;
    let data: VerifiablePresentation = VerifiablePresentation::from_json(&json)?;

    Ok(data)
  }

  /// Creates a signature for the given `data` with the specified DID Document
  /// Verification Method.
  ///
  /// An additional `proof` property is required if using a Merkle Key
  /// Collection verification Method.
  #[wasm_bindgen(js_name = signData)]
  pub fn sign_data(&self, data: &JsValue, args: &JsValue) -> Result<JsValue> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Args {
      MerkleKey {
        method: String,
        public: String,
        secret: String,
        proof: String,
      },
      Default {
        method: String,
        secret: String,
      },
    }

    let mut data: verifiable::Properties = data.into_serde().wasm_result()?;
    let args: Args = args.into_serde().wasm_result()?;

    match args {
      Args::MerkleKey {
        method,
        public,
        secret,
        proof,
      } => {
        let merkle_key: Vec<u8> = self
          .0
          .try_resolve(&*method)
          .and_then(|method| method.key_data().try_decode().map_err(Error::InvalidDoc))
          .wasm_result()?;

        let public: PublicKey = decode_b58(&public).map(Into::into).wasm_result()?;
        let secret: SecretKey = decode_b58(&secret).map(Into::into).wasm_result()?;

        let digest: MerkleDigestTag = MerkleKey::extract_tags(&merkle_key).wasm_result()?.1;
        let proof: Vec<u8> = decode_b58(&proof).wasm_result()?;

        let signer: _ = self.0.signer(&secret).method(&method);

        match digest {
          MerkleDigestTag::SHA256 => match Proof::<Sha256>::decode(&proof) {
            Some(proof) => signer.merkle_key((&public, &proof)).sign(&mut data).wasm_result()?,
            None => return Err("Invalid Public Key Proof".into()),
          },
          _ => return Err("Invalid Merkle Key Digest".into()),
        }
      }
      Args::Default { method, secret } => {
        let secret: SecretKey = decode_b58(&secret).wasm_result().map(Into::into)?;

        self.0.signer(&secret).method(&method).sign(&mut data).wasm_result()?;
      }
    }

    JsValue::from_serde(&data).wasm_result()
  }

  /// Verifies the authenticity of `data` using the target verification method.
  #[wasm_bindgen(js_name = verifyData)]
  pub fn verify_data(&self, data: &JsValue) -> Result<bool> {
    let data: verifiable::Properties = data.into_serde().wasm_result()?;
    let result: bool = self.0.verifier().verify(&data).is_ok();

    Ok(result)
  }

  #[wasm_bindgen(js_name = resolveKey)]
  pub fn resolve_key(&mut self, query: &str) -> Result<WasmVerificationMethod> {
    Ok(WasmVerificationMethod(self.0.try_resolve(query).wasm_result()?.clone()))
  }

  #[wasm_bindgen(js_name = revokeMerkleKey)]
  pub fn revoke_merkle_key(&mut self, query: &str, index: usize) -> Result<bool> {
    let method: &mut IotaVerificationMethod = self
      .0
      .try_resolve_mut(query)
      .and_then(IotaVerificationMethod::try_from_mut)
      .wasm_result()?;

    method.revoke_merkle_key(index).wasm_result()
  }

  // ===========================================================================
  // Diffs
  // ===========================================================================

  /// Generate the difference between two DID Documents and sign it
  #[wasm_bindgen]
  pub fn diff(&self, other: &WasmDocument, message: &str, key: &KeyPair) -> Result<WasmDocumentDiff> {
    self
      .0
      .diff(&other.0, MessageId::from_str(message).wasm_result()?, key.0.secret())
      .map(WasmDocumentDiff::from)
      .wasm_result()
  }

  /// Verifies the `diff` signature and merges the changes into `self`.
  #[wasm_bindgen]
  pub fn merge(&mut self, diff: &str) -> Result<()> {
    let diff: DocumentDiff = DocumentDiff::from_json(diff).wasm_result()?;
    self.0.merge(&diff).wasm_result()?;
    Ok(())
  }

  // ===========================================================================
  // Publishing
  // ===========================================================================

  /// Returns the Tangle index of the integration chain for this DID.
  ///
  /// This is simply the tag segment of the [`IotaDID`].
  /// E.g.
  /// For an [`IotaDocument`] `doc` with DID: did:iota:1234567890abcdefghijklmnopqrstuvxyzABCDEFGHI,
  /// `doc.integration_index()` == "1234567890abcdefghijklmnopqrstuvxyzABCDEFGHI"
  #[wasm_bindgen(js_name = integrationIndex)]
  pub fn integration_index(&self) -> String {
    self.0.integration_index().to_owned()
  }

  /// Returns the Tangle index of the DID diff chain. This should only be called on documents
  /// published on the integration chain.
  ///
  /// This is the Base58-btc encoded SHA-256 digest of the hex-encoded message id.
  #[wasm_bindgen(js_name = diffIndex)]
  pub fn diff_index(message_id: &str) -> Result<String> {
    let message_id = MessageId::from_str(message_id).wasm_result()?;
    IotaDocument::diff_index(&message_id).wasm_result()
  }

  /// Serializes a `Document` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a `Document` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<WasmDocument> {
    json.into_serde().map(Self).wasm_result()
  }
}

impl From<IotaDocument> for WasmDocument {
  fn from(document: IotaDocument) -> Self {
    Self(document)
  }
}
