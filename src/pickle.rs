//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use std::path::PathBuf;
use crate::common;
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};

//---------------------------------------------------------------------------------------------------- Rmp
crate::common::impl_macro!(Pickle, "pickle");

/// [`Pickle`](https://docs.rs/serde_pickle) (binary) file format
///
/// File extension is `.pickle`.
///
/// ## Safety
/// When manually implementing, you are **promising** that the `PATH`'s manually specified are correct.
pub unsafe trait Pickle: serde::Serialize + serde::de::DeserializeOwned {
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

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}
