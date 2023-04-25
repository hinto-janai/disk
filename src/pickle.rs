//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use std::path::PathBuf;
use crate::common;
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};

//---------------------------------------------------------------------------------------------------- Rmp
/// [`Pickle`](https://docs.rs/serde_pickle) (binary) file format
///
/// File extension is `.pickle`.
pub trait Pickle: serde::Serialize + serde::de::DeserializeOwned {
	// Common data/functions.
	common::impl_binary!("pickle");

	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, anyhow::Error> {
		common::convert_error(serde_pickle::de::from_slice(bytes, serde_pickle::de::DeOptions::new()))
	}

	#[inline(always)]
	fn to_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
		common::convert_error(serde_pickle::ser::to_vec(self, serde_pickle::ser::SerOptions::new()))
	}
}

/// Quickly implement the [`Pickle`] trait.
///
/// File extension is `.pickle`.
#[macro_export]
macro_rules! pickle {
	($type:ty, $dir:expr, $project_directory:tt, $sub_directories:tt, $file_name:tt) => {
		$crate::const_assert!($crate::const_format!("{}", $project_directory).len() != 0);
		$crate::const_assert!($crate::const_format!("{}", $file_name).len() != 0);
		#[$crate::inherent]
 		impl $crate::Pickle for $type {
			const OS_DIRECTORY: $crate::Dir = $dir;
			const PROJECT_DIRECTORY: &'static str = $project_directory;
			const SUB_DIRECTORIES: &'static str = $sub_directories;
			const FILE_NAME: &'static str = $crate::const_format!("{}.{}", $file_name, "pickle");
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}
