//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use std::path::PathBuf;
use crate::common;
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};

//---------------------------------------------------------------------------------------------------- Toml
crate::common::impl_macro!(Toml, "toml");

/// [`TOML`](https://docs.rs/toml_edit) file format
///
/// File extension is `.toml`.
pub trait Toml: serde::Serialize + serde::de::DeserializeOwned {
	// Common data/functions.
	common::impl_string!("toml");

	// Required functions for generic-ness.
	#[inline(always)]
	fn to_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
		Ok(Self::to_string(self)?.into_bytes())
	}
	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, anyhow::Error> {
		common::convert_error(toml_edit::de::from_slice(bytes))
	}

	// TOML operations.
	#[inline(always)]
	/// This uses [`toml_edit::ser::to_string_pretty`];
	fn to_string(&self) -> Result<String, anyhow::Error> {
		common::convert_error(toml_edit::ser::to_string_pretty(self))
	}
	#[inline(always)]
	fn from_string(string: &str) -> Result<Self, anyhow::Error> {
		common::convert_error(toml_edit::de::from_str(string))
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}
