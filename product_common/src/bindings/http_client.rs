// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use js_sys::JsString;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsError, JsValue};

use crate::http_client::{HttpClient, Method, Request, Response};

#[wasm_bindgen(typescript_custom_section)]
const _WASM_METHOD: &str = r#"
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
"#;

#[wasm_bindgen]
extern "C" {
  #[derive(Clone)]
  #[wasm_bindgen(typescript_type = Method)]
  pub type WasmMethod;
}

impl TryFrom<WasmMethod> for Method {
  type Error = JsError;
  fn try_from(value: WasmMethod) -> Result<Self, JsError> {
    let method_str: String = value
      .dyn_ref::<JsString>()
      .ok_or_else(|| JsError::new("`Method` not an instance of string"))?
      .into();
    let method = match method_str.as_str() {
      "GET" => Method::Get,
      "HEAD" => Method::Head,
      "POST" => Method::Post,
      "PUT" => Method::Put,
      "DELETE" => Method::Delete,
      "CONNECT" => Method::Connect,
      "OPTIONS" => Method::Options,
      "TRACE" => Method::Trace,
      "PATCH" => Method::Patch,
      _ => return Err(JsError::new(&format!("`{method_str}` is not a valid HTTP Method"))),
    };

    Ok(method)
  }
}

impl From<Method> for WasmMethod {
  fn from(value: Method) -> Self {
    JsValue::from_str(&value.as_str()).unchecked_into()
  }
}

#[wasm_bindgen(typescript_custom_section)]
const _HTTP_CLIENT_INTERFACE: &str = r#"
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
"#;

#[wasm_bindgen]
extern "C" {
  #[derive(Clone)]
  #[wasm_bindgen(typescript_type = HttpClient)]
  pub type WasmHttpClient;

  #[wasm_bindgen(js_name = send, method, catch)]
  pub async fn send_impl(this: &WasmHttpClient, request: WasmRequest) -> Result<WasmResponse, JsValue>;

  #[wasm_bindgen(typescript_type = Request, extends = js_sys::Object)]
  pub type WasmRequest;

  #[wasm_bindgen(typescript_type = Response, extends = js_sys::Object)]
  pub type WasmResponse;
}

#[async_trait(?Send)]
impl HttpClient for WasmHttpClient {
  type Error = String;
  async fn send(&self, request: Request<Vec<u8>>) -> Result<Response<Vec<u8>>, Self::Error> {
    let wasm_request = serde_wasm_bindgen::to_value(&request)
      .map_err(|e| e.to_string())?
      .unchecked_into();
    let wasm_response = self.send_impl(wasm_request).await.map_err(|value| {
      value
        .dyn_ref::<js_sys::Error>()
        .and_then(|js_err| js_err.message().as_string())
        .unwrap_or_else(|| "`HttpClient.send` failed.".to_owned())
    })?;

    serde_wasm_bindgen::from_value(wasm_response.into()).map_err(|e| e.to_string())
  }
}

#[cfg(feature = "default-http-client")]
pub mod default_http_client {
  use std::ops::Deref;

  use reqwest::Client;

  /// A default implementation for {@link HttpClient}.
  #[wasm_bindgen(js_name = DefaultHttpClient)]
  pub struct WasmDefaultHttpClient(pub(crate) Client);

  impl Deref for WasmDefaultHttpClient {
    type Target = Client;
    fn deref(&self) -> &Self::Target {
      &self.0
    }
  }

  #[wasm_bindgen(js_class = DefaultHttpClient)]
  impl WasmDefaultHttpClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
      Self(Client::default())
    }
  }
}
