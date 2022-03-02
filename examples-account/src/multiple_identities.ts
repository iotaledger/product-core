// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { AccountBuilder, ExplorerUrl, Storage } from './../../node/identity_wasm.js';

/**
 * This example demonstrates how to create multiple identities from a builder 
 * and how to load existing identities into an account.
 */
async function multipleIdentities(storage?: Storage) {

    // Create an AccountBuilder to make it easier to create multiple identities.
    // Every account created from the builder will use the same storage - the default memory storage in this case.
    let builder = new AccountBuilder({
        storage,
    });

    // The creation step generates a keypair, builds an identity
    // and publishes it to the IOTA mainnet.
    let account1 = await builder.createIdentity();

    // Create a second identity.
    let account2 = await builder.createIdentity();

    // Retrieve the did of the identity that account1 manages.
    let iotaDid1 = account1.did();

    // Suppose we're done with account1 and free it.
    account1.free();

    // Now we want to modify the iotaDid1 identity - how do we do that?
    // We can load the identity from storage into an account using the builder.
    let account1Reconstructed = await builder.loadIdentity(iotaDid1);

    // Now we can modify the identity.
    await account1Reconstructed.createMethod({
        fragment: "my_key"
    })

    // Note that there can only ever be one account that manages the same did.
    // If we attempt to create another account that manages the same did as account2, we get an error.
    try {
        await builder.loadIdentity(account2.did());
    } catch (e) {
        if (e instanceof Error) {
            console.assert(e.name === "IdentityInUse")
        }
    }

    // Print the Explorer URL for the DID.
    let did = account1Reconstructed.did().toString();
    console.log(`Explorer Url:`, ExplorerUrl.mainnet().resolverUrl(did));
}

export { multipleIdentities };
