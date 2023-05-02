//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use std::path::PathBuf;
use crate::common;
use serde::{Serialize,Deserialize};

// TODO:
// Fix import resolution errors.
//#[cfg(feature = "bincode2")]
//use bincode2::{Encode,Decode};

//---------------------------------------------------------------------------------------------------- Metadata
//#[cfg_attr(feature = "bincode2", derive(::bincode2::Encode, ::bincode2::Decode))]
#[derive(Clone,Hash,Debug,Serialize,Deserialize,PartialEq,Eq,PartialOrd,Ord)]
/// Metadata collected about a file/directory.
///
/// This store a:
/// [`u64`]: the amount of bytes saved to disk.
/// [`PathBuf`]: the PATH where the file/directory was saved.
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

	/// Return the amount of bytes removed/saved to disk.
	pub const fn size(&self) -> u64 {
		self.size
	}

	/// Return the [`PathBuf`] of the file/directory.
	pub fn path(self) -> PathBuf {
		self.path
	}

	/// Clone and return the inner parts.
	pub fn to_parts(&self) -> (u64, PathBuf) {
		(self.size, self.path.clone())
	}

	/// Consume [`Metadata`] and return the inner parts.
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
