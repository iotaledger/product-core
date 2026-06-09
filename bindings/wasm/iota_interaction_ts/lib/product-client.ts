import { IotaClient } from "@iota/iota-sdk/client";

export interface ProductClient extends IotaClient {
    get packageId(): string;
    get network(): string;
}
