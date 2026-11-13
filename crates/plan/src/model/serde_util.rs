// SPDX-License-Identifier: MIT

//! Small helpers for `serde` defaults and `skip_serializing_if`.
//!
//! Optional settings are left out of saved plans when they hold their default,
//! so files stay short and diffs only show what the user changed.

/// Default for flags that are on unless the plan says otherwise.
pub(crate) const fn default_true() -> bool {
    true
}

/// `skip_serializing_if` for flags that default to `true`.
pub(crate) const fn is_true(value: &bool) -> bool {
    *value
}

/// `skip_serializing_if` for counters that default to `0`.
pub(crate) const fn is_zero(value: &u64) -> bool {
    *value == 0
}
