//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow};
use std::path::PathBuf;
use crate::common;

//---------------------------------------------------------------------------------------------------- Toml
crate::common::impl_macro_no_ext!(Empty);

/// [`Empty`] file
///
/// This is a an empty file that
/// - Contains no data
/// - Doesn't need `serde`
/// - Inherits useful `PATH` methods.
///
/// Typically used for file-based signals.
///
/// If you implement this on a `struct` that contains data, the data will be ignored
/// and an empty file will always be created.
///
/// The file created will have _no_ file extension, e.g:
/// ```rust
/// # use serde::{Serialize,Deserialize};
/// # use disk::*;
/// disk::empty!(Hello, Dir::Data, "disk_test", "signal", "hello");
/// #[derive(Serialize, Deserialize)]
/// struct Hello {
///     data: bool,
/// }
///
/// // The filename should be "hello".
/// assert!(Hello::FILE_NAME == "hello");
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
///
/// ## Safety
/// When manually implementing, you are **promising** that the `PATH`'s manually specified are correct.
pub unsafe trait Empty {
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

	// Common path methods.
	common::impl_common!("");
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}
