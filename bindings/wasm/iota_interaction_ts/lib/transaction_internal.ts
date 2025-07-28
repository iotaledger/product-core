// Copyright 2021-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { IotaObjectRef, IotaTransactionBlockResponse, TransactionEffects } from "@iota/iota-sdk/client";
import { IotaEvent } from "@iota/iota-sdk/src/client/types/generated";
import { TransactionDataBuilder } from "@iota/iota-sdk/transactions";
import { TransactionSigner } from "~iota_interaction_ts";
import { CoreClient, CoreClientReadOnly } from "./core_client";
import { GasStationParamsI } from "./gas-station";
import { HttpClient } from "./http-client";

export interface TransactionOutput<T extends Transaction<unknown>> {
    response: IotaTransactionBlockResponse;
    output: Awaited<ReturnType<T["apply"]>>;
}

// This interface is generally not implemented by users of IOTA products.
// Instead, users use product-specific `TransactionBuilder` instances provided by an IOTA product.
// Additionally, IOTA products create objects implementing the Transaction interface **partially**
// using wasm-bindgen. As only specific methods are implemented by the product-specific
// `Transaction` implementations, this interface can be seen more as a placeholder for
// the product-specific `Transaction` implementations.
//
// A `TransactionBuilder` used by the user will call the `applyWithEvents`
// method of the product-specific Transaction implementation only in case it exists.
// If the product-specific Transaction implementation doesn't provide an `applyWithEvents` method,
// The product-specific Transaction implementation must provide either an `applyWithEvents`
// or an `apply` method, but not both. See the `apply` and `applyWithEvents` method documentation for more details.
export interface Transaction<Output> {
    /// IOTA products Implement this method to provide programmable transaction blocks performing the necessary
    /// transaction processing as binary BSC serializations.
    buildProgrammableTransaction(client: CoreClientReadOnly): Promise<Uint8Array>;

    /// IOTA products implement this method if they don't need to process events in their Transaction implementation.
    /// Otherwise, IOTA products will implement the `applyWithEvents` method instead.
    /// If the Transaction implementation provides an `applyWithEvents` method, generally no `apply` method is provided.
    apply(effects: TransactionEffects, client: CoreClientReadOnly): Promise<Output>;

    /// IOTA products implement this method only if they need to process events in their Transaction implementation.
    /// Otherwise, IOTA products will implement the `apply` method instead.
    /// If the Transaction implementation provides an `apply` method, generally no `applyWithEvents` method is provided.
    applyWithEvents(effects: TransactionEffects, events: IotaEvent[], client: CoreClientReadOnly): Promise<Output>;
}

export type SponsorFn = (tx_data: TransactionDataBuilder) => Promise<string>;

export interface TransactionBuilder<T extends Transaction<unknown>> {
    get transaction(): Readonly<T>;
    withGasPrice(price: bigint): TransactionBuilder<T>;
    withGasBudget(budget: bigint): TransactionBuilder<T>;
    withGasOwner(owner: string): TransactionBuilder<T>;
    withGasPayment(payment: IotaObjectRef[]): TransactionBuilder<T>;
    withSender(sender: String): TransactionBuilder<T>;
    withSignature<S extends TransactionSigner>(client: CoreClient<S>): TransactionBuilder<T>;
    withSponsor(client: CoreClientReadOnly, sponsorFn: SponsorFn): Promise<TransactionBuilder<T>>;
    build<S extends TransactionSigner>(client: CoreClient<S>): Promise<[Uint8Array, string[], T]>;
    buildAndExecute<S extends TransactionSigner>(client: CoreClient<S>): Promise<TransactionOutput<T>>;
    executeWithGasStation<S extends TransactionSigner>(
        client: CoreClient<S>,
        gasStationUrl: string,
        httpClient: HttpClient,
        options?: GasStationParamsI,
    ): Promise<TransactionOutput<T>>;
}
