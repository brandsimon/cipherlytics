use std::collections::HashMap;
use std::ops::AddAssign;
use std::hash::Hash;

pub fn count_letters<T: Copy + AddAssign + Eq + Hash>(vec: &Vec<T>) -> HashMap<T, usize> {
	let mut map: HashMap<T, usize> = HashMap::new();
	for i in vec {
		*map.entry(*i).or_insert(0) += 1
	}
	return map;
}

#[cfg(test)]
mod tests {
	use crate::analytics::count_letters::count_letters;
	use std::collections::HashMap;

	#[test]
	fn test_count_letters() {
		let vec_u8: Vec<u8> = vec![223, 3, 17, 223, 255, 42, 3, 17, 223];
		let result_u8 = count_letters(&vec_u8);
		assert_eq!(
			result_u8,
			[(223, 3), (3, 2), (17, 2), (255, 1), (42, 1)].iter().cloned().collect());

		let vec_u64: Vec<u64> = vec![
			12743, 684631, 547, 1253, 12743, 64374, 547, 2, 3562, 12743, 47652];
		let result_u64 = count_letters(&vec_u64);
		assert_eq!(
			result_u64,
			[(684631, 1), (2, 1), (47652, 1), (64374, 1), (12743, 3), (547, 2),
			  (3562, 1), (1253, 1)].iter().cloned().collect());

		assert_eq!(
			count_letters::<u128>(&Vec::new()),
			HashMap::new());
	}
}
