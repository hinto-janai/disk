//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use std::path::PathBuf;
use crate::common;

//---------------------------------------------------------------------------------------------------- Json
/// [`JSON`](https://docs.rs/serde_json) file format
///
/// File extension is `.json`.
pub trait Json: serde::Serialize + serde::de::DeserializeOwned {
	// Common functions.
	common::impl_string!("json");

	// Required functions for generic-ness.
	#[inline(always)]
	/// This uses [`serde_json::ser::to_vec_pretty`];
	fn to_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
		Ok(serde_json::ser::to_vec_pretty(self)?)
	}
	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, anyhow::Error> {
		common::convert_error(serde_json::de::from_slice(bytes))
	}

	// JSON operations.
	#[inline(always)]
	/// This uses [`serde_json::ser::to_string_pretty`];
	fn to_string(&self) -> Result<String, anyhow::Error> {
		common::convert_error(serde_json::ser::to_string_pretty(self))
	}
	#[inline(always)]
	fn from_string(string: &str) -> Result<Self, anyhow::Error> {
		common::convert_error(serde_json::de::from_str(string))
	}
}

/// Quickly implement the [`Json`] trait.
///
/// File extension is `.json`.
#[macro_export]
macro_rules! json_file {
	($type:ty, $dir:expr, $project_directory:tt, $sub_directories:tt, $file_name:tt) => {
		const_assert!(const_format!("{}", $project_directory).len() != 0);
		const_assert!(const_format!("{}", $file_name).len() != 0);
 		impl Json for $type {
			const OS_DIRECTORY: Dir = $dir;
			const PROJECT_DIRECTORY: &'static str = $project_directory;
			const SUB_DIRECTORIES: &'static str = $sub_directories;
			const FILE_NAME: &'static str = const_format!("{}.{}", $file_name, "json");
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}
