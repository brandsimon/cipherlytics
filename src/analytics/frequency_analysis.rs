use std::collections::HashMap;
use std::ops::AddAssign;
use std::hash::Hash;
use std::fmt::Display;
use std::io::Write;
use std::io;

pub fn frequency_analysis<T: Copy + AddAssign + Eq + Hash>(vec: &Vec<T>) -> HashMap<T, usize> {
	let mut map: HashMap<T, usize> = HashMap::new();
	for i in vec {
		*map.entry(*i).or_insert(0) += 1
	}
	return map;
}

pub fn print_frequency_analysis_result<
	T: Display, W: Write
>(map: HashMap<T, usize>, out: &mut W) -> Result<(), io::Error> {
	let mut vec: Vec<(&T, &usize)> = map.iter().collect();
	vec.sort_by(|a, b| b.1.cmp(a.1));
	for (i, j) in vec {
		writeln!(out, "{}: {}", j, i)?;
	}
	return Ok(());
}

#[cfg(test)]
mod tests {
	use crate::analytics::frequency_analysis::frequency_analysis;
	use crate::analytics::frequency_analysis::print_frequency_analysis_result;
	use std::collections::HashMap;
	use std::io::Write;

	#[test]
	fn frequency_analysis_u8() {
		let vec_u8: Vec<u8> = vec![223, 3, 17, 223, 255, 42, 3, 17, 223];
		let result_u8 = frequency_analysis(&vec_u8);
		assert_eq!(
			result_u8,
			[(223, 3), (3, 2), (17, 2), (255, 1), (42, 1)].iter().cloned().collect());
	}

	#[test]
	fn frequency_analysis_u64() {
		let vec_u64: Vec<u64> = vec![
			12743, 684631, 547, 1253, 12743, 64374, 547, 2, 3562, 12743, 47652];
		let result_u64 = frequency_analysis(&vec_u64);
		assert_eq!(
			result_u64,
			[(684631, 1), (2, 1), (47652, 1), (64374, 1), (12743, 3), (547, 2),
			  (3562, 1), (1253, 1)].iter().cloned().collect());
	}

	#[test]
	fn print_frequency_analysis_result_test() {
		let vec: Vec<(u32, usize)> = vec![(684, 4), (2, 1), (242, 5), (2, 1), (123, 3)];
		let mut out = Vec::new();
		print_frequency_analysis_result(vec.iter().cloned().collect(), &mut out).unwrap();
		let mut expected = Vec::new();
		writeln!(expected, "5: 242\n4: 684\n3: 123\n1: 2").unwrap();
		assert_eq!(out, expected);
	}

	#[test]
	fn frequency_analysis_empty() {
		assert_eq!(
			frequency_analysis::<u128>(&Vec::new()),
			HashMap::new());
	}
}
