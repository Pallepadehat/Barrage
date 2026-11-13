// SPDX-License-Identifier: MIT

//! Thread groups: a set of virtual users running the same steps.

use serde::{Deserialize, Serialize};

use super::serde_util::{default_true, is_true, is_zero};
use super::step::Step;

/// A group of virtual users that run the same steps in a loop.
///
/// A group stops when its duration or its iteration count is reached,
/// whichever comes first.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ThreadGroup {
    /// Display name, also used to label the group's results.
    pub name: String,
    /// Whether the group runs. Disabled groups stay in the file.
    #[serde(default = "default_true", skip_serializing_if = "is_true")]
    pub enabled: bool,
    /// Number of virtual users.
    pub users: u32,
    /// Seconds over which users start, evenly spaced.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub ramp_up_secs: u64,
    /// Stop after this many seconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub duration_secs: Option<u64>,
    /// Stop each user after this many iterations of the steps.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub iterations: Option<u64>,
    /// Wait between steps, in milliseconds.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub think_time_ms: u64,
    /// Random extra wait added to the think time, from 0 to this many milliseconds.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub think_jitter_ms: u64,
    /// Target requests per second, shared across the group's users. Must be finite.
    #[serde(
        default,
        deserialize_with = "super::scalar::opt_finite",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub target_rps: Option<f64>,
    /// The steps each user runs, in order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub steps: Vec<Step>,
}
