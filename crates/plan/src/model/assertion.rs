// SPDX-License-Identifier: MIT

//! Assertions: pass/fail checks on a response.

use serde::{Deserialize, Serialize};

/// A check applied to a response, tagged by `type` on the wire.
///
/// Patterns and paths are kept as text here; the engine parses them once
/// before a run and reports errors in the editor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[serde(tag = "type", rename_all = "camelCase", deny_unknown_fields)]
pub enum Assertion {
    /// Status code matches a spec such as `200,201`, `200-299` or `2xx`.
    Status {
        /// The status spec.
        #[serde(deserialize_with = "super::scalar::text")]
        expected: String,
    },
    /// Response body contains, or does not contain, some text.
    Body {
        /// Which check to run.
        check: TextCheck,
        /// The text to look for.
        #[serde(deserialize_with = "super::scalar::text")]
        text: String,
    },
    /// An RFC 9535 JSONPath query on the response body.
    #[serde(rename_all = "camelCase")]
    JsonPath {
        /// The JSONPath expression.
        expr: String,
        /// Which check to run.
        check: JsonPathCheck,
        /// The value compared against, for `equals` and `contains`.
        #[serde(
            default,
            deserialize_with = "super::scalar::opt_text",
            skip_serializing_if = "Option::is_none"
        )]
        #[cfg_attr(feature = "ts", ts(optional))]
        value: Option<String>,
    },
    /// The response arrived within a time limit.
    #[serde(rename_all = "camelCase")]
    ResponseTime {
        /// Maximum response time in milliseconds.
        max_ms: u64,
    },
    /// A response header is present, or contains a value.
    Header {
        /// Header name, matched case-insensitively.
        name: String,
        /// Which check to run.
        check: HeaderCheck,
        /// The value looked for, for `contains`.
        #[serde(
            default,
            deserialize_with = "super::scalar::opt_text",
            skip_serializing_if = "Option::is_none"
        )]
        #[cfg_attr(feature = "ts", ts(optional))]
        value: Option<String>,
    },
}

/// Text checks for body assertions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum TextCheck {
    /// The text appears in the body.
    Contains,
    /// The text does not appear in the body.
    NotContains,
}

/// Checks for JSONPath assertions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum JsonPathCheck {
    /// The path matches at least one node.
    Exists,
    /// The path matches no node.
    NotExists,
    /// The matched node equals `value`.
    Equals,
    /// The matched node contains `value`.
    Contains,
}

/// Checks for header assertions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum HeaderCheck {
    /// The header is present.
    Present,
    /// The header's value contains `value`.
    Contains,
}
