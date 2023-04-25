//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use std::path::PathBuf;
use crate::common;

//---------------------------------------------------------------------------------------------------- Yaml
crate::common::impl_macro!(Yaml, "yml");

/// [`YAML`](http://docs.rs/serde_yaml) file format
///
/// File extension is `.yml`.
pub trait Yaml: serde::Serialize + serde::de::DeserializeOwned {
	// Common data/functions.
	common::impl_string!("yml");

	// Required functions for generic-ness.
	#[inline(always)]
	fn to_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
		let mut vec = Vec::with_capacity(128);
		serde_yaml::to_writer(&mut vec, self)?;
		Ok(vec)
	}
	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, anyhow::Error> {
		common::convert_error(serde_yaml::from_slice(bytes))
	}

	// YAML operations.
	#[inline(always)]
	fn to_string(&self) -> Result<String, anyhow::Error> {
		common::convert_error(serde_yaml::to_string(self))
	}
	#[inline(always)]
	fn from_string(string: &str) -> Result<Self, anyhow::Error> {
		common::convert_error(serde_yaml::from_str(string))
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}
