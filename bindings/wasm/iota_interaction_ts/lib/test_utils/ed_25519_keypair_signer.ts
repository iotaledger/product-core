// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Ed25519Keypair } from "@iota/iota-sdk/keypairs/ed25519"
import {PublicKey} from "@iota/iota-sdk/dist/esm/cryptography";
import { TransactionSigner } from "~iota_interaction_ts";

/// Simple signer useful to test IOTA products.
/// IMPORTANT: Do not use this signer in production environments
export class Ed25519KeypairSigner implements TransactionSigner {
    signer: Ed25519Keypair;

    constructor(signer: Ed25519Keypair) {
        this.signer = signer;
    }

    async sign(tx_data_bcs: Uint8Array): Promise<string> {
        const signature = await this.signer.signTransaction(tx_data_bcs);
        return signature.signature;
    }
    publicKey(): Promise<PublicKey> {
        return Promise.resolve(this.signer.getPublicKey());
    }
    iotaPublicKeyBytes(): Promise<Uint8Array> {
        return Promise.resolve(this.signer.getPublicKey().toIotaBytes())
    }
    keyId(): string {
        const base64 = this.signer.getPublicKey().toBase64().toString();
        return base64.substring(0, 16);
    }
}