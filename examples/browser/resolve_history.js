import {getExplorerUrl, logExplorerUrlToScreen, logObjectToScreen, logToScreen} from "./utils.js";
import {createIdentity} from "./create_did.js";

import * as identity from "../../web/identity_wasm.js";

/**
 Advanced example that performs multiple diff chain and integration chain updates and
 demonstrates how to resolve the DID Document history to view these chains.

 @param {{defaultNodeURL: string, explorerURL: string, network: Network}} clientConfig
 @param {boolean} log log the events to the output window
 **/
export async function resolveHistory(clientConfig, log = true) {
    if (log) logToScreen("Resolve History Example");

    // Create a default client configuration from network.
    const config = identity.Config.fromNetwork(clientConfig.network);

    // Create a client instance to publish messages to the Tangle.
    const client = identity.Client.fromConfig(config);

    // ===========================================================================
    // DID Creation
    // ===========================================================================

    // Create a new identity (see "create_did.js" example).
    const {doc, key, receipt: originalReceipt} = await createIdentity(clientConfig, false);

    // ===========================================================================
    // Integration Chain Spam
    // ===========================================================================

    // Publish several spam messages to the same index as the integration chain on the Tangle.
    // These are not valid DID documents and are simply to demonstrate that invalid messages can be
    // included in the history, potentially for debugging invalid DID documents.
    const intIndex = doc.integrationIndex();
    await client.publishJSON(intIndex, {"intSpam:1": true});
    await client.publishJSON(intIndex, {"intSpam:2": true});
    await client.publishJSON(intIndex, {"intSpam:3": true});
    await client.publishJSON(intIndex, {"intSpam:4": true});
    await client.publishJSON(intIndex, {"intSpam:5": true});

    // ===========================================================================
    // Integration Chain Update 1
    // ===========================================================================

    // Prepare an integration chain update, which writes the full updated DID document to the Tangle.
    const intDoc1 = identity.Document.fromJSON(doc.toJSON()); // clone the Document

    // Add a new VerificationMethod with a new KeyPair, with the tag "keys-1"
    const keys1 = new identity.KeyPair(identity.KeyType.Ed25519);
    const method1 = identity.VerificationMethod.fromDID(intDoc1.id, keys1, "keys-1");
    intDoc1.insertMethod(method1, "VerificationMethod");

    // Add the `messageId` of the previous message in the chain.
    // This is REQUIRED in order for the messages to form a chain.
    // Skipping / forgetting this will render the publication useless.
    intDoc1.previousMessageId = originalReceipt.messageId;
    intDoc1.updated = identity.Timestamp.nowUTC();

    // Sign the DID Document with the original private key.
    intDoc1.sign(key);

    // Publish the updated DID Document to the Tangle, updating the integration chain.
    // This may take a few seconds to complete proof-of-work.
    const intReceipt1 = await client.publishDocument(intDoc1.toJSON());

    // Log the results.
    if (log) logToScreen("Int. Chain Update (1):");
    if (log) logExplorerUrlToScreen(getExplorerUrl(doc, intReceipt1.messageId));

    // ===========================================================================
    // Diff Chain Update 1
    // ===========================================================================

    // Prepare a diff chain DID Document update.
    const diffDoc1 = identity.Document.fromJSON(intDoc1.toJSON()); // clone the Document

    // Add a new Service with the tag "linked-domain-1"
    let serviceJSON1 = {
        id: diffDoc1.id + "#linked-domain-1",
        type: "LinkedDomains",
        serviceEndpoint: "https://iota.org",
    };
    diffDoc1.insertService(identity.Service.fromJSON(serviceJSON1));
    diffDoc1.updated = identity.Timestamp.nowUTC();

    // Create a signed diff update.
    //
    // This is the first diff so the `previousMessageId` property is
    // set to the last DID document published on the integration chain.
    const diff1 = intDoc1.diff(diffDoc1, intReceipt1.messageId, key);

    // Publish the diff to the Tangle, starting a diff chain.
    const diffReceipt1 = await client.publishDiff(intReceipt1.messageId, diff1);
    if (log) logToScreen("Diff Chain Update (1):");
    if (log) logExplorerUrlToScreen(getExplorerUrl(doc, diffReceipt1.messageId));

    // ===========================================================================
    // Diff Chain Update 2
    // ===========================================================================

    // Prepare another diff chain update.
    const diffDoc2 = identity.Document.fromJSON(diffDoc1.toJSON());

    // Add a second Service with the tag "linked-domain-2"
    let serviceJSON2 = {
        id: diffDoc2.id + "#linked-domain-2",
        type: "LinkedDomains",
        serviceEndpoint: "https://example.com",
    };
    diffDoc2.insertService(identity.Service.fromJSON(serviceJSON2));
    diffDoc2.updated = identity.Timestamp.nowUTC();

    // This is the second diff therefore its `previousMessageId` property is
    // set to the first published diff to extend the diff chain.
    const diff2 = diffDoc1.diff(diffDoc2, diffReceipt1.messageId, key);

    // Publish the diff to the Tangle.
    // Note that we still use the `messageId` from the last integration chain message here to link
    // the current diff chain to that point on the integration chain.
    const diffReceipt2 = await client.publishDiff(intReceipt1.messageId, diff2);
    if (log) logToScreen("Diff Chain Update (2):");
    if (log) logExplorerUrlToScreen(getExplorerUrl(doc, diffReceipt2.messageId));

    // ===========================================================================
    // Diff Chain Spam
    // ===========================================================================

    // Publish several spam messages to the same index as the new diff chain on the Tangle.
    // These are not valid DID diffs and are simply to demonstrate that invalid messages
    // can be included in the history for debugging invalid DID diffs.
    let diffIndex = identity.Document.diffIndex(intReceipt1.messageId);
    await client.publishJSON(diffIndex, {"diffSpam:1": true});
    await client.publishJSON(diffIndex, {"diffSpam:2": true});
    await client.publishJSON(diffIndex, {"diffSpam:3": true});

    // ===========================================================================
    // DID History 1
    // ===========================================================================

    // Retrieve the message history of the DID.
    const history1 = await client.resolveHistory(doc.id.toString());

    // The history shows two documents in the integration chain, and two diffs in the diff chain.
    if (log) logToScreen("History (1):")
    if (log) logObjectToScreen(history1);

    // ===========================================================================
    // Integration Chain Update 2
    // ===========================================================================

    // Publish a second integration chain update
    let intDoc2 = identity.Document.fromJSON(diffDoc2.toJSON());

    // Remove the #keys-1 VerificationMethod
    intDoc2.removeMethod(identity.DID.parse(intDoc2.id.toString() + "#keys-1"));

    // Remove the #linked-domain-1 Service
    intDoc2.removeService(identity.DID.parse(intDoc2.id.toString() + "#linked-domain-1"));

    // Add a VerificationMethod with a new KeyPair, called "keys-2"
    const keys2 = new identity.KeyPair(identity.KeyType.Ed25519);
    const method2 = identity.VerificationMethod.fromDID(intDoc1.id, keys2, "keys-2");
    intDoc2.insertMethod(method2, "VerificationMethod");

    // Note: the `previous_message_id` points to the `message_id` of the last integration chain
    //       update, NOT the last diff chain message.
    intDoc2.previousMessageId = intReceipt1.messageId;
    intDoc2.updated = identity.Timestamp.nowUTC();
    intDoc2.sign(key);
    const intReceipt2 = await client.publishDocument(intDoc2.toJSON());

    // Log the results.
    if (log) logToScreen("Int. Chain Update (2):")
    if (log) logExplorerUrlToScreen(getExplorerUrl(doc, intReceipt2.messageId));

    // ===========================================================================
    // DID History 2
    // ===========================================================================

    // Retrieve the updated message history of the DID.
    const history2 = await client.resolveHistory(doc.id.toString());

    // The history now shows three documents in the integration chain, and no diffs in the diff chain.
    // This is because each integration chain document has its own diff chain but only the last one
    // is used during resolution.
    if (log) logToScreen("History (2):")
    if (log) logObjectToScreen(history2);

    // ===========================================================================
    // Diff Chain History
    // ===========================================================================

    // Fetch the diff chain of the previous integration chain message.
    // Old diff chains can be retrieved but they no longer affect DID resolution.
    let previousIntegrationDocument = history2.integrationChainData()[1];
    let previousDiffHistory = await client.resolveDiffHistory(previousIntegrationDocument);
    if (log) logToScreen("Previous Diff History:")
    if (log) logObjectToScreen(previousDiffHistory);
}
