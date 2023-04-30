//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use std::path::PathBuf;
use crate::common;

//---------------------------------------------------------------------------------------------------- Json
crate::common::impl_macro!(Json, "json");

/// [`JSON`](https://docs.rs/serde_json) file format
///
/// File extension is `.json`.
///
/// ## Safety
/// When manually implementing, you are **promising** that the `PATH`'s manually specified are correct.
pub unsafe trait Json: serde::Serialize + serde::de::DeserializeOwned {
	// Common functions.
	common::impl_string!("json");

	// Required functions for generic-ness.
	#[inline(always)]
	/// Convert [`Self`] to bytes.
	///
	/// This uses [`serde_json::ser::to_vec_pretty`];
	fn to_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
		Ok(serde_json::ser::to_vec_pretty(self)?)
	}
	#[inline(always)]
	/// Create [`Self`] from bytes.
	fn from_bytes(bytes: &[u8]) -> Result<Self, anyhow::Error> {
		common::convert_error(serde_json::de::from_slice(bytes))
	}

	// JSON operations.
	#[inline(always)]
	/// This uses [`serde_json::ser::to_string_pretty`];
	fn to_string(&self) -> Result<String, anyhow::Error> {
		common::convert_error(serde_json::ser::to_string_pretty(self))
	}
	#[inline(always)]
	/// Convert [`Self`] to a [`String`].
	fn from_string(string: &str) -> Result<Self, anyhow::Error> {
		common::convert_error(serde_json::de::from_str(string))
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}
