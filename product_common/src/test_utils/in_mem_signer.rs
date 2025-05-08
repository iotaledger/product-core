use std::sync::Arc;

use iota_interaction::IotaKeySignature;
use iota_keys::keystore::{AccountKeystore, InMemKeystore};
use iota_sdk::types::base_types::IotaAddress;
use iota_sdk::types::crypto::{IotaSignature, SignatureScheme};
use secret_storage::{SignatureScheme as SignerSignatureScheme, Signer as SignerTrait};

#[derive(Clone)]
pub struct InMemSigner(pub Arc<InMemKeystore>);

impl InMemSigner {
    pub fn new(alias: impl Into<String>) -> Self {
        let mut mem = InMemKeystore::new_insecure_for_tests(0);
        mem.generate_and_add_new_key(SignatureScheme::ED25519, Some(alias.into()), None, None)
            .expect("Could not generate key");

        InMemSigner(Arc::new(mem))
    }

    pub fn new_with_scheme(scheme: SignatureScheme, alias: impl Into<String>) -> Self {
        let mut mem = InMemKeystore::new_insecure_for_tests(0);
        mem.generate_and_add_new_key(scheme, Some(alias.into()), None, None)
            .expect("Could not generate key");

        InMemSigner(Arc::new(mem))
    }

    pub fn get_address(&self, alias: impl Into<String>) -> anyhow::Result<IotaAddress> {
        let address = self.0.get_address_by_alias(alias.into())?;
        Ok(*address)
    }
}

#[async_trait::async_trait]
impl SignerTrait<IotaKeySignature> for InMemSigner {
    type KeyId = ();
    async fn sign(
        &self,
        hash: &[u8],
    ) -> secret_storage::Result<<IotaKeySignature as SignerSignatureScheme>::Signature> {
        let address = self.0.get_address_by_alias(TEST_ALIAS.to_owned()).unwrap();

        let signature = self.0.sign_hashed(address, hash).unwrap();

        Ok(signature.signature_bytes().to_vec())
    }

    async fn public_key(
        &self,
    ) -> secret_storage::Result<<IotaKeySignature as secret_storage::SignatureScheme>::PublicKey>
    {
        let address = self.0.get_address_by_alias(TEST_ALIAS.to_owned()).unwrap();
        let res = self.0.get_key(address).unwrap();

        let public_key = match res {
            iota_sdk::types::crypto::IotaKeyPair::Ed25519(key) => key.public().as_bytes().to_vec(),
            _ => panic!(),
        };

        Ok(public_key)
    }
    fn key_id(&self) -> &Self::KeyId {
        &()
    }
}
