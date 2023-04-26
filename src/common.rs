//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure,Error};
use directories::ProjectDirs;
use serde::{Serialize,Deserialize};
use std::path::{Path,PathBuf};
use crate::Dir;

//---------------------------------------------------------------------------------------------------- Constants
pub(crate) const DASH: &str = "--------------------------------------------";

//---------------------------------------------------------------------------------------------------- Common Functions.
#[inline(always)]
// Create the `ProjectDirs` struct from a project name.
pub(crate) fn base(project_name: &str) -> Result<ProjectDirs, Error> {
	match ProjectDirs::from("", "", project_name) {
		Some(p) => Ok(p),
		None    => Err(anyhow!("User directories could not be found")),
	}
}

// Get the absolute OS + Project PATH.
pub(crate) fn get_projectdir(dir: &Dir, project_name: &str) -> Result<PathBuf, Error> {
	let project_dir = base(project_name)?;

	use Dir::*;
	Ok(match &dir {
		Project    => project_dir.project_path(),
		Cache      => project_dir.cache_dir(),
		Config     => project_dir.config_dir(),
		Data       => project_dir.data_dir(),
		DataLocal  => project_dir.data_local_dir(),
		Preference => project_dir.preference_dir(),
	}.to_path_buf())
}

#[inline(always)]
// Some errors don't work with `anyhow` since they don't implement `std::error::Error`
// but they usually do implement `Display`, so use that and rewrap the `Result`.
pub(crate) fn convert_error<T, E: std::fmt::Display + std::fmt::Debug + Send + Sync + 'static>(result: Result<T, E>) -> Result<T, Error> {
	match result {
		Ok(t)  => Ok(t),
		Err(e) => Err(anyhow!(e)),
	}
}

#[inline(always)]
// Assert PATH is safe (absolute).
pub(crate) fn assert_safe_path(path: &Path) -> Result<(), Error> {
	if !path.is_absolute() { bail!("Aborting: dangerous PATH detected") }

	Ok(())
}

//---------------------------------------------------------------------------------------------------- impl_file_bytes
// Implements `file_bytes()` for 32/64bit.
macro_rules! impl_file_bytes {
	($bit:literal, $unsigned:tt) => {
		#[inline]
		#[cfg(target_pointer_width = $bit)]
		/// Reads a range of bytes of the associated file of [`Self`].
		///
		/// ## Errors
		/// If `start` is greater than `end`, this returns error.
		fn file_bytes(start: usize, end: usize) -> Result<Vec<u8>, anyhow::Error> {
			use std::io::Read;
			use std::io::{Seek,SeekFrom};

			if start > end {
				bail!("file_bytes(): start > end");
			}

			let mut buf = {
				if start == end {
					Vec::with_capacity(1)
				} else {
					Vec::with_capacity(end - start)
				}
			};

			let file = std::fs::File::open(Self::absolute_path()?)?;
			let mut file = std::io::BufReader::new(file);

			file.seek(SeekFrom::Start(start as $unsigned))?;
			file.read_exact(&mut buf)?;

			Ok(buf)
		}
	}
}
pub(crate) use impl_file_bytes;

//---------------------------------------------------------------------------------------------------- impl_io
// Implements I/O methods for all traits.
macro_rules! impl_io {
	($file_ext:literal) => {
		#[inline(always)]
		/// Read the file directly as bytes.
		fn read_to_bytes() -> Result<Vec<u8>, anyhow::Error> {
			Ok(std::fs::read(Self::absolute_path()?)?)
		}

		/// Read the file directly as bytes, and attempt `gzip` decompression.
		///
		/// This assumes the file is suffixed with `.gz`, for example:
		/// ```text,ignore
		/// config.json    // What `.read_to_bytes()` will look for
		/// config.json.gz // What `.read_to_bytes_gzip()` will look for
		/// ```
		fn read_to_bytes_gzip() -> Result<Vec<u8>, anyhow::Error> {
			use std::io::prelude::*;
			use flate2::read::GzDecoder;

			// Buffer to store decompressed bytes.
			let mut buf = Vec::new();

			// Decode compressed file bytes into buffer.
			GzDecoder::new(
				&std::fs::read(Self::absolute_path_gzip()?)?[..]
			).read_to_end(&mut buf)?;

			Ok(buf)
		}

		#[inline(always)]
		/// Same as `Self::exists()` but checks if the `gzip` file exists.
		///
		/// - `Self::exists()` checks for `file.toml`.
		/// - `Self::exists_gzip()` checks for `file.toml.gz`.
		fn exists_gzip() -> Result<bool, anyhow::Error> {
			Ok(PathBuf::from(Self::absolute_path_gzip()?).exists())
		}

		#[inline(always)]
		/// Read the file directly as bytes and turn into a Rust structure.
		fn from_file() -> Result<Self, anyhow::Error> {
			Ok(Self::from_bytes(&Self::read_to_bytes()?)?)
		}

		#[inline(always)]
		/// Read the file directly as bytes, decompress with `gzip` and turn into a Rust structure.
		fn from_file_gzip() -> Result<Self, anyhow::Error> {
			Ok(Self::from_bytes(&Self::read_to_bytes_gzip()?)?)
		}

		/// Try saving as a file.
		///
 		/// Calling this will automatically create the directories leading up to the file.
		fn save(&self) -> Result<(), anyhow::Error> {
			// Create PATH.
			let mut path = Self::base_path()?;
			std::fs::create_dir_all(&path)?;
			path.push(Self::FILE_NAME);

			// Write.
			std::fs::write(path, self.into_writable_fmt()?)?;
			Ok(())
		}

		/// Try saving as a compressed file using `gzip`.
		///
		/// This will suffix the file with `.gz`, for example:
		/// ```text,ignore
		/// config.json    // Normal file name with `.save()`
		/// config.json.gz // File name when using `.save_gzip()`
		/// ```
		///
		/// Calling this will automatically create the directories leading up to the file.
		fn save_gzip(&self) -> Result<(), anyhow::Error> {
			use std::io::prelude::*;
			use flate2::Compression;
			use flate2::write::GzEncoder;

			// Create PATH.
			let mut path = Self::base_path()?;
			std::fs::create_dir_all(&path)?;
			path.push(Self::FILE_NAME_GZIP);

			// Compress bytes and write.
			let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
			encoder.write_all(&self.to_bytes()?[..])?;
			std::fs::write(path, encoder.finish()?)?;

			Ok(())
		}

		/// **Note: This may not truely be atomic on Windows.**
		///
		/// Try saving to a TEMPORARY file first, then renaming it to the associated file.
		///
		/// This lowers the chance for data corruption on interrupt.
		///
		/// The temporary file is removed if the rename fails.
		///
		/// The temporary file name is: `file_name` + `extension` + `.tmp`, for example:
		/// ```text,ignore
		/// config.toml     // <- Real file
		/// config.toml.tmp // <- Temporary version
		/// ```
		/// Already existing `.tmp` files will be overwritten.
		///
		/// Calling this will automatically create the directories leading up to the file.
		fn save_atomic(&self) -> Result<(), anyhow::Error> {
			// Create PATH.
			let mut path = Self::base_path()?;
			std::fs::create_dir_all(&path)?;

			// TMP and normal PATH.
			let mut tmp = path.clone();
			tmp.push(Self::FILE_NAME_TMP);
			path.push(Self::FILE_NAME);

			// Write to TMP.
			if let Err(e) = std::fs::write(&tmp, self.into_writable_fmt()?) {
				std::fs::remove_file(&tmp)?;
				bail!(e);
			}

			// Rename TMP to normal.
			if let Err(e) = std::fs::rename(&tmp, &path) {
				std::fs::remove_file(&tmp)?;
				bail!(e);
			}

			Ok(())
		}

		/// Combines [`Self::save_gzip()`] and [`Self::save_atomic()`].
		fn save_atomic_gzip(&self) -> Result<(), anyhow::Error> {
			use std::io::prelude::*;
			use flate2::Compression;
			use flate2::write::GzEncoder;

			// Create PATH.
			let mut path = Self::base_path()?;
			std::fs::create_dir_all(&path)?;

			// Create TMP and normal.
			let mut tmp = path.clone();
			tmp.push(Self::FILE_NAME_GZIP_TMP);
			path.push(Self::FILE_NAME_GZIP);

			// Compress bytes.
			let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
			encoder.write_all(&self.to_bytes()?[..])?;

			// Write to TMP.
			if let Err(e) = std::fs::write(&tmp, &encoder.finish()?) {
				std::fs::remove_file(&tmp)?;
				bail!(e);
			}

			// Rename TMP to normal.
			if let Err(e) = std::fs::rename(&tmp, &path) {
				std::fs::remove_file(&tmp)?;
				bail!(e);
			}

			Ok(())
		}

		/// **Note: This may not truely be atomic on Windows.**
		///
		/// Rename the associated file before attempting to delete it.
		///
		/// This lowers the chance for data corruption on interrupt.
		///
		/// The temporary file name is: `file_name` + `extension` + `.tmp`, for example:
		/// ```text,ignore
		/// config.toml     // <- Real file
		/// config.toml.tmp // <- Temporary version
		/// ```
		/// Already existing `.tmp` files will be overwritten.
		fn rm_atomic(&self) -> Result<(), anyhow::Error> {
			let mut path = Self::base_path()?;

			let mut tmp = path.clone();
			tmp.push(Self::FILE_NAME_TMP);
			path.push(Self::FILE_NAME);

			if !path.exists() { return Ok(()) }

			std::fs::rename(&path, &tmp)?;
			std::fs::remove_file(&tmp)?;

			Ok(())
		}

		/// Same as [`Self::rm_atomic()`] but looks for the `.gz` extension.
		fn rm_atomic_gzip(&self) -> Result<(), anyhow::Error> {
			let mut path = Self::base_path()?;

			let mut tmp = path.clone();
			tmp.push(Self::FILE_NAME_TMP);
			path.push(Self::FILE_NAME_GZIP);

			if !path.exists() { return Ok(()) }

			std::fs::rename(&path, &tmp)?;
			std::fs::remove_file(&tmp)?;

			Ok(())
		}

		/// Try deleting any leftover `.tmp` files from [`Self::save_atomic()`] or [`Self::save_atomic_gzip()`]
		///
		/// This will return success if the files don't exist or if deleted.
		///
		/// It will return failure if files existed but could not be deleted or if any other error occurs.
		fn rm_tmp() -> Result<(), anyhow::Error> {
			let mut tmp = Self::base_path()?;
			let mut gzip = tmp.clone();

			tmp.push(Self::FILE_NAME_TMP);
			gzip.push(Self::FILE_NAME_GZIP_TMP);

			if !tmp.exists() && !gzip.exists() { return Ok(()) }

			std::fs::remove_file(tmp)?;
			std::fs::remove_file(gzip)?;
			Ok(())
		}

		#[inline(always)]
		/// The absolute PATH of the file associated with this struct WITH the `.gz` extension.
		fn absolute_path_gzip() -> Result<PathBuf, anyhow::Error> {
			let mut base = Self::base_path()?;
			base.push(Self::FILE_NAME_GZIP);

			common::assert_safe_path(&base)?;

			Ok(base)
		}

		$crate::common::impl_file_bytes!("64", u64);
		$crate::common::impl_file_bytes!("32", u32);
	}
}
pub(crate) use impl_io;

//---------------------------------------------------------------------------------------------------- impl_common
// Implements the CONSTANTS and common PATH methods for all traits.
macro_rules! impl_common {
	($file_ext:literal) => {
		/// Which OS directory it will be saved in.
		const OS_DIRECTORY: $crate::Dir;
		/// What the main project directory will be.
		const PROJECT_DIRECTORY: &'static str;
		/// Optional sub directories in between the project directory and file.
		const SUB_DIRECTORIES: &'static str;
		/// What the raw file name will be (no extension).
		const FILE: &'static str;
		/// What the file extension will be.
		const FILE_EXT: &'static str;
		/// What the full filename + extension will be.
		const FILE_NAME: &'static str;
		/// What the `gzip` variant of the filename will be.
		const FILE_NAME_GZIP: &'static str;
		/// What the `tmp` variant of the filename will be.
		const FILE_NAME_TMP: &'static str;
		/// What the `gzip` + `tmp` variant of the filename will be.
		const FILE_NAME_GZIP_TMP: &'static str;

		#[inline]
		/// Create the directories leading up-to the file.
		///
		/// This is not necessary when using any variant of
		/// `Self::save()` as the directories are created implicitly.
		fn mkdir() -> Result<(), anyhow::Error> {
			Ok(std::fs::create_dir_all(Self::base_path()?)?)
		}

		#[inline]
		/// Recursively remove this file's project directory.
		///
		/// This deletes _all_ directories starting from [`Self::PROJECT_DIRECTORY`].
		/// For example:
		/// ```rust,ignore
		/// disk::toml!(State, disk::Dir::Data, "MyProject", "sub_dir", "state");
		/// ```
		/// This project's file would be located at `~/.local/share/myproject`.
		/// This is the `PATH` that gets removed recursively.
		///
		/// This is akin to running:
		/// ```ignore
		/// rm -rf ~/.local/share/myproject
		/// ```
		/// The input to all `disk` macros are sanity checked.
		/// The worst you can do with this function is delete your project's directory.
		///
		/// This function calls [`std::fs::remove_dir_all`], which does _not_ follow symlinks.
		fn rm_rf() -> Result<(), anyhow::Error> {
			Ok(std::fs::remove_dir_all(Self::base_path()?)?)
		}

		/// Try deleting the file.
		///
		/// This will return success if the file doesn't exist or if deleted.
		///
		/// It will return failure if the file existed but could not be deleted or if any other error occurs.
		fn rm() -> Result<(), anyhow::Error> {
			let mut path = Self::base_path()?;
			path.push(Self::FILE_NAME);

			if !path.exists() { return Ok(()) }

			Ok(std::fs::remove_file(path)?)
		}

		#[inline(always)]
		/// Check if the file exists.
		///
		/// - `true`  == The file exists.
		/// - `false` == The file does not exist.
		/// - `anyhow::Error` == There was an error, existance is unknown.
		fn exists() -> Result<bool, anyhow::Error> {
			let path = Self::absolute_path()?;

			Ok(path.exists())
		}

		#[inline(always)]
		/// Returns the file size in bytes.
		fn file_size() -> Result<u64, anyhow::Error> {
			let path = Self::absolute_path()?;
			let file = std::fs::File::open(path)?;

			Ok(file.metadata()?.len())
		}

		#[inline(always)]
		/// Returns the file's parent sub-directory size in bytes.
		///
		/// This starts from the first [`Self::SUB_DIRECTORIES`],
		/// and does not include the [`Self::PROJECT_DIRECTORY`].
		fn sub_dir_size() -> Result<u64, anyhow::Error> {
			let path = Self::sub_dir_parent_path()?;
			let dir = std::fs::File::open(path)?;

			Ok(dir.metadata()?.len())
		}

		#[inline(always)]
		/// Returns the file's project directory size in bytes ([`Self::PROJECT_DIRECTORY`])
		fn project_dir_size() -> Result<u64, anyhow::Error> {
			let path = Self::project_dir_path()?;
			let file = std::fs::File::open(path)?;

			Ok(file.metadata()?.len())
		}

		/// Return the full parent project directory associated with this struct.
		///
		/// This is the `PATH` leading up to [`Self::PROJECT_DIRECTORY`].
		fn project_dir_path() -> Result<PathBuf, anyhow::Error> {
			// Get a `ProjectDir` from our project name.
			Ok(common::get_projectdir(&Self::OS_DIRECTORY, &Self::PROJECT_DIRECTORY)?.to_path_buf())
		}

		/// Returns the top-level parent sub-directory associated with this struct.
		///
		/// If _only_ returns the top level sub-directory, so if multiple are defined,
		/// only the first will be returned, e.g: `my/sub/dirs` would return `/.../my`
		///
		/// If no sub-directory is defined, this will return the PATH leading up to [`Self::PROJECT_DIRECTORY`].
		fn sub_dir_parent_path() -> Result<PathBuf, anyhow::Error> {
			// Get a `ProjectDir` from our project name.
			let mut base = Self::project_dir_path()?;

			// Append sub directories (if any).
			if Self::SUB_DIRECTORIES.len() != 0 {
				#[cfg(target_os = "windows")]
				if let Some(sub) = Self::SUB_DIRECTORIES.split_terminator(&['/', '\\'][..]).next() {
					base.push(sub);
				}
				#[cfg(target_family = "unix")]
				if let Some(sub) = Self::SUB_DIRECTORIES.split_terminator('/').next() {
					base.push(sub);
				}
			}

			Ok(base)
		}

		/// Returns the full base path associated with this struct (PATH leading up to the file).
		///
		/// In contrast to [`Self::sub_dir_parent_path`], this returns all sub-directories,
		/// e.g: `my/sub/dirs` would return `/.../my/sub/dirs`
		///
		/// This includes [`Self::PROJECT_DIRECTORY`], [`Self::SUB_DIRECTORIES`] and excludes [`Self::FILE_NAME`].
		fn base_path() -> Result<PathBuf, anyhow::Error> {
			// Get a `ProjectDir` from our project name.
			let mut base = Self::project_dir_path()?;

			// Append sub directories (if any).
			if Self::SUB_DIRECTORIES.len() != 0 {
				#[cfg(target_os = "windows")]
				Self::SUB_DIRECTORIES.split_terminator(&['/', '\\'][..]).for_each(|dir| base.push(dir));
				#[cfg(target_family = "unix")]
				Self::SUB_DIRECTORIES.split_terminator('/').for_each(|dir| base.push(dir));
			}

			Ok(base)
		}

		#[inline(always)]
		/// Returns the absolute PATH of the file associated with this struct.
		///
		/// This includes [`Self::PROJECT_DIRECTORY`], [`Self::SUB_DIRECTORIES`] and [`Self::FILE_NAME`].
		fn absolute_path() -> Result<PathBuf, anyhow::Error> {
			let mut base = Self::base_path()?;
			base.push(Self::FILE_NAME);

			common::assert_safe_path(&base)?;

			Ok(base)
		}
	}
}
pub(crate) use impl_common;

//---------------------------------------------------------------------------------------------------- impl_string
// Implements common methods on a [String] based trait.
// This automatically implements [impl_common!()].
macro_rules! impl_string {
	($file_ext:literal) => {
		common::impl_common!($file_ext);
		common::impl_io!($file_ext);

		#[inline(always)]
		/// Turn [`Self`] into a [`String`], maintaining formatting if possible.
		fn into_writable_fmt(&self) -> Result<String, anyhow::Error> {
			self.to_string()
		}

		#[inline(always)]
		/// Read the file directly as a [`String`].
		fn read_to_string() -> Result<String, anyhow::Error> {
			Ok(std::fs::read_to_string(Self::absolute_path()?)?)
		}

		#[inline(always)]
		#[cfg(feature = "log")]
		/// Print the file's contents to console surrounded by dashes with the [`log`] crate.
		fn info_dash(string: &str) {
			log::info!("{}", common::DASH);
			string.lines().for_each(|i| log::info!("{}", i));
			log::info!("{}", common::DASH);
		}
	};
}
pub(crate) use impl_string;

//---------------------------------------------------------------------------------------------------- impl_binary
// Implements common methods on a binary based trait.
// This automatically implements `impl_common!()`.
macro_rules! impl_binary {
	($file_ext:literal) => {
		crate::common::impl_common!($file_ext);
		crate::common::impl_io!($file_ext);

		#[inline(always)]
		/// Turn [`Self`] into bytes that can be written to disk.
		fn into_writable_fmt(&self) -> Result<Vec<u8>, anyhow::Error> {
			self.to_bytes()
		}
	};
}
pub(crate) use impl_binary;

//---------------------------------------------------------------------------------------------------- Macro for macro impl.
// Assert string is not empty.
#[doc(hidden)]
#[macro_export]
macro_rules! assert_str {
	($project_directory:tt, $file_name:tt) => {
		$crate::const_assert!($project_directory.len() != 0, "disk: 'Project Directory' must not be an empty string!");
		$crate::const_assert!($file_name.len() != 0, "disk: 'File Name' must not be an empty string!");
	}
}

// Binary files.
macro_rules! impl_macro_binary {
	($trait:ident, $file_ext:literal) => {
		use $crate::Dir;
		paste::item! {
			#[doc = "
Implement the [`" $trait "`] trait

File extension is `" $file_ext "` and is automatically appended.

### Input
These are the inputs you need to provide to implement [`" $trait "`].

| Variable             | Description                             | Related Trait Constant            | Type               | Example       |
|----------------------|-----------------------------------------|-----------------------------------|--------------------|---------------|
| `$data`              | Identifier of the data to implement for |                                   | `struct` or `enum` | `State`
| `$dir`               | Which OS directory to use               | [`" $trait "::OS_DIRECTORY`]      | [`Dir`]            | [`Dir::Data`]
| `$project_directory` | The name of the top project folder      | [`" $trait "::PROJECT_DIRECTORY`] | [`&str`]           | `\"MyProject\"`
| `$sub_directories`   | (Optional) sub-directories before file  | [`" $trait "::SUB_DIRECTORIES`]   | [`&str`]           | `\"some/dirs\"`
| `$file_name`         | The file name to use                    | [`" $trait "::FILE_NAME`]         | [`&str`]           | `\"state\"`
| `$header`            | `24` custom byte header                 | [`" $trait "::HEADER`]            | `[u8; 24]`         | `[1_u8; 24]`
| `$version`           | `1` byte custom version                 | [`" $trait "::VERSION`]           | `u8`               | `5_u8`

### Example
```rust
use serde::{Serialize,Deserialize};
use disk::*;

const HEADER: [u8; 24] = [1_u8; 24];
const VERSION: u8 = 5;

" $trait:lower "!(State, Dir::Data, \"MyProject\", \"some/dirs\", \"state\", HEADER, VERSION);
#[derive(Serialize,Deserialize)]
struct State {
    string: String,
    number: u32,
}
```

This example would be located at `~/.local/share/myproject/some/dirs/state." $file_ext "`.
"]
			#[macro_export]
			macro_rules! [<$trait:lower>] {
				($data:ty, $dir:expr, $project_directory:tt, $sub_directories:tt, $file_name:tt, $header:tt, $version:tt) => {
					$crate::assert_str!($project_directory, $file_name);

					// SAFETY: The input to this `" $trait "` implementation was verified and sanity-checked via macro.
			 		unsafe impl $crate::$trait for $data {
						const OS_DIRECTORY:       $crate::Dir  = $dir;
						const PROJECT_DIRECTORY:  &'static str = $project_directory;
						const SUB_DIRECTORIES:    &'static str = $sub_directories;
						const FILE:               &'static str = $file_name;
						const FILE_EXT:           &'static str = $file_ext;
						const FILE_NAME:          &'static str = $crate::const_format!("{}.{}", $file_name, $file_ext);
						const FILE_NAME_GZIP:     &'static str = $crate::const_format!("{}.{}.gzip", $file_name, $file_ext);
						const FILE_NAME_TMP:      &'static str = $crate::const_format!("{}.{}.tmp", $file_name, $file_ext);
						const FILE_NAME_GZIP_TMP: &'static str = $crate::const_format!("{}.{}.gzip.tmp", $file_name, $file_ext);
						const HEADER:             [u8; 24]     = $header;
						const VERSION:            u8           = $version;
					}
				}
			}
			pub(crate) use [<$trait:lower>];
		}
	};
}
pub(crate) use impl_macro_binary;

// Empty (no extension) file.
macro_rules! impl_macro_no_ext {
	($trait:ident) => {
		use $crate::Dir;
		paste::item! {
			#[doc = "
Implement the [`" $trait "`] trait

[`" $trait "`] has no file extension.

### Input
These are the inputs you need to provide to implement [`" $trait "`].

| Variable             | Description                             | Related Trait Constant            | Type               | Example       |
|----------------------|-----------------------------------------|-----------------------------------|--------------------|---------------|
| `$data`              | Identifier of the data to implement for |                                   | `struct` or `enum` | `MyState`
| `$dir`               | Which OS directory to use               | [`" $trait "::OS_DIRECTORY`]      | [`Dir`]            | [`Dir::Data`]
| `$project_directory` | The name of the top project folder      | [`" $trait "::PROJECT_DIRECTORY`] | [`&str`]           | `\"MyProject\"`
| `$sub_directories`   | (Optional) sub-directories before file  | [`" $trait "::SUB_DIRECTORIES`]   | [`&str`]           | `\"some/dirs\"`
| `$file_name`         | The file name to use                    | [`" $trait "::FILE_NAME`]         | [`&str`]           | `\"state\"`

### Example
```rust
use serde::{Serialize,Deserialize};
use disk::*;

" $trait:lower "!(State, Dir::Data, \"MyProject\", \"some/dirs\", \"state\");
#[derive(Serialize,Deserialize)]
struct State {
    string: String,
    number: u32,
}
```

This example would be located at `~/.local/share/myproject/some/dirs/state`.
"]
			#[macro_export]
			macro_rules! [<$trait:lower>] {
				($data:ty, $dir:expr, $project_directory:tt, $sub_directories:tt, $file_name:tt) => {
					$crate::assert_str!($project_directory, $file_name);

					// SAFETY: The input to this `" $trait "` implementation was verified and sanity-checked via macro.
			 		unsafe impl $crate::$trait for $data {
						const OS_DIRECTORY:      $crate::Dir  = $dir;
						const PROJECT_DIRECTORY:  &'static str = $project_directory;
						const SUB_DIRECTORIES:    &'static str = $sub_directories;
						const FILE:               &'static str = $file_name;
						const FILE_EXT:           &'static str = "";
						const FILE_NAME:          &'static str = $file_name;
						const FILE_NAME_GZIP:     &'static str = $crate::const_format!("{}.gzip", $file_name);
						const FILE_NAME_TMP:      &'static str = $crate::const_format!("{}.tmp", $file_name);
						const FILE_NAME_GZIP_TMP: &'static str = $crate::const_format!("{}.gzip.tmp", $file_name);
					}
				}
			}
			pub(crate) use [<$trait:lower>];
		}
	};
}
pub(crate) use impl_macro_no_ext;

// Regular files.
macro_rules! impl_macro {
	($trait:ident, $file_ext:literal) => {
		use $crate::Dir;
		paste::item! {
			#[doc = "
Implement the [`" $trait "`] trait

File extension is `" $file_ext "` and is automatically appended.

### Input
These are the inputs you need to provide to implement [`" $trait "`].

| Variable             | Description                             | Related Trait Constant            | Type               | Example       |
|----------------------|-----------------------------------------|-----------------------------------|--------------------|---------------|
| `$data`              | Identifier of the data to implement for |                                   | `struct` or `enum` | `MyState`
| `$dir`               | Which OS directory to use               | [`" $trait "::OS_DIRECTORY`]      | [`Dir`]            | [`Dir::Data`]
| `$project_directory` | The name of the top project folder      | [`" $trait "::PROJECT_DIRECTORY`] | [`&str`]           | `\"MyProject\"`
| `$sub_directories`   | (Optional) sub-directories before file  | [`" $trait "::SUB_DIRECTORIES`]   | [`&str`]           | `\"some/dirs\"`
| `$file_name`         | The file name to use                    | [`" $trait "::FILE_NAME`]         | [`&str`]           | `\"state\"`

### Example
```rust
use serde::{Serialize,Deserialize};
use disk::*;

" $trait:lower "!(State, Dir::Data, \"MyProject\", \"some/dirs\", \"state\");
#[derive(Serialize,Deserialize)]
struct State {
    string: String,
    number: u32,
}
```

This example would be located at `~/.local/share/myproject/some/dirs/state." $file_ext "`.
"]
			#[macro_export]
			macro_rules! [<$trait:lower>] {
				($data:ty, $dir:expr, $project_directory:tt, $sub_directories:tt, $file_name:tt) => {
					$crate::assert_str!($project_directory, $file_name);

					// SAFETY: The input to this `" $trait "` implementation was verified and sanity-checked via macro.
			 		unsafe impl $crate::$trait for $data {
						const OS_DIRECTORY:       $crate::Dir  = $dir;
						const PROJECT_DIRECTORY:  &'static str = $project_directory;
						const SUB_DIRECTORIES:    &'static str = $sub_directories;
						const FILE:               &'static str = $file_name;
						const FILE_EXT:           &'static str = $file_ext;
						const FILE_NAME:          &'static str = $crate::const_format!("{}.{}", $file_name, $file_ext);
						const FILE_NAME_GZIP:     &'static str = $crate::const_format!("{}.{}.gzip", $file_name, $file_ext);
						const FILE_NAME_TMP:      &'static str = $crate::const_format!("{}.{}.tmp", $file_name, $file_ext);
						const FILE_NAME_GZIP_TMP: &'static str = $crate::const_format!("{}.{}.gzip.tmp", $file_name, $file_ext);
					}
				}
			}
			pub(crate) use [<$trait:lower>];
		}
	};
}
pub(crate) use impl_macro;

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod test {
//  #[test]
//  fn _() {
//  }
//}
