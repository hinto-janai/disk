//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use std::path::PathBuf;
use crate::common;
use bincode2::config::*;
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};

//---------------------------------------------------------------------------------------------------- Bincode
lazy_static::lazy_static! {
	pub static ref ENCODING_OPTIONS: Configuration = bincode2::config::standard();
}

crate::common::impl_macro_binary!(Bincode2, "bin");

/// [`Bincode2`](https://docs.rs/bincode) (`2.x.x-rc.x`) (binary) file format
///
/// ## `2.x.x-rc.x`
/// [`bincode 2.0.0`](https://docs.rs/bincode/2.0.0-rc.3) (currently not stable) brings big performance improvements.
///
/// It also no longer requires `serde`, having it's own `Encode` and `Decode` traits.
///
/// This means your type must implement these as well, e.g:
/// ```rust
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
	// Common data/functions.
	common::impl_binary!("bincode2");

	#[inline(always)]
	/// Create [`Self`] from bytes.
	fn from_bytes(bytes: &[u8]) -> Result<Self, anyhow::Error> {
		let len = bytes.len();

		// Ensure our `[u8; 25]` HEADER + VERSION bytes are there.
		if len < 25 {
			bail!("Invalid Bincode header data, total byte length less than 25: {}", len);
		}

		// Ensure our HEADER is correct.
		if bytes[..24] != Self::HEADER {
			bail!("Incorrect Bincode header\nExpected: {:?}\nFound: {:?}", Self::HEADER, &bytes[..24],);
		}

		// Ensure our VERSION is correct.
		if bytes[24] != Self::VERSION {
			bail!("Incorrect Bincode version\nExpected: {:?}\nFound: {:?}", Self::VERSION, &bytes[24],);
		}

		match bincode2::decode_from_slice(&bytes[25..], *ENCODING_OPTIONS) {
			Ok((s, _))  => Ok(s),
			Err(e) => common::convert_error(Err(e)),
		}
	}

	#[inline(always)]
	/// Convert [`Self`] to bytes.
	fn to_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
		let mut vec = match bincode2::encode_to_vec(self, *ENCODING_OPTIONS) {
			Ok(v)  => v,
			Err(e) => return common::convert_error(Err(e)),
		};

		let mut bytes = self.header_version_bytes().to_vec();
		bytes.append(&mut vec);

		Ok(bytes)
	}

	// Bincode specific.
	/// A custom 24-byte length identifying header for your Bincode file.
	///
	/// This is combined with [`Self::VERSION`] to prefix your file with 25 bytes.
	///
	/// **Note: [`Self::save_gzip()`] applies compression AFTER, meaning the entire file must be decompressed to get these headers.**
	const HEADER: [u8; 24];
	/// What the version byte will be (0-255).
	const VERSION: u8;

	#[inline]
	/// Return the 25 bytes header bytes.
	///
	/// First 24 bytes are the [`Self::HEADER`] bytes.
	///
	/// Last byte is [`Self::VERSION`].
	fn header_version_bytes(&self) -> [u8; 25] {
		[
			Self::HEADER[0],
			Self::HEADER[1],
			Self::HEADER[2],
			Self::HEADER[3],
			Self::HEADER[4],
			Self::HEADER[5],
			Self::HEADER[6],
			Self::HEADER[7],
			Self::HEADER[8],
			Self::HEADER[9],
			Self::HEADER[10],
			Self::HEADER[11],
			Self::HEADER[12],
			Self::HEADER[13],
			Self::HEADER[14],
			Self::HEADER[15],
			Self::HEADER[16],
			Self::HEADER[17],
			Self::HEADER[18],
			Self::HEADER[19],
			Self::HEADER[20],
			Self::HEADER[21],
			Self::HEADER[22],
			Self::HEADER[23],
			Self::VERSION
		]
	}

	#[inline(always)]
	/// Read the associated file and attempt to convert the first 24 bytes to a [`String`].
	///
	/// This is useful if your [`Self::HEADER`] should be bytes representing a UTF-8 string.
	fn file_header_to_string(&self) -> Result<String, anyhow::Error> {
		let bytes = Self::file_bytes(0,24)?;

		Ok(String::from_utf8(bytes)?)
	}

	#[inline]
	/// Reads the first 24 bytes of the associated file and matches it against [`Self::HEADER`].
	///
	/// If the bytes match, the next byte _should_ be our [`Self::VERSION`] and is returned.
	///
	/// **Note: This only works on a non-compressed version.**
	fn file_version() -> Result<u8, anyhow::Error> {
		use std::io::Read;

		let mut bytes = [0; 25];

		let mut file = std::fs::File::open(Self::absolute_path()?)?;

		file.read_exact(&mut bytes)?;

		if bytes[0..24] == Self::HEADER {
			Ok(bytes[24])
		} else {
			bail!("Bincode header failed to match.\nExpected: {:?}\nFound: {:?}", Self::HEADER, &bytes[0..24]);
		}
	}
}


//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}