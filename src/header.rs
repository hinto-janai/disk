//---------------------------------------------------------------------------------------------------- Header check/append.
macro_rules! ensure_header {
	($bytes:ident) => {
		let len = $bytes.len();

		// Ensure our `[u8; 25]` HEADER + VERSION bytes are there.
		if len < 25 {
			bail!("Invalid header bytes, total byte length less than 25: {}", len);
		}

		// Ensure our HEADER is correct.
		if $bytes[..24] != Self::HEADER {
			bail!("Incorrect header bytes\nExpected: {:?}\nFound: {:?}", Self::HEADER, &$bytes[..24],);
		}

		// Ensure our VERSION is correct.
		if $bytes[24] != Self::VERSION {
			bail!("Incorrect version byte\nExpected: {:?}\nFound: {:?}", Self::VERSION, &$bytes[24],);
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
				bail!("Bincode header failed to match.\nExpected: {:?}\nFound: {:?}", Self::HEADER, &bytes[0..24]);
			}
		}
	}
}
pub(crate) use impl_header;
