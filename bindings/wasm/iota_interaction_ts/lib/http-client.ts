// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/** HTTP Request's method. */
export const enum Method {
  Get = "GET",
  Head = "HEAD",
  Post = "POST",
  Put = "PUT",
  Delete = "DELETE",
  Connect = "CONNECT",
  Options = "OPTIONS",
  Trace = "TRACE",
  Patch = "PATCH",
}

/** Structure to model HTTP's headers. */
export type HeaderMap = Map<string, string[]>;

/** HTTP Request. */
export interface Request {
  method: Method,
  headers: HeaderMap,
  url: string,
  payload: Uint8Array,
}

/** HTTP Response. */
export interface Response {
  statusCode: number,
  headers: HeaderMap,
  payload: Uint8Array,
}

/** HTTP Client abstract interface. */
export interface HttpClient {
  /** Execute the given HTTP request, returning an HTTP response. */
  send(request: Request): Promise<Response>;
}
