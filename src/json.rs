//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail};
use std::path::PathBuf;
use crate::common;
use serde_json::ser::{Serializer,PrettyFormatter};
use std::io::{
	Read,Write,
	BufReader,
};
use once_cell::sync::Lazy;

//---------------------------------------------------------------------------------------------------- Json
static ENCODING_OPTIONS: Lazy<PrettyFormatter<'static>> = Lazy::new(|| PrettyFormatter::with_indent(b"    "));

crate::common::impl_macro!(Json, "json");
//crate::common::impl_macro_outer!(Json, "json");

/// [`JSON`](https://docs.rs/serde_json) file format
///
/// File extension is `.json`.
///
/// ## Encoding
/// The encoding option used is:
/// ```txt
/// serde_json::ser::PrettyFormatter::with_indent(b"    ");
/// ```
/// This is 4 spaces instead of the default 2.
///
/// ## Safety
/// When manually implementing, you are **promising** that the `PATH`'s manually specified are correct.
pub unsafe trait Json: serde::Serialize + serde::de::DeserializeOwned {
	#[doc(hidden)]
	#[inline(always)]
	/// Internal function. Most efficient `from_file()` impl.
	fn __from_file() -> Result<Self, anyhow::Error> {
		let path = Self::absolute_path()?;
		let file = std::fs::File::open(path)?;
		Ok(serde_json::from_reader(BufReader::new(file))?)
	}

	#[doc(hidden)]
	#[inline(always)]
	/// Internal function. Most efficient `from_path()` impl.
	fn __from_path(path: &std::path::Path) -> Result<Self, anyhow::Error> {
		let file = std::fs::File::open(path)?;
		Ok(serde_json::from_reader(BufReader::new(file))?)
	}

	// Required functions for generic-ness.
	#[inline(always)]
	/// Convert [`Self`] to bytes.
	fn to_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
		let mut vec = Vec::with_capacity(128);
		let mut ser = Serializer::with_formatter(&mut vec, ENCODING_OPTIONS.clone());
		self.serialize(&mut ser)?;
		Ok(vec)
	}
	#[inline(always)]
	/// Create [`Self`] from bytes.
	fn from_bytes(bytes: &[u8]) -> Result<Self, anyhow::Error> {
		Ok(serde_json::de::from_slice(bytes)?)
	}

	// JSON operations.
	#[inline(always)]
	/// This uses [`serde_json::ser::to_string_pretty`];
	fn to_string(&self) -> Result<String, anyhow::Error> {
		let vec = self.to_bytes()?;
		// SAFETY
		// `serde_json` doesn't emit
		// invalid UTF-8 if vec is successful.
		unsafe { Ok(String::from_utf8_unchecked(vec)) }
	}
	#[inline(always)]
	/// Create [`Self`] from a [`String`].
	fn from_string(string: &str) -> Result<Self, anyhow::Error> {
		Ok(serde_json::de::from_str(string)?)
	}

	// Common functions.
	common::impl_string!("json");
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}
