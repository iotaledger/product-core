// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { HeaderMap } from "./http-client";

/** IOTA gas-station options. */
export interface GasStationParamsI {
  /** 
   * Duration of the gas reservation in seconds.
   * Defaults to 60.
  */
  gasReservationDuration?: bigint,
  /**
   * HTTP headers to be passed to all gas station requests.
  */
  headers?: HeaderMap,
}
