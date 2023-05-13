//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail};
use std::path::PathBuf;
use crate::common;
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
use std::io::{
	Read,Write,
	BufReader,
};

//---------------------------------------------------------------------------------------------------- Rmp
crate::common::impl_macro!(Pickle, "pickle");

/// [`Pickle`](https://docs.rs/serde_pickle) (binary) file format
///
/// File extension is `.pickle`.
///
/// ## Encoding
/// The encoding option used is:
/// ```txt
/// serde_pickle::de::DeOptions::new();
/// serde_pickle::ser::DeOptions::new();
/// ```
///
/// ## Safety
/// When manually implementing, you are **promising** that the `PATH`'s manually specified are correct.
pub unsafe trait Pickle: serde::Serialize + serde::de::DeserializeOwned {
	#[doc(hidden)]
	#[inline(always)]
	/// Internal function. Most efficient `from_file()` impl.
	fn __from_file() -> Result <Self, anyhow::Error> {
		let path = Self::absolute_path()?;
		let file = std::fs::File::open(path)?;
		Ok(serde_pickle::de::from_reader(&mut BufReader::new(file), serde_pickle::de::DeOptions::new())?)
	}

	#[inline(always)]
	/// Create [`Self`] from bytes.
	fn from_bytes(bytes: &[u8]) -> Result<Self, anyhow::Error> {
		common::convert_error(serde_pickle::de::from_slice(bytes, serde_pickle::de::DeOptions::new()))
	}

	#[inline(always)]
	/// Convert [`Self`] to bytes.
	fn to_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
		common::convert_error(serde_pickle::ser::to_vec(self, serde_pickle::ser::SerOptions::new()))
	}

	// Common data/functions.
	common::impl_binary!("pickle");
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}
