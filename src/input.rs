use std::io;
use std::fs::File;
use std::io::Read;
use std::ops::Shl;
use std::ops::AddAssign;
use super::types::NumBytes;


pub fn read_file(filename: &str) -> Result<Vec<u8>, io::Error> {
	let mut file = File::open(filename)?;
	let mut vec: Vec<u8> = Vec::new();
	file.read_to_end(&mut vec)?;
	Ok(vec)
}


pub fn convert_vec<T: Copy + NumBytes + AddAssign + Shl + From<u8> +
               From<<T as Shl>::Output>>(vec: &Vec<u8>) -> Result<Vec<T>, io::Error> {
	let num_bytes = T::BYTES;
	let num_bytes_usize = usize::from(num_bytes);
	if vec.len() % num_bytes_usize != 0 {
		return Err(io::Error::new(io::ErrorKind::Other,
		                          "Vector length needs to be a multiple of T's length"));
	}
	let new_len: usize = vec.len() / num_bytes_usize;
	let mut conv: Vec<T> = Vec::with_capacity(new_len);
	let three: u8 = 3;
	let t_three: T = T::from(three);
	for i in 0..new_len {
		let mut val: T = T::from(0);
		for j in 0..num_bytes {
			let k: T = T::from(num_bytes - j - 1);
			let shl: T = T::from(k << t_three); // * 8
			let pos = i * num_bytes_usize + usize::from(j);
			val += T::from(T::from(vec[pos]) << shl);
		}
		conv.push(val);
	}
	Ok(conv)
}


#[cfg(test)]
mod tests {
	use std::io;
	use crate::input::convert_vec;
	use crate::input::read_file;
	use std::ops::Shl;
	use std::ops::AddAssign;
	use crate::types::NumBytes;

	#[test]
	fn test_convert_vec() -> Result<(), io::Error> {
		let orig_vec: Vec<u8> = vec![
			0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80,
			0x11, 0x22, 0x44, 0x88, 0x18, 0x24, 0x42, 0x00,
			0x12, 0x34, 0x56, 0x78, 0x90, 0x55, 0x33, 0x99,
			0x12, 0x34, 0x56, 0x78, 0x90, 0x55, 0x33, 0x99];
		let vec8 = convert_vec::<u8>(&orig_vec)?;
		assert_eq!(orig_vec, vec8);
		let vec16 = convert_vec::<u16>(&orig_vec)?;
		assert_eq!(vec16,
			[0x0102, 0x0408, 0x1020, 0x4080,
			 0x1122, 0x4488, 0x1824, 0x4200,
			 0x1234, 0x5678, 0x9055, 0x3399,
			 0x1234, 0x5678, 0x9055, 0x3399]);

		let vec32 = convert_vec::<u32>(&orig_vec)?;
		assert_eq!(vec32,
			[0x01020408, 0x10204080,
			 0x11224488, 0x18244200,
			 0x12345678, 0x90553399,
			 0x12345678, 0x90553399]);
		let vec64 = convert_vec::<u64>(&orig_vec)?;
		assert_eq!(vec64,
			[0x0102040810204080,
			 0x1122448818244200,
			 0x1234567890553399,
			 0x1234567890553399]);
		let vec128 = convert_vec::<u128>(&orig_vec)?;
		assert_eq!(vec128,
			[0x01020408102040801122448818244200,
			 0x12345678905533991234567890553399]);
		return Ok(());
	}

	fn check_convert_vec_error(some_err: Option<io::Error>) -> Result<(), ()> {
		if let Some(err) = some_err {
			assert_eq!(err.kind(), io::ErrorKind::Other);
			assert_eq!(err.to_string(),
			           "Vector length needs to be a multiple of T's length");
			return Ok(());
		}
		return Err(());
	}

	fn create_test_vec(size: usize) -> Vec<u8> {
		let mut vec = Vec::new();
		for _i in 0..size {
			vec.push(2);
		}
		return vec;
	}

	fn test_convert_vec_auto<T: Copy + NumBytes + AddAssign + Shl + From<u8> +
                                 From<<T as Shl>::Output>>() -> Result<(), ()> {
		for i in 1..T::BYTES {
			let vec = create_test_vec(usize::from(T::BYTES + i));
			check_convert_vec_error(convert_vec::<T>(&vec).err())?;
		}
		return Ok(());
	}

	#[test]
	fn test_convert_vec_errors() -> Result<(), ()> {
		test_convert_vec_auto::<u16>()?;
		test_convert_vec_auto::<u32>()?;
		test_convert_vec_auto::<u64>()?;
		test_convert_vec_auto::<u128>()?;
		return Ok(());
	}

	#[test]
	fn test_read_file() {
		let vec = read_file("tests/files/read_file").unwrap();
		assert_eq!(
			vec,
			[0xde, 0xad, 0xbe, 0xef, 0x00, 0x00, 0x99, 0x88, 0x55, 0x11, 0x33])
	}

	#[test]
	fn test_read_file_errors() -> Result<(), ()> {
		if let Some(result) = read_file("tests/files/inexistant").err() {
			assert_eq!(result.kind(), io::ErrorKind::NotFound);
			assert_eq!(result.to_string(), "No such file or directory (os error 2)");
			return Ok(());
		}
		return Err(());
	}
}
