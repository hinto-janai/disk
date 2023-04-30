//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use std::path::PathBuf;
use crate::common;
use crate::header::*;
use bincode::config::*;
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};

//---------------------------------------------------------------------------------------------------- Bincode
lazy_static::lazy_static! {
	pub static ref ENCODING_OPTIONS: WithOtherIntEncoding<DefaultOptions, VarintEncoding> =
		bincode::DefaultOptions::new().with_varint_encoding();
}

common::impl_macro_binary!(Bincode, "bin");

/// [`Bincode`](https://docs.rs/bincode) (binary) file format
///
/// ## Encoding
/// The encoding option used is:
/// ```rust
/// # use bincode::Options;
/// bincode::DefaultOptions::new().with_varint_encoding();
/// ```
///
/// File extension is `.bin`.
///
/// ## Safety
/// When manually implementing, you are **promising** that the `PATH`'s manually specified are correct.
pub unsafe trait Bincode: serde::Serialize + serde::de::DeserializeOwned {
	#[inline(always)]
	/// Create a [`Self`] from bytes.
	fn from_bytes(bytes: &[u8]) -> Result<Self, anyhow::Error> {
		ensure_header!(bytes);
		common::convert_error(ENCODING_OPTIONS.deserialize(&bytes[25..]))
	}

	#[inline(always)]
	/// Convert [`Self`] to bytes.
	fn to_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
		let mut vec = common::convert_error(ENCODING_OPTIONS.serialize(self))?;
		header_return!(vec)
	}

	impl_header!();
	common::impl_binary!("bincode");
}


//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}
