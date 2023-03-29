# Disk
[![Windows](https://github.com/hinto-janai/disk/actions/workflows/windows.yml/badge.svg)](https://github.com/hinto-janai/disk/actions/workflows/windows.yml) [![macOS](https://github.com/hinto-janai/disk/actions/workflows/macos.yml/badge.svg)](https://github.com/hinto-janai/disk/actions/workflows/macos.yml) [![Linux](https://github.com/hinto-janai/disk/actions/workflows/linux.yml/badge.svg)](https://github.com/hinto-janai/disk/actions/workflows/linux.yml) [![crates.io](https://img.shields.io/crates/v/disk.svg)](https://crates.io/crates/disk) [![docs.rs](https://docs.rs/disk/badge.svg)](https://docs.rs/disk)

Disk: [`serde`](https://docs.rs/serde) + [`directories`](https://docs.rs/directories) + a whole bunch of file formats as [`Traits`](https://doc.rust-lang.org/book/ch10-02-traits.html).

This crate is for (de)serializing to/from various file formats (provided by `serde`) to/from disk locations that follow OS-specific specifications/conventions (provided by `directories`).

All errors returned are of type [`anyhow::Error`](https://github.com/dtolnay/anyhow).

## File Formats
Use the feature flag `full` to enable _everything_.

| File Format | Feature flag to enable |
|-------------|------------------------|
| Bincode     | `bincode`
| JSON        | `json`
| TOML        | `toml`
| YAML        | `yaml`
| Pickle      | `pickle`
| MessagePack | `messagepack`
| BSON        | `bson`
| Plain Text  | `plain`
| Empty File  | `empty`

## Example
Defining our struct, `State`:
```rust
use disk::prelude::*;       // Necessary imports to get things working.
use disk::{Toml,toml_file}; // <- TOML trait & macro.
use serde::{Serialize, Deserialize};

// To make this struct a file, use the following macro:
//
// |- 1. The file format used will be TOML.
// |
// |          |- 2. The struct "State" will be used.
// |          |
// |          |      |- 3. It will be saved in the OS Data directory.
// |          |      |
// |          |      |          |- 4. The main project directory is called "MyProject".
// |          |      |          |
// |          |      |          |            |- 6. It won't be in any sub-directories.
// |          |      |          |            |
// |          |      |          |            |   |- 7. The file name will be "state.toml".
// v          v      v          v            v   v
   toml_file!(State, Dir::Data, "MyProject", "", "state");
#[derive(Serialize,Deserialize)] // <- Your data must implement `serde`.
struct State {
    string: String,
    number: u32,
}
```

Saving `State` to disk:
```rust
let state = State { string: "Hello".to_string(), number: 0 };

// This saves to `~/.local/share/myproject/state.toml`
state.save().unwrap();
```

Creating a `State` _from_ disk:
```rust
// This reads from `~/.local/share/myproject/state.toml`
let state = State::from_file().unwrap();
```
