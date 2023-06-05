//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail};
use std::path::PathBuf;
use crate::common;
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};

//---------------------------------------------------------------------------------------------------- Toml
crate::common::impl_macro_no_ext!(Plain);

/// [`Plain`](https://docs.rs/serde_plain) text file format
///
/// This is a plain text file with no file extension.
/// Typically used for small and simple data types like integers, strings, and enums.
///
/// ## Safety
/// When manually implementing, you are **promising** that the `PATH`'s manually specified are correct.
pub unsafe trait Plain: serde::Serialize + serde::de::DeserializeOwned {
	#[doc(hidden)]
	#[inline(always)]
	/// Internal function. Most efficient `from_file()` impl.
	fn __from_file() -> Result <Self, anyhow::Error> {
		Self::from_bytes(&Self::read_to_bytes()?)
	}

	// Required functions for generic-ness.
	#[inline(always)]
	/// Convert [`Self`] to bytes.
	fn to_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
		Ok(Self::to_string(self)?.into_bytes())
	}
	#[inline(always)]
	/// Create [`Self`] from bytes.
	fn from_bytes(bytes: &[u8]) -> Result<Self, anyhow::Error> {
		let string = std::str::from_utf8(bytes)?;
		common::convert_error(serde_plain::from_str(string))
	}

	// Plain text operations.
	#[inline(always)]
	/// Convert [`Self`] to a [`String`].
	///
	/// This uses [`toml_edit::ser::to_string_pretty`];
	fn to_string(&self) -> Result<String, anyhow::Error> {
		// Newline must be appended.
		Ok(format!("{}\n", serde_plain::to_string(self)?))
	}
	#[inline(always)]
	/// Create [`Self`] from a [`String`].
	fn from_string(string: &str) -> Result<Self, anyhow::Error> {
		common::convert_error(serde_plain::from_str(string))
	}

	// Common data/functions.
	common::impl_string!("");
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}
