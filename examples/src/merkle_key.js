// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Client,
    Config,
    Credential,
    Digest,
    KeyCollection,
    KeyType,
    MethodScope,
    SignatureOptions,
    Timestamp,
    VerificationMethod,
    VerifierOptions
} from '@iota/identity-wasm';
import {createIdentity} from './create_did';

/**
 This example shows how to sign/revoke verifiable credentials on scale.
 Instead of revoking the entire verification method, a single key can be revoked from a MerkleKeyCollection.
 This MerkleKeyCollection can be created as a collection of a power of 2 amount of keys.
 Every key should be used once by the issuer for signing a verifiable credential.
 When the verifiable credential must be revoked, the issuer revokes the index of the revoked key.

 @param {{network: Network, explorer: ExplorerUrl}} clientConfig
 **/
async function merkleKey(clientConfig) {
    // Create a default client configuration from the parent config network.
    const config = Config.fromNetwork(clientConfig.network);

    // Create a client instance to publish messages to the Tangle.
    const client = Client.fromConfig(config);

    // Creates new identities (See "create_did" example)
    const alice = await createIdentity(clientConfig);
    const issuer = await createIdentity(clientConfig);

    // Add a Merkle Key Collection Verification Method with 8 keys (Must be a power of 2)
    const keys = new KeyCollection(KeyType.Ed25519, 8);
    const method = VerificationMethod.createMerkleKey(Digest.Sha256, issuer.doc.id, keys, "key-collection")

    // Add to the DID Document as a general-purpose verification method
    issuer.doc.insertMethod(method, MethodScope.VerificationMethod());
    issuer.doc.metadataPreviousMessageId = issuer.receipt.messageId;
    issuer.doc.metadataUpdated = Timestamp.nowUTC();
    issuer.doc.signSelf(issuer.key, issuer.doc.defaultSigningMethod().id);

    // Publish the Identity to the IOTA Network and log the results.
    // This may take a few seconds to complete proof-of-work.
    const receipt = await client.publishDocument(issuer.doc);
    console.log(`Identity Update: ${clientConfig.explorer.messageUrl(receipt.messageId)}`);

    // Prepare a credential subject indicating the degree earned by Alice
    let credentialSubject = {
        id: alice.doc.id.toString(),
        name: "Alice",
        degreeName: "Bachelor of Science and Arts",
        degreeType: "BachelorDegree",
        GPA: "4.0"
    };

    // Create an unsigned `UniversityDegree` credential for Alice
    const unsignedVc = Credential.extend({
        id: "https://example.edu/credentials/3732",
        type: "UniversityDegreeCredential",
        issuer: issuer.doc.id.toString(),
        credentialSubject,
    });

    // Sign the credential with Issuer's Merkle Key Collection method, with key index 0
    const signedVc = issuer.doc.signCredential(unsignedVc, {
        method: method.id.toString(),
        public: keys.public(0),
        private: keys.private(0),
        proof: keys.merkleProof(Digest.Sha256, 0)
    }, SignatureOptions.default());

    // Check the verifiable credential is valid
    const result = await client.checkCredential(signedVc.toString(), VerifierOptions.default());
    console.log(`VC verification result: ${result.verified}`);
    if (!result.verified) throw new Error("VC not valid");

    // The Issuer would like to revoke the credential (and therefore revokes key 0)
    issuer.doc.revokeMerkleKey(method.id.toString(), 0);
    issuer.doc.metadataPreviousMessageId = receipt.messageId;
    issuer.doc.metadataUpdated = Timestamp.nowUTC();
    issuer.doc.signSelf(issuer.key, issuer.doc.defaultSigningMethod().id);
    const nextReceipt = await client.publishDocument(issuer.doc);
    console.log(`Identity Update: ${clientConfig.explorer.messageUrl(nextReceipt.messageId)}`);

    // Check the verifiable credential is revoked
    const newResult = await client.checkCredential(signedVc.toString(), VerifierOptions.default());
    console.log(`VC verification result (false = revoked): ${newResult.verified}`);
    if (newResult.verified) throw new Error("VC not revoked");
}

export {merkleKey};
