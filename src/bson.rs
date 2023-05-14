//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail};
use std::path::PathBuf;
use crate::common;
use std::io::BufReader;
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};

//---------------------------------------------------------------------------------------------------- Rmp
crate::common::impl_macro!(Bson, "bson");

/// [`Bson`](https://docs.rs/bson) (binary) file format
///
/// File extension is `.bson`.
///
/// ## Safety
/// When manually implementing, you are **promising** that the `PATH`'s manually specified are correct.
pub unsafe trait Bson: serde::Serialize + serde::de::DeserializeOwned {
	#[doc(hidden)]
	#[inline(always)]
	/// Internal function. Most efficient `from_file()` impl.
	fn __from_file() -> Result <Self, anyhow::Error> {
		let path = Self::absolute_path()?;
		let file = std::fs::File::open(path)?;
		Ok(bson::from_reader(BufReader::new(file))?)
	}

	#[inline(always)]
	/// Create [`Self`] from bytes.
	fn from_bytes(bytes: &[u8]) -> Result<Self, anyhow::Error> {
		Ok(bson::from_slice(bytes)?)
	}

	#[inline(always)]
	/// Convert [`Self`] to bytes.
	fn to_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
		Ok(bson::to_vec(self)?)
	}

	// Common data/functions.
	common::impl_binary!("bson");
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}
