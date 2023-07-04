//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail};
use std::path::PathBuf;
use crate::common;
use std::io::{
	Read,Write,
	BufReader,
};

//---------------------------------------------------------------------------------------------------- Yaml
crate::common::impl_macro!(Yaml, "yml");

/// [`YAML`](http://docs.rs/serde_yaml) file format
///
/// File extension is `.yml`.
///
/// ## Safety
/// When manually implementing, you are **promising** that the `PATH`'s manually specified are correct.
pub unsafe trait Yaml: serde::Serialize + serde::de::DeserializeOwned {
	#[doc(hidden)]
	#[inline(always)]
	/// Internal function. Most efficient `from_file()` impl.
	fn __from_file() -> Result <Self, anyhow::Error> {
		let path = Self::absolute_path()?;
		let file = std::fs::File::open(path)?;
		Ok(serde_yaml::from_reader(BufReader::new(file))?)
	}

	#[doc(hidden)]
	#[inline(always)]
	/// Internal function. Most efficient `from_path()` impl.
	fn __from_path(path: &std::path::Path) -> Result <Self, anyhow::Error> {
		let file = std::fs::File::open(path)?;
		Ok(serde_yaml::from_reader(BufReader::new(file))?)
	}

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

	// Common data/functions.
	common::impl_string!("yml");
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}
