//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail};
use std::path::PathBuf;
use crate::common;
use bincode2::config::*;
use crate::header::*;
use std::io::{Seek};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
use std::io::{
	Read,Write,
	BufReader,BufWriter,
};
use once_cell::sync::Lazy;

//---------------------------------------------------------------------------------------------------- Bincode
static ENCODING_OPTIONS: Lazy<Configuration> = Lazy::new(bincode2::config::standard);

crate::common::impl_macro_binary!(Bincode2, "bin");

/// [`Bincode2`](https://docs.rs/bincode/2.0.0-rc.3) (`2.x.x-rc.x`) (binary) file format
///
/// ## `2.x.x-rc.x`
/// [`bincode 2.0.0`](https://docs.rs/bincode/2.0.0-rc.3) (currently not stable) brings big performance improvements.
///
/// It also no longer requires `serde`, having it's own `Encode` and `Decode` traits.
///
/// This means your type must implement these as well, e.g:
/// ```rust,ignore
/// use bincode::{Encode, Decode};
///
/// #[derive(Encode, Decode)]
/// struct State;
/// ```
///
/// To implement `bincode 2.x.x`'s new traits, add it to `Cargo.toml`:
/// ```txt
/// bincode = "2.0.0-rc.3"
/// ```
/// and add `#[derive(Encode, Decode)]` to your types, like you would with `serde`.
///
/// ## Encoding
/// The encoding option used is:
/// ```txt
/// bincode::config::standard()
/// ```
///
/// File extension is `.bin`.
///
/// ## Safety
/// When manually implementing, you are **promising** that the `PATH`'s manually specified are correct.
pub unsafe trait Bincode2: bincode2::Encode + bincode2::Decode {
	#[doc(hidden)]
	#[inline(always)]
	/// Internal function. Most efficient `from_file()` impl.
	fn __from_file() -> Result <Self, anyhow::Error> {
		let path = Self::absolute_path()?;
		let mut file = std::fs::File::open(path)?;
		Self::from_reader(&mut file)
	}

	#[doc(hidden)]
	#[inline(always)]
	/// Internal function. Most efficient `from_path()` impl.
	fn __from_path(path: &std::path::Path) -> Result <Self, anyhow::Error> {
		let mut file = std::fs::File::open(path)?;
		Self::from_reader(&mut file)
	}

	#[inline(always)]
	/// Create [`Self`] from bytes.
	fn from_bytes(bytes: &[u8]) -> Result<Self, anyhow::Error> {
		ensure_header!(bytes);

		match bincode2::decode_from_slice(&bytes[25..], *ENCODING_OPTIONS) {
			Ok((s, _))  => Ok(s),
			Err(e) => Err(e)?,
		}
	}

	#[inline(always)]
	/// Convert [`Self`] to bytes.
	fn to_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
		let mut vec = match bincode2::encode_to_vec(self, *ENCODING_OPTIONS) {
			Ok(v)  => v,
			Err(e) => Err(e)?,
		};

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
		Ok(bincode2::decode_from_std_read(&mut reader, *ENCODING_OPTIONS)?)
	}

	#[inline(always)]
	/// Convert [`Self`] directly to the given [`slice`].
	///
	/// The amount of bytes written is returned on success.
	///
	/// ## Error
	/// The slice must be at least `25` bytes in length to holder the header
	/// bytes and must be large enough to hold the resulting serialized bytes.
	fn to_slice(&self, slice: &mut [u8]) -> Result<usize, anyhow::Error> {
		let len = slice.len();
		if len < 25 {
			bail!("input slice length less than 25: {len}");
		}
		slice[..25].copy_from_slice(&Self::full_header());
		Ok(bincode2::encode_into_slice(self, &mut slice[25..], *ENCODING_OPTIONS)?)
	}

	#[inline(always)]
	/// Convert [`Self`] to directly to the writer `W` without intermediate bytes.
	///
	/// The amount of bytes written is returned on success.
	fn to_writer<W>(&self, writer: &mut W) -> Result<usize, anyhow::Error>
		where
			W: Write,
	{
		let mut writer = BufWriter::new(writer);
		writer.write_all(&Self::full_header())?;
		Ok(bincode2::encode_into_std_write(self, &mut writer, *ENCODING_OPTIONS)?)
	}

	impl_header!();
	common::impl_binary!("bincode2");
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}
