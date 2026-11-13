// SPDX-License-Identifier: MIT

//! Lenient readers for scalar fields.
//!
//! Inside tagged items (steps, assertions, extractors) the YAML reader keeps
//! the scalar's type, so `expected: 200` arrives as a number. These helpers
//! accept any scalar for text fields, and reject non-finite numbers that
//! JSON can't represent.

use std::fmt;

use serde::Deserializer;
use serde::de::{self, Visitor};

/// Reads a string, number or boolean as text.
pub(crate) fn text<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    deserializer.deserialize_any(TextVisitor)
}

/// Like [`text`], for optional fields (absence is handled by `#[serde(default)]`).
pub(crate) fn opt_text<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<String>, D::Error> {
    text(deserializer).map(Some)
}

/// Reads an optional number, rejecting NaN and infinity.
pub(crate) fn opt_finite<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<f64>, D::Error> {
    let value: f64 = serde::Deserialize::deserialize(deserializer)?;
    if value.is_finite() {
        Ok(Some(value))
    } else {
        Err(de::Error::custom("expected a finite number"))
    }
}

struct TextVisitor;

impl Visitor<'_> for TextVisitor {
    type Value = String;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a string, number or boolean")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<String, E> {
        Ok(v.to_owned())
    }

    fn visit_string<E: de::Error>(self, v: String) -> Result<String, E> {
        Ok(v)
    }

    fn visit_bool<E: de::Error>(self, v: bool) -> Result<String, E> {
        Ok(v.to_string())
    }

    fn visit_i64<E: de::Error>(self, v: i64) -> Result<String, E> {
        Ok(v.to_string())
    }

    fn visit_u64<E: de::Error>(self, v: u64) -> Result<String, E> {
        Ok(v.to_string())
    }

    fn visit_f64<E: de::Error>(self, v: f64) -> Result<String, E> {
        Ok(v.to_string())
    }
}
