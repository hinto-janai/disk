# Disk
[![Windows](https://github.com/hinto-janai/disk/actions/workflows/windows.yml/badge.svg)](https://github.com/hinto-janai/disk/actions/workflows/windows.yml) [![macOS](https://github.com/hinto-janai/disk/actions/workflows/macos.yml/badge.svg)](https://github.com/hinto-janai/disk/actions/workflows/macos.yml) [![Linux](https://github.com/hinto-janai/disk/actions/workflows/linux.yml/badge.svg)](https://github.com/hinto-janai/disk/actions/workflows/linux.yml) [![crates.io](https://img.shields.io/crates/v/disk.svg)](https://crates.io/crates/disk) [![docs.rs](https://docs.rs/disk/badge.svg)](https://docs.rs/disk)

Disk: [`serde`](https://docs.rs/serde) + [`directories`](https://docs.rs/directories) + various file formats as [`Traits`](https://doc.rust-lang.org/book/ch10-02-traits.html).

This crate is for:

- (De)serializing various file formats (provided by `serde`)
- To/from disk locations that follow OS specifications (provided by `directories`)

All errors returned are of type [`anyhow::Error`](https://github.com/dtolnay/anyhow).

Full documentation @ https://docs.rs/disk
