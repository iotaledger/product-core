import { IotaClient } from "@iota/iota-sdk/client";
import { PublicKey } from "@iota/iota-sdk/cryptography";
import { TransactionSigner } from "~iota_interaction_ts";

export interface CoreClientReadOnly {
    packageId(): string;
    network(): string;
    iotaClient(): IotaClient;
}

export interface CoreClient<S extends TransactionSigner> extends CoreClientReadOnly {
    signer(): S;
    senderAddress(): string;
    senderPublicKey(): PublicKey;
}
