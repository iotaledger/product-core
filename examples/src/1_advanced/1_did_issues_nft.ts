// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Bip39 } from "@iota/crypto.js";
import { IotaDID, IotaDocument, IotaIdentityClient } from "@iota/identity-wasm/node";
import { Client, MnemonicSecretManager } from "@iota/iota-client-wasm/node";
import {
    ADDRESS_UNLOCK_CONDITION_TYPE,
    AddressTypes,
    ALIAS_ADDRESS_TYPE,
    IIssuerFeature,
    INftOutput,
    IOutputResponse,
    IRent,
    ISSUER_FEATURE_TYPE,
    ITransactionPayload,
    METADATA_FEATURE_TYPE,
    NFT_OUTPUT_TYPE,
    OutputTypes,
    PayloadTypes,
    TRANSACTION_ESSENCE_TYPE,
    TRANSACTION_PAYLOAD_TYPE,
    TransactionHelper,
} from "@iota/iota.js";
import { Converter } from "@iota/util.js";
import { API_ENDPOINT, createDid } from "../util";

/** Demonstrates how an identity can issue and own NFTs,
and how observers can verify the issuer of the NFT.

For this example, we consider the case where a manufacturer issues
a digital product passport (DPP) as an NFT. */
export async function didIssuesNft() {
    // ==============================================
    // Create the manufacturer's DID and the DPP NFT.
    // ==============================================

    // Create a new Client to interact with the IOTA ledger.
    const client = await Client.new({
        primaryNode: API_ENDPOINT,
        localPow: true,
    });
    const didClient = new IotaIdentityClient(client);

    // Generate a random mnemonic for our wallet.
    const secretManager: MnemonicSecretManager = {
        mnemonic: Bip39.randomMnemonic(),
    };

    // Create a new DID for the manufacturer. (see "0_create_did" example).
    var { document } = await createDid(client, secretManager);
    let manufacturerDid = document.id();

    // Get the current byte costs.
    const rentStructure: IRent = await didClient.getRentStructure();

    // Get the Bech32 human-readable part (HRP) of the network.
    const networkName: string = await didClient.getNetworkHrp();

    // Create the Alias Address of the manufacturer.
    var manufacturerAliasAddress: AddressTypes = {
        type: ALIAS_ADDRESS_TYPE,
        aliasId: manufacturerDid.toAliasId(),
    };

    // Create a Digital Product Passport NFT issued by the manufacturer.
    let productPassportNft: INftOutput = await client.buildNftOutput({
        nftId: "0x0000000000000000000000000000000000000000000000000000000000000000",
        immutableFeatures: [
            {
                // Set the manufacturer as the immutable issuer.
                type: ISSUER_FEATURE_TYPE,
                address: manufacturerAliasAddress,
            },
            {
                // A proper DPP would hold its metadata here.
                type: METADATA_FEATURE_TYPE,
                data: Converter.utf8ToHex("Digital Product Passport Metadata", true),
            },
        ],
        unlockConditions: [
            {
                // The NFT will initially be owned by the manufacturer.
                type: ADDRESS_UNLOCK_CONDITION_TYPE,
                address: manufacturerAliasAddress,
            },
        ],
    });

    // Set the appropriate storage deposit.
    productPassportNft.amount = TransactionHelper.getStorageDeposit(productPassportNft, rentStructure).toString();

    // Publish the NFT.
    const [blockId, block] = await client.buildAndPostBlock(secretManager, { outputs: [productPassportNft] });
    await client.retryUntilIncluded(blockId);

    // ========================================================
    // Resolve the Digital Product Passport NFT and its issuer.
    // ========================================================

    // Extract the identifier of the NFT from the published block.
    // Non-null assertion is safe because we published a block with a payload.
    let nftId: string = nft_output_id(block.payload!);

    // Fetch the NFT Output.
    const nftOutputId: string = await client.nftOutputId(nftId);
    const outputResponse: IOutputResponse = await client.getOutput(nftOutputId);
    const output: OutputTypes = outputResponse.output;

    // Extract the issuer of the NFT.
    let manufacturerAliasId: string;
    if (output.type === NFT_OUTPUT_TYPE && output.immutableFeatures) {
        const issuerFeature: IIssuerFeature = output.immutableFeatures.find(feature =>
            feature.type === ISSUER_FEATURE_TYPE
        ) as IIssuerFeature;

        if (issuerFeature && issuerFeature.address.type === ALIAS_ADDRESS_TYPE) {
            manufacturerAliasId = issuerFeature.address.aliasId;
        } else {
            throw new Error("expected to find issuer feature with an alias address");
        }
    } else {
        throw new Error("expected NFT output with immutable features");
    }

    // Reconstruct the manufacturer's DID from the Alias Id.
    manufacturerDid = IotaDID.fromAliasId(manufacturerAliasId, networkName);

    // Resolve the issuer of the NFT.
    const manufacturerDocument: IotaDocument = await didClient.resolveDid(manufacturerDid);

    console.log("The issuer of the Digital Product Passport NFT is:", JSON.stringify(manufacturerDocument, null, 2));
}

function nft_output_id(payload: PayloadTypes): string {
    if (payload.type === TRANSACTION_PAYLOAD_TYPE) {
        const txPayload: ITransactionPayload = payload;
        const txHash = Converter.bytesToHex(TransactionHelper.getTransactionPayloadHash(txPayload), true);

        if (txPayload.essence.type === TRANSACTION_ESSENCE_TYPE) {
            const outputs = txPayload.essence.outputs;
            for (let index in txPayload.essence.outputs) {
                if (outputs[index].type === NFT_OUTPUT_TYPE) {
                    const outputId: string = TransactionHelper.outputIdFromTransactionData(txHash, parseInt(index));
                    return TransactionHelper.resolveIdFromOutputId(outputId);
                }
            }
            throw new Error("no NFT output in transaction essence");
        } else {
            throw new Error("expected transaction essence");
        }
    } else {
        throw new Error("expected transaction payload");
    }
}
