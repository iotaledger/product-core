use std::collections::HashMap;
use std::fmt::Display;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// An URL.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Url(url::Url);

impl Display for Url {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.0.fmt(f)
  }
}

impl AsRef<str> for Url {
  fn as_ref(&self) -> &str {
    self.as_str()
  }
}

impl Url {
  /// Attempts to parse an [Url] from a string slice.
  #[inline(always)]
  pub fn parse(s: &str) -> Result<Self, UrlParsingError> {
    url::Url::parse(s).map(Self).map_err(|e| UrlParsingError {
      input: s.to_owned(),
      error: e,
    })
  }

  /// Returns this [Url] as string slice.
  #[inline(always)]
  pub fn as_str(&self) -> &str {
    self.0.as_str()
  }

  /// Attempts to parse a new [Url] created from the concatenation of
  /// this one and `other`.
  pub fn join(&self, other: &str) -> Result<Url, UrlParsingError> {
    self.0.join(other).map(Self).map_err(|e| UrlParsingError {
      input: other.to_owned(),
      error: e,
    })
  }
}

#[derive(Debug, thiserror::Error)]
#[error("invalid URL `{input}`")]
#[non_exhaustive]
pub struct UrlParsingError {
  pub input: String,
  #[source]
  pub(crate) error: url::ParseError,
}

/// HTTP request method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Method {
  Get,
  Head,
  Post,
  Put,
  Delete,
  Connect,
  Options,
  Trace,
  Patch,
}

impl Serialize for Method {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(self.as_str())
  }
}

impl Method {
  /// Returns the string representation for this HTTP method.
  #[inline(always)]
  pub fn as_str(&self) -> &'static str {
    use Method::*;
    match self {
      Get => "GET",
      Head => "HEAD",
      Post => "POST",
      Put => "PUT",
      Delete => "DELETE",
      Connect => "CONNECT",
      Options => "OPTIONS",
      Trace => "TRACE",
      Patch => "PATCH",
    }
  }
}

impl Display for Method {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_str())
  }
}

/// A basic HTTP request.
#[derive(Debug, Clone, Serialize)]
pub struct Request<T> {
  pub method: Method,
  pub url: Url,
  pub headers: HashMap<String, String>,
  pub payload: T,
}

/// A basic HTTP response.
#[derive(Debug, Clone, Deserialize)]
pub struct Response<T> {
  pub status_code: u16,
  pub headers: HashMap<String, String>,
  pub payload: T,
}

/// Abstract HTTP Client.
#[cfg_attr(feature = "send-sync", async_trait)]
#[cfg_attr(not(feature = "send-sync"), async_trait(?Send))]
pub trait HttpClient {
  /// Request execution error.
  type Error;
  /// Performs a request.
  async fn send(&self, request: Request<Vec<u8>>) -> Result<Response<Vec<u8>>, Self::Error>;
}

#[cfg(feature = "default-http-client")]
mod reqwest_impl {
  use async_trait::async_trait;
  use reqwest::Client;

  use super::{HttpClient, Method, Request, Response};

  #[cfg_attr(feature = "send-sync", async_trait)]
  #[cfg_attr(not(feature = "send-sync"), async_trait(?Send))]
  impl HttpClient for Client {
    type Error = reqwest::Error;
    async fn send(&self, request: Request<Vec<u8>>) -> Result<Response<Vec<u8>>, Self::Error> {
      // Convert to reqwest's Request type.
      let request = {
        let Request {
          method,
          url,
          ref headers,
          payload,
        } = request;
        // Convert the method.
        let method = match method {
          Method::Get => reqwest::Method::GET,
          Method::Head => reqwest::Method::HEAD,
          Method::Post => reqwest::Method::POST,
          Method::Put => reqwest::Method::PUT,
          Method::Delete => reqwest::Method::DELETE,
          Method::Connect => reqwest::Method::CONNECT,
          Method::Options => reqwest::Method::OPTIONS,
          Method::Trace => reqwest::Method::TRACE,
          Method::Patch => reqwest::Method::PATCH,
        };
        self
          .request(method, url.0)
          .headers(headers.try_into().expect("infallible"))
          .body(payload)
          .build()?
      };

      let response = self.execute(request).await?;
      let status_code = response.status().as_u16();
      let headers = response
        .headers()
        .into_iter()
        .flat_map(|(key, value)| value.to_str().map(|value| (key.to_string(), value.to_owned())))
        .collect();
      let payload = response.bytes().await.unwrap_or_default().to_vec();

      Ok(Response {
        status_code,
        headers,
        payload,
      })
    }
  }
}
