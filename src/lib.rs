//! Disk: [`serde`](https://docs.rs/serde) + [`directories`](https://docs.rs/directories) + various file formats as [`Traits`](https://doc.rust-lang.org/book/ch10-02-traits.html).
//!
//! This crate is for (de)serializing to/from various file formats (provided by `serde`) to/from disk locations that follow OS-specific specifications/conventions (provided by `directories`).
//!
//! All errors returned will be an [`anyhow::Error`].

//------------------------------------------------------------------------------------------------------------------------
//! # Implementing `disk`
//! ```rust
//! use serde::{Serialize, Deserialize};
//! use disk::Toml;
//!
//! #[derive(Serialize,Deserialize)] // <- Your data must implement `serde`.
//! struct State {
//! 	string: String,
//! 	number: u32,
//! }
//! // To make this struct a file, use the following macro:
//! //
//! //    |- 1. The file format used will be TOML.
//! //    |
//! //    |     |- 2. This is implemented for the struct "State".
//! //    |     |
//! //    |     |      |- 3. It will be saved in the OS Data directory.
//! //    |     |      |
//! //    |     |      |                 |- 4. The main project directory is called "MyProject".
//! //    |     |      |                 |
//! //    |     |      |                 |            |- 6. It won't be in any sub-directories.
//! //    |     |      |                 |            |
//! //    |     |      |                 |            |   |- 7. The file name will be "state.toml".
//! //    v     v      v                 v            v   v
//! disk::toml!(State, disk::Dir::Data, "MyProject", "", "state");
//! ```
//!
//! Now our `State` struct implements the `Toml` trait.
//!
//! The PATH the file would be saved in would be:
//!
//! | OS      | PATH                                                             |
//! |---------|------------------------------------------------------------------|
//! | Windows | `C:\Users\Alice\AppData\Roaming\My_Project\state.toml`           |
//! | macOS   | `/Users/Alice/Library/Application Support/My-Project/state.toml` |
//! | Linux   | `/home/alice/.local/share/myproject/state.toml`                  |

//------------------------------------------------------------------------------------------------------------------------
//! ### `.save()` and `.from_file()`
//! These two functions are the basic ways to:
//! - _Save_ a struct to disk
//! - _Create_ a struct from disk
//! ```rust
//! # use serde::{Serialize, Deserialize};
//! # use disk::*;
//! #
//! # disk::toml!(State, disk::Dir::Data, "MyProject", "", "state");
//! # #[derive(PartialEq,Serialize,Deserialize)]
//! # struct State {
//! #    string: String,
//! #    number: u32,
//! # }
//! // Create our struct.
//! let my_state = State { string: "Hello".to_string(), number: 123 };
//!
//! // Save our `State` as a `Toml` file.
//! match my_state.save() {
//! 	Ok(_) => println!("We saved to disk"),
//! 	Err(e) => eprintln!("We failed to save to disk"),
//! }
//!
//! // Let's create a new `State` by reading the file that we just created:
//! let from_disk = State::from_file().expect("Failed to read disk");
//!
//! // These should be the same.
//! assert!(my_state == from_disk);
//! ```

//------------------------------------------------------------------------------------------------------------------------
//! ### `.save_atomic()`
//! `disk` provides an `atomic` version of `.save()`.
//!
//! Atomic in this context means, the data will be saved to a TEMPORARY file first, then renamed to the associated file.
//!
//! This lowers the chance for data corruption on interrupt.
//!
//! The temporary file is removed if the rename fails.
//!
//! The temporary file name is: `file_name` + `extension` + `.tmp`, for example:
//! ```text,ignore
//! config.toml     // <- Real file
//! config.toml.tmp // <- Temporary version
//! ```
//! Already existing `.tmp` files will be overwritten.

//------------------------------------------------------------------------------------------------------------------------
//! ### `.save_gzip()` & `.from_file_gzip()`
//! `disk` provides `gzip` versions of `.save()` and `.from_file()`.
//!
//! This saves the file as a compressed file using `gzip`.
//!
//! This will suffix the file with `.gz`, for example:
//! ```text,ignore
//! config.json    // Normal file name with `.save()`
//! config.json.gz // File name when using `.save_gzip()`
//! ```
//! To recover data from this file, you _must_ also use the matching `.from_file_gzip()` when reading the data.

//------------------------------------------------------------------------------------------------------------------------
//! ### Sub-Directories
//! Either a single or multiple sub-directories can be specified with a `/` delimiter.
//!
//! `\` is also allowed but ONLY if building on Windows.
//!
//! An empty string `""` means NO sub directories.
//! ```rust,ignore
//! # use disk::Dir::Data;
//! // Windows ... C:\Users\Alice\AppData\Roaming\My_Project\sub1\sub2\state.toml
//! disk::toml!(State, Data, "MyProject", r"sub1\sub2", "state");
//!
//! // macOS ... /Users/Alice/Library/Application Support/My-Project/sub1/sub2/state.json
//! disk::json!(State, Data, "MyProject", "sub1/sub2", "state");
//!
//! // Linux ... /home/alice/.local/share/myproject/sub1/sub2/state.yml
//! disk::yaml!(State, Data, "MyProject", "sub1/sub2", "state");
//!
//! // NO sub directory:
//! disk::toml!(State, Data, "MyProject", "", "state");
//! ```

//! ### Manually implementing `disk`
//! Manually implementing `disk`'s traits is possible as well. It requires 4 constants to be defined.
//!
//! The file extension (`.bin`, `.toml`, `.json`, `.bson`, etc) is inferred based on what trait you use.
//! ```rust,ignore
//! impl disk::Toml for State {
//!     // Which OS directory it will be saved in.
//! 	const OS_DIRECTORY: disk::Dir = disk::Dir::Data;
//!     // Which the main project directory is called.
//! 	const PROJECT_DIRECTORY: &'static str = "MyProject";
//!     // If it should be in any sub-directories.
//!     const SUB_DIRECTORIES: &'static str = ""
//!     // What the saved filename will be.
//! 	const FILE_NAME: &'static str = "state";
//! }
//! ```

//------------------------------------------------------------------------------------------------------------------------
//! ### `bincode` Header and Version
//! `disk` provides a custom header and versioning feature for the binary format, `bincode`.
//!
//! The custom header is an arbitrary `24` byte array that is appended to the front of all files.
//!
//! The version is a single `u8` that comes after the header, representing a version from `0-255`.
//!
//! These must be passed to the implementation macro.
//!
//! Example:
//! ```rust
//! # use serde::{Serialize, Deserialize};
//! # use disk::*;
//! const HEADER: [u8; 24] = [255_u8; 24];
//! const VERSION: u8 = 5;
//!
//! // Define.
//! disk::bincode!(State, disk::Dir::Data, "MyProject", "", "state", HEADER, VERSION);
//! #[derive(Serialize,Deserialize)]
//! struct State {
//! 	string: String,
//! 	number: u32,
//! }
//!
//! // Save file.
//! let state = State { string: "Hello".to_string(), number: 123 };
//! state.save().unwrap();
//!
//! // Assert the file's header+version on
//! // disk is correct and extract our version.
//! let version = State::file_version().unwrap();
//! assert!(version == State::VERSION);
//! ```
//! The header and version make up the first `25` bytes of the file, byte `1..=24` being the header and
//! byte `25` being the version. These bytes are checked upon using any `.from_file()` variant and will
//! return an error if it does not match your struct's implementation.

//------------------------------------------------------------------------------------------------------------------------
//! ### File Formats
//! No file formats are enabled by default, you must enable them with feature flags.
//!
//! Use the `full` feature flag to enable _everything_.
//!
//! | File Format | Feature flag to enable |
//! |-------------|------------------------|
//! | Bincode     | `bincode`
//! | Postcard    | `postcard`
//! | JSON        | `json`
//! | TOML        | `toml`
//! | YAML        | `yaml`
//! | Pickle      | `pickle`
//! | MessagePack | `messagepack`
//! | BSON        | `bson`
//! | Plain Text  | `plain`
//! | Empty File  | `empty`

//------ Common
mod common;
pub use crate::common::Dir as Dir;

//------ Hidden re-exports
//#[doc(hidden)]
//pub use inherent::inherent as inherent;
#[doc(hidden)]
pub use const_format::assertcp as const_assert;
#[doc(hidden)]
pub use const_format::formatcp as const_format;

//------ File formats
#[cfg(feature = "bincode")]
mod bincode;
#[cfg(feature = "bincode")]
pub use crate::bincode::Bincode as Bincode;

#[cfg(feature = "postcard")]
mod postcard;
#[cfg(feature = "postcard")]
pub use crate::postcard::Postcard as Postcard;

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "json")]
pub use crate::json::Json as Json;

#[cfg(feature = "toml")]
mod toml;
#[cfg(feature = "toml")]
pub use crate::toml::Toml as Toml;

#[cfg(feature = "yaml")]
mod yaml;
#[cfg(feature = "yaml")]
pub use crate::yaml::Yaml as Yaml;

#[cfg(feature = "pickle")]
mod pickle;
#[cfg(feature = "pickle")]
pub use crate::pickle::Pickle as Pickle;

#[cfg(feature = "messagepack")]
mod messagepack;
#[cfg(feature = "messagepack")]
pub use crate::messagepack::MessagePack as MessagePack;

#[cfg(feature = "bson")]
mod bson;
#[cfg(feature = "bson")]
pub use crate::bson::Bson as Bson;

#[cfg(feature = "plain")]
mod plain;
#[cfg(feature = "plain")]
pub use crate::plain::Plain as Plain;

#[cfg(feature = "empty")]
mod empty;
#[cfg(feature = "empty")]
pub use crate::empty::Empty as Empty;
