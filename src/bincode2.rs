//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use std::path::PathBuf;
use crate::common;
use bincode2::config::*;
use crate::header::*;
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};

//---------------------------------------------------------------------------------------------------- Bincode
lazy_static::lazy_static! {
	pub static ref ENCODING_OPTIONS: Configuration = bincode2::config::standard();
}

crate::common::impl_macro_binary!(Bincode2, "bin");

/// [`Bincode2`](https://docs.rs/bincode/2.0.0-rc.3) (`2.x.x-rc.x`) (binary) file format
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
	#[inline(always)]
	/// Create [`Self`] from bytes.
	fn from_bytes(bytes: &[u8]) -> Result<Self, anyhow::Error> {
		ensure_header!(bytes);

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

		header_return!(vec)
	}

	impl_header!();
	common::impl_binary!("bincode2");
}


//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}
