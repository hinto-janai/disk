//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use std::path::PathBuf;
use crate::common;
use serde::{Serialize,Deserialize};
use std::fmt::Display;

// TODO:
// Fix import resolution errors.
//#[cfg(feature = "bincode2")]
//use bincode2::{Encode,Decode};

//---------------------------------------------------------------------------------------------------- Metadata
//#[cfg_attr(feature = "bincode2", derive(::bincode2::Encode, ::bincode2::Decode))]
#[derive(Clone,Hash,Debug,Serialize,Deserialize,PartialEq,Eq,PartialOrd,Ord)]
/// Metadata collected about a file/directory.
///
/// This stores:
/// - [`u64`]: the amount of bytes (saved|removed) (to|from) disk.
/// - [`PathBuf`]: the PATH where the (file|directory) (is|was) (saved|removed).
///
/// ## Display
/// This implements a more human readable [`Display`].
///
/// `format!("{metadata}")` or `metadata.to_string()` looks like this:
/// ```txt
/// 12336 bytes @ /the/path/to/your/file
/// ```
pub struct Metadata {
	size: u64,
	path: PathBuf,
}

impl Metadata {
	/// Create a new [`Metadata`].
	pub(crate) const fn new(size: u64, path: PathBuf) -> Self {
		Self { size, path }
	}

	/// Create a new `0` byte size [`Metadata`].
	pub(crate) const fn zero(path: PathBuf) -> Self {
		Self { size: 0, path }
	}

	/// Returns the amount of bytes removed/saved to disk.
	pub const fn size(&self) -> u64 {
		self.size
	}

	/// Returns the [`PathBuf`] of the file/directory.
	pub fn path(self) -> PathBuf {
		self.path
	}

	/// Clone and returns the inner parts.
	pub fn to_parts(&self) -> (u64, PathBuf) {
		(self.size, self.path.clone())
	}

	/// Consume [`Metadata`] and returns the inner parts.
	pub fn into_parts(self) -> (u64, PathBuf) {
		(self.size, self.path)
	}
}


//---------------------------------------------------------------------------------------------------- Display
impl std::fmt::Display for Metadata {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} bytes @ {}", self.size, self.path.display())
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}
