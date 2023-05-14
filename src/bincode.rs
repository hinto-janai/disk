//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail};
use std::path::PathBuf;
use crate::common;
use crate::header::*;
use bincode::config::*;
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
use std::io::{
	Read,Write,
	BufReader,BufWriter,
};
use once_cell::sync::Lazy;

//---------------------------------------------------------------------------------------------------- Bincode
static ENCODING_OPTIONS: Lazy<WithOtherIntEncoding<DefaultOptions, VarintEncoding>> =
		Lazy::new(|| bincode::DefaultOptions::new().with_varint_encoding());

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
	#[doc(hidden)]
	#[inline(always)]
	/// Internal function. Most efficient `from_file()` impl.
	fn __from_file() -> Result <Self, anyhow::Error> {
		let path = Self::absolute_path()?;
		let mut file = std::fs::File::open(path)?;
		Self::from_reader(&mut file)
	}

	#[inline(always)]
	/// Create a [`Self`] from bytes.
	fn from_bytes(bytes: &[u8]) -> Result<Self, anyhow::Error> {
		ensure_header!(bytes);
		Ok(ENCODING_OPTIONS.deserialize(&bytes[25..])?)
	}

	#[inline(always)]
	/// Convert [`Self`] to bytes.
	fn to_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
		let mut vec = ENCODING_OPTIONS.serialize(self)?;
		header_return!(vec)
	}

	#[inline(always)]
	/// Create [`Self`] directly from reader `R`.
	fn from_reader<R>(reader: &mut R) -> Result<Self, anyhow::Error>
		where
			R: Read,
	{
		let mut bytes = [0_u8; 25];
		let mut reader = BufReader::new(reader);
		reader.read_exact(&mut bytes)?;
		ensure_header!(bytes);
		Ok(ENCODING_OPTIONS.deserialize_from(&mut reader)?)
	}

	#[inline(always)]
	/// Convert [`Self`] to directly to the writer `W` without intermediate bytes.
	fn to_writer<W>(&self, writer: &mut W) -> Result<(), anyhow::Error>
		where
			W: Write,
	{
		let mut writer = BufWriter::new(writer);
		writer.write_all(&Self::full_header())?;
		Ok(ENCODING_OPTIONS.serialize_into(&mut writer, self)?)
	}

	impl_header!();
	common::impl_binary!("bincode");
}


//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}
