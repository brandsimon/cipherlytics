pub type BoxError = std::boxed::Box<dyn std::error::Error>;

pub trait NumBytes {
	const BYTES: u8;
}

impl NumBytes for u8 {
	const BYTES: u8 = 1;
}

impl NumBytes for u16 {
	const BYTES: u8 = 2;
}

impl NumBytes for u32 {
	const BYTES: u8 = 4;
}

impl NumBytes for u64 {
	const BYTES: u8 = 8;
}

impl NumBytes for u128 {
	const BYTES: u8 = 16;
}


#[cfg(test)]
mod tests {
	use crate::types::NumBytes;

	#[test]
	fn num_bytes() {
		assert_eq!(u8::BYTES, 1);
		assert_eq!(u16::BYTES, 2);
		assert_eq!(u32::BYTES, 4);
		assert_eq!(u64::BYTES, 8);
		assert_eq!(u128::BYTES, 16);
	}
}
