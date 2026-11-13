// SPDX-License-Identifier: MIT

//! The plan root and the settings that apply to the whole plan.

use serde::{Deserialize, Serialize};

use super::serde_util::{default_true, is_true};
use super::thread_group::ThreadGroup;

/// A complete test plan: the root of a `.barrage.yaml` / `.barrage.json` file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Plan {
    /// Display name of the plan.
    pub name: String,
    /// Plan variables, available as `${key}` in every thread group.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variables: Vec<Variable>,
    /// HTTP settings shared by every request in the plan.
    #[serde(default, skip_serializing_if = "HttpDefaults::is_empty")]
    pub defaults: HttpDefaults,
    /// CSV files that feed variables, one row per user iteration.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub csv_data: Vec<CsvDataSet>,
    /// The groups of virtual users that run the plan's steps.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub thread_groups: Vec<ThreadGroup>,
}

/// A plan variable: a name and its initial value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Variable {
    /// Variable name, referenced as `${key}`.
    pub key: String,
    /// Initial value; may itself contain `${...}` references.
    pub value: String,
}

/// A name/value pair used for headers and query parameters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct KeyValue {
    /// Header or parameter name.
    pub key: String,
    /// Header or parameter value; may contain `${...}` references.
    pub value: String,
    /// Whether the pair is sent. Disabled pairs stay in the file.
    #[serde(default = "default_true", skip_serializing_if = "is_true")]
    pub enabled: bool,
}

/// HTTP settings applied to every request unless the request overrides them.
///
/// Unset fields fall back to the engine's defaults.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HttpDefaults {
    /// Base URL that relative request URLs are resolved against.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    /// Headers sent with every request.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub headers: Vec<KeyValue>,
    /// Request timeout in milliseconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
    /// Whether redirects are followed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub follow_redirects: Option<bool>,
    /// Whether each virtual user keeps a cookie jar.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cookies: Option<bool>,
}

impl HttpDefaults {
    /// True when no default is set, so the block can be left out of the file.
    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// A CSV file whose rows feed variables, one row per user iteration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CsvDataSet {
    /// Path to the CSV file, relative to the plan file.
    pub path: String,
    /// Variable names for the columns. Empty means the first row is a header.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variables: Vec<String>,
    /// Column delimiter. Unset means a comma.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<char>,
    /// At end of file, start over (`true`) or stop the user (`false`).
    #[serde(default = "default_true")]
    pub recycle: bool,
}
