//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use std::path::PathBuf;
use crate::common;

//---------------------------------------------------------------------------------------------------- Toml
/// [`Empty`] file
///
/// This is a an empty file. It contains no data, but it inherits useful `PATH` methods.
/// Typically used for file-based signals.
///
/// If you implement this on a `struct` that contains data, the data will be ignored
/// and an empty file will always be created.
///
/// The file created will have _no_ file extension, e.g:
/// ```rust
/// # use serde::{Serialize,Deserialize};
/// # use disk::{Empty,empty_file};
/// # use disk::prelude::*;
///
/// empty_file!(Hello, Dir::Data, "disk_test", "signal", "hello");
/// #[derive(Serialize, Deserialize)]
/// struct Hello {
///     data: bool,
/// }
///
/// // The filename should be "hello".
/// assert!(Hello::file_name() == "hello");
///
/// // Create the file.
/// Hello::touch().unwrap();
///
/// // Make sure it (and the directories) exist.
/// assert!(Hello::exists().unwrap());
///
/// // Delete the project directory.
/// Hello::rm_rf().unwrap();
///
/// // Make sure the file no longer exist.
/// assert!(!Hello::exists().unwrap());
/// ```
/// This creates a file called `hello`, containing no data. The `bool` is ignored.
///
/// The `PATH` on Linux would be: `~/.local/share/disk_test/signal/hello`.
pub trait Empty: serde::Serialize + serde::de::DeserializeOwned {
	// Common path methods.
	common::impl_common!("");

	/// Try creating an empty file associated with this struct.
	///
	/// Calling this will automatically create the directories leading up to the file.
	fn touch() -> Result<(), anyhow::Error> {
		// Create PATH.
		let mut path = Self::base_path()?;
		std::fs::create_dir_all(&path)?;
		path.push(Self::FILE_NAME);

		// Create file.
		std::fs::File::create(path)?;
		Ok(())
	}
}

/// Quickly implement the [`Empty`] trait.
///
/// No file extension.
#[macro_export]
macro_rules! empty_file {
	($type:ty, $dir:expr, $project_directory:expr, $sub_directories:expr, $file_name:expr) => {
		const_assert!(const_format!("{}", $project_directory).len() != 0);
		const_assert!(const_format!("{}", $file_name).len() != 0);
 		impl Empty for $type {
			const OS_DIRECTORY: Dir = $dir;
			const PROJECT_DIRECTORY: &'static str = $project_directory;
			const SUB_DIRECTORIES: &'static str = $sub_directories;
			const FILE_NAME: &'static str = $file_name;
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}
