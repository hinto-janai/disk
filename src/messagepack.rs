//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use std::path::PathBuf;
use crate::common;
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
use rmp_serde::{Deserializer, Serializer};

//---------------------------------------------------------------------------------------------------- Rmp
crate::common::impl_macro!(MessagePack, "messagepack");

/// [`MessagePack`](https://docs.rs/rmp-serde) (binary) file format
///
/// File extension is `.messagepack`.
///
/// ## Safety
/// When manually implementing, you are **promising** that the `PATH`'s manually specified are correct.
pub unsafe trait MessagePack: serde::Serialize + serde::de::DeserializeOwned {
	// Common data/functions.
	common::impl_binary!("messagepack");

	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, anyhow::Error> {
		common::convert_error(rmp_serde::decode::from_slice(bytes))
	}

	#[inline(always)]
	fn to_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
		common::convert_error(rmp_serde::encode::to_vec(self))
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//}
