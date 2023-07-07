//---------------------------------------------------------------------------------------------------- Header check/append.
macro_rules! ensure_header {
	($bytes:ident) => {
		let len = $bytes.len();

		// Ensure our `[u8; 25]` HEADER + VERSION bytes are there.
		if len < 25 {
			bail!("invalid header bytes, total byte length less than 25: {len}");
		}

		// Ensure our HEADER is correct.
		if $bytes[..24] != Self::HEADER {
			bail!("incorrect header bytes\nexpected: {:?}\nfound: {:?}", Self::HEADER, &$bytes[..24],);
		}

		// Ensure our VERSION is correct.
		if $bytes[24] != Self::VERSION {
			bail!("incorrect version byte\nexpected: {:?}\nfound: {:?}", Self::VERSION, &$bytes[24],);
		}
	}
}
pub(crate) use ensure_header;

macro_rules! header_return {
	($buf:ident) => {{
		let mut bytes = Self::full_header().to_vec();
		bytes.append(&mut $buf);

		Ok(bytes)
	}}
}
pub(crate) use header_return;

//---------------------------------------------------------------------------------------------------- Header impl.
macro_rules! impl_header {
	() => {
		/// A custom 24-byte length identifying header for your binary file.
		///
		/// This is combined with [`Self::VERSION`] to prefix your file with 25 bytes.
		///
		/// **Note: [`Self::save_gzip()`] applies compression AFTER, meaning the entire file must be decompressed to get these headers.**
		const HEADER: [u8; 24];
		/// What the version byte will be (0-255).
		const VERSION: u8;

		#[inline(always)]
		/// Read the associated file and attempt to convert the first 24 bytes to a [`String`].
		///
		/// This is useful if your [`Self::HEADER`] should be bytes representing a UTF-8 [`String`].
		fn file_header_to_string() -> Result<String, anyhow::Error> {
			let bytes = Self::file_bytes(0,24)?;
			Ok(String::from_utf8(bytes)?)
		}

		#[inline]
		/// Return the 25 bytes header bytes.
		///
		/// First 24 bytes are the [`Self::HEADER`] bytes.
		///
		/// Last byte is [`Self::VERSION`].
		fn full_header() -> [u8; 25] {
			[
				Self::HEADER[0],
				Self::HEADER[1],
				Self::HEADER[2],
				Self::HEADER[3],
				Self::HEADER[4],
				Self::HEADER[5],
				Self::HEADER[6],
				Self::HEADER[7],
				Self::HEADER[8],
				Self::HEADER[9],
				Self::HEADER[10],
				Self::HEADER[11],
				Self::HEADER[12],
				Self::HEADER[13],
				Self::HEADER[14],
				Self::HEADER[15],
				Self::HEADER[16],
				Self::HEADER[17],
				Self::HEADER[18],
				Self::HEADER[19],
				Self::HEADER[20],
				Self::HEADER[21],
				Self::HEADER[22],
				Self::HEADER[23],
				Self::VERSION
			]
		}

		#[inline]
		/// Reads the first 24 bytes of the associated file and matches it against [`Self::HEADER`].
		///
		/// If the bytes match, the next byte _may be_ be our [`Self::VERSION`] and is returned.
		///
		/// ## Note
		/// This only works on a non-compressed file.
		fn file_version() -> Result<u8, anyhow::Error> {
			use std::io::Read;

			let mut bytes = [0; 25];

			let mut file = std::fs::File::open(Self::absolute_path()?)?;

			file.read_exact(&mut bytes)?;

			if bytes[0..24] == Self::HEADER {
				Ok(bytes[24])
			} else {
				bail!("header bytes failed to match.\nexpected: {:?}\nfound: {:?}", Self::HEADER, &bytes[0..24]);
			}
		}

		#[inline]
		/// This is the function that ties the versioning system together.
		///
		/// It takes a variable static array of `(VERSION, Struct::constructor)`
		/// tuples, attempting to deserialize them starting from index `0`.
		///
		/// AKA, you give a list of versions and your choice of `disk`
		/// constructors for various versions of the same-ish struct.
		///
		/// An example:
		/// ```rust,ignore
		/// disk::bincode!(Data0, Dir::Data, "Data", "", "data", [255_u8; 24], 0); // <- note: version 0.
		/// struct Data0 {
		///     data: Vec<u8>,
		/// }
		///
		/// // This converts a `Data0` into a `Data5`
		/// impl Data0 {
		///     fn to_data5() -> Result<Data5, anyhow::Error> {
		///         match Self::from_file() {
		///             Ok(s)  => Ok(Data1 { data: s.data, ..Default::default() }),
		///             Err(e) => Err(e),
		///         }
		///     }
		/// }
		///
		/// /* ... data1, data2, data3, data4 ... */
		///
		/// disk::bincode!(Data1, Dir::Data, "Data", "", "data", [255_u8; 24], 5); // <- note: version 5.
		/// struct Data5 {
		///     data: Vec<u8>,
		///     more_data: Vec<u8>,
		/// }
		/// ```
		///
		/// The `Data0::to_data5()` can be used as the constructor for this function.
		///
		/// Now, if we'd like to deserialize `Data5`, but fallback if
		/// the file detected is an older version, we can write this:
		/// ```rust,ignore
		/// let data = Data5::from_versions(&[
		///     (5, Data5::from_file), // Your choice of function here.
		///     (4, Data4::to_data5),  // These as well.
		///     (3, Data3::to_data5),
		///     (2, Data2::to_data5),
		///     (1, Data1::to_data5),
		///     (0, Data0::to_data5),
		/// ]).unwrap();
		///```
		/// This will go top-to-bottom starting at `5`, ending at `0`,
		/// checking if the version matches, then attempting deserialization.
		///
		/// The returned `Ok(u8, Self)` contains the version that successfully
		/// matched and the resulting (converted) deserialized data.
		///
		/// The output data is always `Self`, so the `fn()` constructors you
		/// input are responsible for converting between the various types.
		fn from_versions(
			versions_and_constructors: &'static [(u8, fn() -> Result<Self, anyhow::Error>)],
		) -> Result<(u8, Self), anyhow::Error> {
			// Get on-disk version.
			let file = Self::file_version()?;

//			// If target version, attempt deserialization and return.
//			if file == Self::VERSION {
//				return match constructor() {
//					Ok(data) => Ok((file, data)),
//					Err(e)   => Err(e),
//				};
//			}

			// Else, attempt the other version constructors.
			for (version, constructor) in versions_and_constructors {
				// If not the matching version, continue.
				if file != *version {
					continue;
				}

				// If version matches, attempt to construct.
				return match constructor() {
					Ok(data) => Ok((*version, data)),
					Err(e)   => Err(e),
				};
			}

			// Return error if nothing worked.
			Err(anyhow!("all versions failed to match: {versions_and_constructors:#?}"))
		}
	}
}
pub(crate) use impl_header;
