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
crate::common::impl_macro!(MessagePack, "messagepack");

/// [`MessagePack`](https://docs.rs/rmp-serde) (binary) file format
///
/// File extension is `.messagepack`.
///
/// ## Safety
/// When manually implementing, you are **promising** that the `PATH`'s manually specified are correct.
pub unsafe trait MessagePack: serde::Serialize + serde::de::DeserializeOwned {
	#[doc(hidden)]
	#[inline(always)]
	/// Internal function. Most efficient `from_file()` impl.
	fn __from_file() -> Result<Self, anyhow::Error> {
		let path = Self::absolute_path()?;
		let file = std::fs::File::open(path)?;
		Ok(rmp_serde::decode::from_read(BufReader::new(file))?)
	}

	#[doc(hidden)]
	#[inline(always)]
	/// Internal function. Most efficient `from_path()` impl.
	fn __from_path(path: &std::path::Path) -> Result<Self, anyhow::Error> {
		let file = std::fs::File::open(path)?;
		Ok(rmp_serde::decode::from_read(BufReader::new(file))?)
	}

	#[inline(always)]
	/// Create [`Self`] from bytes.
	fn from_bytes(bytes: &[u8]) -> Result<Self, anyhow::Error> {
		common::convert_error(rmp_serde::decode::from_slice(bytes))
	}

	#[inline(always)]
	/// Convert [`Self`] to bytes.
	fn to_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
		common::convert_error(rmp_serde::encode::to_vec(self))
	}

	// Common data/functions.
	common::impl_binary!("messagepack");
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}
