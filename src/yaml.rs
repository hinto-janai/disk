//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use std::path::PathBuf;
use crate::common;

//---------------------------------------------------------------------------------------------------- Yaml
crate::common::impl_macro!(Yaml, "yml");

/// [`YAML`](http://docs.rs/serde_yaml) file format
///
/// File extension is `.yml`.
///
/// ## Safety
/// When manually implementing, you are **promising** that the `PATH`'s manually specified are correct.
pub unsafe trait Yaml: serde::Serialize + serde::de::DeserializeOwned {
	// Common data/functions.
	common::impl_string!("yml");

	// Required functions for generic-ness.
	#[inline(always)]
	/// Convert [`Self`] to bytes.
	fn to_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
		let mut vec = Vec::with_capacity(128);
		serde_yaml::to_writer(&mut vec, self)?;
		Ok(vec)
	}
	#[inline(always)]
	/// Create [`Self`] from bytes.
	fn from_bytes(bytes: &[u8]) -> Result<Self, anyhow::Error> {
		common::convert_error(serde_yaml::from_slice(bytes))
	}

	// YAML operations.
	#[inline(always)]
	/// Convert [`Self`] to a [`String`].
	///
	/// This uses [`toml_edit::ser::to_string_pretty`];
	fn to_string(&self) -> Result<String, anyhow::Error> {
		common::convert_error(serde_yaml::to_string(self))
	}
	#[inline(always)]
	/// Create [`Self`] from [`String`].
	fn from_string(string: &str) -> Result<Self, anyhow::Error> {
		common::convert_error(serde_yaml::from_str(string))
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}
