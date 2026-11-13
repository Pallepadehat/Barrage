// SPDX-License-Identifier: MIT

//! Extractors: pull values from a response into user variables.

use serde::{Deserialize, Serialize};

/// Pulls a value from a response into a variable scoped to the virtual user,
/// tagged by `kind` on the wire.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum Extractor {
    /// An RFC 9535 JSONPath query on the response body; takes the first match.
    JsonPath {
        /// Variable that receives the value.
        var: String,
        /// The JSONPath expression.
        expr: String,
        /// Value used when nothing matches.
        #[serde(
            default,
            deserialize_with = "super::scalar::opt_text",
            skip_serializing_if = "Option::is_none"
        )]
        #[cfg_attr(feature = "ts", ts(optional))]
        default: Option<String>,
    },
    /// A regular expression on the response body; takes a capture group.
    Regex {
        /// Variable that receives the value.
        var: String,
        /// The regular expression.
        expr: String,
        /// Capture group to take. Unset means group 1.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "ts", ts(optional))]
        group: Option<u32>,
        /// Value used when nothing matches.
        #[serde(
            default,
            deserialize_with = "super::scalar::opt_text",
            skip_serializing_if = "Option::is_none"
        )]
        #[cfg_attr(feature = "ts", ts(optional))]
        default: Option<String>,
    },
    /// A response header's value.
    Header {
        /// Variable that receives the value.
        var: String,
        /// Header name, matched case-insensitively.
        name: String,
        /// Value used when the header is missing.
        #[serde(
            default,
            deserialize_with = "super::scalar::opt_text",
            skip_serializing_if = "Option::is_none"
        )]
        #[cfg_attr(feature = "ts", ts(optional))]
        default: Option<String>,
    },
}
