use std::io;
use std::cmp::PartialOrd;

pub fn min_max<T: Copy + PartialOrd>(vec: &Vec<T>) -> Result<(T, T), io::Error> {
	if vec.len() == 0 {
		return Err(io::Error::new(io::ErrorKind::Other,
		                          "Cannot calculate min/max on empty input"));
	}
	let mut min: T = vec[0];
	let mut max: T = vec[0];
	for i in vec {
		if i < &min {
			min = *i;
		}
		if i > &max {
			max = *i;
		}
	}
	return Ok((min, max));
}

#[cfg(test)]
mod tests {
	use std::io;
	use crate::analytics::min_max::min_max;

	fn check_min_max_error(some_err: Option<io::Error>) -> Result<(), ()> {
		if let Some(err) = some_err {
			assert_eq!(err.kind(), io::ErrorKind::Other);
			assert_eq!(err.to_string(),
			           "Cannot calculate min/max on empty input");
			return Ok(());
		}
		return Err(());
	}

	fn text_min_max_empty_generic<T: Copy + PartialOrd>() -> Result<(), ()> {
		let vec: Vec<T> = Vec::new();
		check_min_max_error(min_max(&vec).err())?;
		return Ok(());
	}

	#[test]
	fn min_max_empty() -> Result<(), ()> {
		text_min_max_empty_generic::<u8>()?;
		text_min_max_empty_generic::<u16>()?;
		text_min_max_empty_generic::<u32>()?;
		text_min_max_empty_generic::<u64>()?;
		text_min_max_empty_generic::<u128>()?;
		return Ok(());
	}

	#[test]
	fn min_max_u8() -> Result<(), io::Error> {
		let vec_u8: Vec<u8> = vec![223, 3, 17, 25, 255, 42, 102];
		assert_eq!(min_max(&vec_u8)?, (3, 255));
		return Ok(());
	}

	#[test]
	fn min_max_u32() -> Result<(), io::Error> {
		let vec_u32: Vec<u32> = vec![223, 3, 17, 25, 255, 42, 102];
		assert_eq!(min_max(&vec_u32)?, (3, 255));
		return Ok(());
	}

	#[test]
	fn min_max_u128() -> Result<(), io::Error> {
		let vec_u128: Vec<u128> = vec![223, 3, 17, 25, 255, 42, 102];
		assert_eq!(min_max(&vec_u128)?, (3, 255));
		return Ok(());
	}
}
