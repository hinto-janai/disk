//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use std::path::PathBuf;
use crate::common;
use std::io::{
	Read,Write,
	BufReader,BufWriter,
};

//---------------------------------------------------------------------------------------------------- Ron
crate::common::impl_macro!(Ron, "ron");

/// [`RON`](https://docs.rs/ron) file format
///
/// The encoding options used is:
/// ```rust
/// ron::ser::PrettyConfig::new();
/// ```
///
/// File extension is `.ron`.
///
/// ## Safety
/// When manually implementing, you are **promising** that the `PATH`'s manually specified are correct.
pub unsafe trait Ron: serde::Serialize + serde::de::DeserializeOwned {
	#[doc(hidden)]
	#[inline(always)]
	/// Internal function. Most efficient `from_file()` impl.
	fn __from_file() -> Result <Self, anyhow::Error> {
		let path = Self::absolute_path()?;
		let file = std::fs::File::open(&path)?;
		Ok(ron::de::from_reader(&mut BufReader::new(file))?)
	}

	// Required functions for generic-ness.
	#[inline(always)]
	/// Convert [`Self`] to bytes.
	///
	/// This uses [`ron::ser::to_writer_pretty`];
	fn to_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
		let mut vec = vec![];
		ron::ser::to_writer_pretty(&mut vec, self, ron::ser::PrettyConfig::new())?;
		Ok(vec)
	}
	#[inline(always)]
	/// Create [`Self`] from bytes.
	fn from_bytes(bytes: &[u8]) -> Result<Self, anyhow::Error> {
		common::convert_error(ron::de::from_bytes(bytes))
	}

	// JSON operations.
	#[inline(always)]
	/// Convert [`Self`] to a [`String`].
	///
	/// This uses [`ron::ser::to_string_pretty`];
	fn to_string(&self) -> Result<String, anyhow::Error> {
		common::convert_error(ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::new()))
	}
	#[inline(always)]
	/// Create [`Self`] from a [`String`].
	fn from_string(string: &str) -> Result<Self, anyhow::Error> {
		common::convert_error(ron::de::from_str(string))
	}

	// Common functions.
	common::impl_string!("ron");
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}
