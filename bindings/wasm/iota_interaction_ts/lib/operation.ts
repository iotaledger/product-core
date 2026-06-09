// Copyright 2020-2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { TransactionEffects } from "@iota/iota-sdk/client";
import { Transaction } from "@iota/iota-sdk/transactions";
import { ProductClient } from "product-client";

export interface Operation<Output = unknown> {
    toTransaction: (client: ProductClient, txBuilder: Transaction) => Promise<Transaction>;
    applyEffects: (client: ProductClient, effects: TransactionEffects) => Promise<Output>;
}

export interface OperationOutput<O extends Operation> {
    output: Awaited<ReturnType<O["applyEffects"]>>,
    remainingEffects: TransactionEffects,
}