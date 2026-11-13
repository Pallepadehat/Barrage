// SPDX-License-Identifier: MIT

//! HTTP request steps.

use serde::{Deserialize, Serialize};

use super::assertion::Assertion;
use super::extractor::Extractor;
use super::plan::KeyValue;
use super::serde_util::{default_true, is_true};

/// An HTTP request, with the assertions and extractors applied to its response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HttpRequest {
    /// Display name, also used to label the request's results.
    pub name: String,
    /// Whether the step runs. Disabled steps stay in the file.
    #[serde(default = "default_true", skip_serializing_if = "is_true")]
    pub enabled: bool,
    /// HTTP method.
    pub method: Method,
    /// Absolute URL, or a path relative to the plan's base URL.
    pub url: String,
    /// Query parameters appended to the URL.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub query: Vec<KeyValue>,
    /// Request headers, added to the plan's default headers.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub headers: Vec<KeyValue>,
    /// How `body` is encoded.
    #[serde(default, skip_serializing_if = "BodyType::is_none")]
    pub body_type: BodyType,
    /// Request body. For `form`, a URL-encoded string such as `a=1&b=2`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// Timeout in milliseconds, overriding the plan default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
    /// Checks applied to the response.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assertions: Vec<Assertion>,
    /// Values pulled from the response into user variables.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extractors: Vec<Extractor>,
}

/// HTTP request method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Method {
    /// `GET`
    Get,
    /// `POST`
    Post,
    /// `PUT`
    Put,
    /// `PATCH`
    Patch,
    /// `DELETE`
    Delete,
    /// `HEAD`
    Head,
    /// `OPTIONS`
    Options,
}

/// How a request body is encoded, which also sets the default `Content-Type`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BodyType {
    /// No body.
    #[default]
    None,
    /// JSON (`application/json`).
    Json,
    /// Plain text (`text/plain`).
    Text,
    /// URL-encoded form (`application/x-www-form-urlencoded`).
    Form,
}

impl BodyType {
    /// True for [`BodyType::None`], the default, which is left out of the file.
    pub const fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}
