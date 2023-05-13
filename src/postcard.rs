//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail};
use std::path::PathBuf;
use crate::common;
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};

//---------------------------------------------------------------------------------------------------- Postcard
crate::common::impl_macro!(Postcard, "bin");

/// [`Postcard`](https://docs.rs/postcard) (binary) file format
///
/// File extension is `.bin`.
///
/// ## Safety
/// When manually implementing, you are **promising** that the `PATH`'s manually specified are correct.
pub unsafe trait Postcard: serde::Serialize + serde::de::DeserializeOwned {
	#[doc(hidden)]
	#[inline(always)]
	/// Internal function. Most efficient `from_file()` impl.
	fn __from_file() -> Result <Self, anyhow::Error> {
		Self::from_bytes(&Self::read_to_bytes()?)
	}

	#[inline(always)]
	/// Create [`Self`] from bytes.
	fn from_bytes(bytes: &[u8]) -> Result<Self, anyhow::Error> {
		common::convert_error(postcard::from_bytes(bytes))
	}

	#[inline(always)]
	/// Convert [`Self`] to bytes.
	fn to_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
		let vec = common::convert_error(postcard::to_stdvec(self))?;
		Ok(vec)
	}

	// Common data/functions.
	common::impl_binary!("postcard");
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}
