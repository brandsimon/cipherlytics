use std::collections::HashMap;
use std::ops::AddAssign;
use std::hash::Hash;
use std::collections::HashSet;
use std::option::Option;
use std::fmt::Display;
use std::io::Write;
use std::io;

fn find_common_length<T: Eq>(vec: &Vec<T>, start1: usize, start2: usize) -> usize {
	let mut common_length: usize = 0;
	let mut pos1 = start1;
	let mut pos2 = start2;
	let vec_len = vec.len();
	while pos1 < vec_len && pos2 < vec_len {
		if vec[pos1] != vec[pos2] {
			break;
		}
		common_length += 1;
		pos1 += 1;
		pos2 += 1;
	}
	return common_length;
}

fn param_to_word<T: Copy>(vec: &Vec<T>, start: usize, length: usize) -> Vec<T> {
	let mut result: Vec<T> = Vec::with_capacity(length);
	for j in start..start + length {
		result.push(vec[j]);
	}
	return result;
}


// Iterator to iterate over Pair<T, T> from Vec<T>
// Process every element value pair (e.g. (2, 7), (7, 2)) only once
// do not iterate over pair where pair.0 == pair.1
struct DeDupPairIter<'a, T: Hash> {
	iter1: std::slice::Iter<'a, T>,
	iter2: std::slice::Iter<'a, T>,
	vec: &'a Vec<T>,
	last1: Option<&'a T>,
	processed: HashSet<(&'a T, &'a T)>,
}

impl<'a, T: Hash> DeDupPairIter<'a, T> {
	pub fn new(vec: &'a Vec<T>) -> DeDupPairIter<'a, T>{
		let mut iter1 = vec.into_iter();
		let last1 = iter1.next();
		return DeDupPairIter {
			iter1: iter1,
			iter2: vec.into_iter(),
			vec: vec,
			last1: last1,
			processed: HashSet::new(),
		};
	}
}

impl<'a, T: 'a + Eq + Hash> Iterator for DeDupPairIter<'a, T> {
	type Item = (&'a T, &'a T);

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			let v1 = match self.last1 {
				None => {
					return None;
				},
				Some(val1) => val1,
			};
			match self.iter2.next() {
				None => {
					self.last1 = self.iter1.next();
					self.iter2 = self.vec.into_iter();
				},
				Some(v2) => {
					let process =
						v1 != v2 &&
						!self.processed.contains(&(v1, v2)) &&
						!self.processed.contains(&(v2, v1));
					if process {
						self.processed.insert((v1, v2));
						return Some((v1, v2));
					}
				},
			};
		};
	}
}


// Return start of duplicate words with a min length
// Shorter duplicates are returned since longer matches can be by accident in the text
pub fn kasiski_examination<
	T: Copy + AddAssign + Eq + Hash
>(vec: &Vec<T>, min_length: usize) -> HashMap<Vec<T>, HashSet<usize>> {
	// If this is too ram heavy, it can be rewritten by comparing the text with
	// itself while shifting one text one char to the right
	let mut words_start: HashMap<Vec<T>, Vec<usize>> = HashMap::new();
	let mut result: HashMap<Vec<T>, HashSet<usize>> = HashMap::new();
	if vec.len() < min_length + 1 {
		return result;
	}
	for i in 0..vec.len() - min_length + 1 {
		let word = param_to_word(vec, i, min_length);
		words_start.entry(word).or_insert(Vec::new()).push(i);
	}
	for (_, word_starts) in &words_start {
		// Only iterate once over every tuple of starts
		for starts in DeDupPairIter::new(word_starts) {
			let check1 = starts.0 + min_length;
			let check2 = starts.1 + min_length;
			let l = find_common_length(vec, check1, check2);
			let total_length = min_length + l;
			let l_word = param_to_word(vec, *starts.0, total_length);
			let set = result.entry(l_word).or_insert(HashSet::new());
			set.insert(*starts.0);
			set.insert(*starts.1);
		}
	}
	return result;
}

pub fn print_kasiski_examination_result<
	T: Display, W: Write
>(map: HashMap<Vec<T>, HashSet<usize>>, out: &mut W) -> Result<(), io::Error>{
	writeln!(out, "Words: {}", map.len())?;
	let mut vec: Vec<(&Vec<T>, &HashSet<usize>)> = map.iter().collect();
	vec.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
	for (i, j) in vec {
		// Write Set
		write!(out, "{{")?;
		let mut starts: Vec<&usize> = j.iter().collect();
		starts.sort_by(|a, b| a.cmp(b));
		let mut f1 = false;
		for k in starts {
			if f1 {
				write!(out, ", {}", k)?;
			} else {
				write!(out, "{}", k)?;
				f1 = true;
			}
		}
		write!(out, "}}: [")?;
		// Write Vec
		let mut f2 = false;
		for k in i {
			if f2 {
				write!(out, ", {}", k)?;
			} else {
				write!(out, "{}", k)?;
				f2 = true;
			}
		}
		writeln!(out, "]")?;
	}
	return Ok(());
}

#[cfg(test)]
mod tests {
	use super::kasiski_examination;
	use super::print_kasiski_examination_result;
	use super::param_to_word;
	use super::find_common_length;
	use super::DeDupPairIter;
	use std::collections::HashMap;
	use std::collections::HashSet;
	use std::io::Write;

	macro_rules! set {
		($($x:tt)*) => {
			{
				let tmp_vec = vec![$($x)*];
				tmp_vec.iter().cloned().collect()
			}
		}
	}

	#[test]
	fn param_to_word_u8() {
		let vec_u8: Vec<u8> = vec![223, 3, 17, 223, 255, 42, 3, 17, 223];
		assert_eq!(param_to_word(&vec_u8, 2, 3), vec![17, 223, 255]);
	}

	#[test]
	fn param_to_word_u32() {
		let vec_u32: Vec<u32> = vec![50, 12, 4, 136];
		assert_eq!(param_to_word(&vec_u32, 2, 2), vec![4, 136]);
	}

	#[test]
	fn find_common_length_u8() {
		let vec_u8: Vec<u8> = vec![223, 3, 17, 223, 223, 3, 17, 223];
		assert_eq!(find_common_length(&vec_u8, 0, 4), 4);
		assert_eq!(find_common_length(&vec_u8, 1, 5), 3);
		assert_eq!(find_common_length(&vec_u8, 2, 6), 2);
		assert_eq!(find_common_length(&vec_u8, 0, 3), 1);
		assert_eq!(find_common_length(&vec_u8, 0, 2), 0);
	}

	#[test]
	fn find_common_length_u64() {
		let vec_u64: Vec<u64> = vec![685654, 54433, 76856, 765835, 24523];
		assert_eq!(find_common_length(&vec_u64, 1, 1), 4);
	}

	#[test]
	fn dedup_pair_iter() {
		let vec: Vec<usize> = vec![8, 2, 7, 2, 3, 7];
		let mut result = Vec::new();
		for i in DeDupPairIter::new(&vec) {
			result.push((*i.0, *i.1));
		}
		let exp_res: Vec<(usize, usize)> = vec![
			(8, 2), (8, 7), (8, 3), (2, 7), (2, 3), (7, 3)];
		assert_eq!(result, exp_res);
	}

	#[test]
	fn kasiski_examination_u8() {
		let vec_u8: Vec<u8> = vec![223, 3, 17, 223, 255, 223, 255, 42, 3, 17, 223];
		assert_eq!(kasiski_examination(&vec_u8, 4), HashMap::new());
		assert_eq!(
			kasiski_examination(&vec_u8, 3),
			[(vec![3, 17, 223], set![1, 8])].iter().cloned().collect());
		assert_eq!(
			kasiski_examination(&vec_u8, 2),
			[(vec![3, 17, 223], set![1, 8]),
			 (vec![17, 223], set![2, 9]),
			 (vec![223, 255], set![3, 5]),
			].iter().cloned().collect());
	}

	#[test]
	fn kasiski_examination_u16_multi_occurence() {
		let vec: Vec<u16> = vec![
			57232, 53723, 5846, 5274, 23524, 54824, 45754, 3563,
			57457, 5645, 46745, 6565, 34534, 7856, 57456, 56751,
			5846, 5274, 23524, 54824, 45754, 3563, // w1
			46745, 6565, 34534, 7856, 57456, 56751, // w2
			2,
			5846, 5274, 23524, 54824, // w3
		];
		let mut result: HashMap<Vec<u16>, HashSet<usize>> = HashMap::new();
		assert_eq!(kasiski_examination(&vec, 8), result);
		result.insert(vec![5846, 5274, 23524, 54824, 45754, 3563], set![2, 16]);
		result.insert(vec![46745, 6565, 34534, 7856, 57456, 56751], set![10, 22]);
		assert_eq!(kasiski_examination(&vec, 6), result);
		result.insert(vec![5274, 23524, 54824, 45754, 3563], set![3, 17]);
		result.insert(vec![6565, 34534, 7856, 57456, 56751], set![11, 23]);
		assert_eq!(kasiski_examination(&vec, 5), result);
		result.insert(vec![23524, 54824, 45754, 3563], set![4, 18]);
		result.insert(vec![34534, 7856, 57456, 56751], set![12, 24]);
		result.insert(vec![5846, 5274, 23524, 54824], set![2, 16, 29]);
		assert_eq!(kasiski_examination(&vec, 4), result);
	}

	#[test]
	fn kasiski_examination_empty() {
		let vec: Vec<u128> = vec![];
		assert_eq!(kasiski_examination(&vec, 8), HashMap::new());
	}

	#[test]
	fn print_kasiski_examination_result_empty() {
		let mut out = Vec::new();
		let map: HashMap<Vec<u16>, HashSet<usize>> = HashMap::new();
		print_kasiski_examination_result(map, &mut out).unwrap();
		let mut expected = Vec::new();
		writeln!(expected, "Words: 0").unwrap();
		assert_eq!(out, expected);
	}

	#[test]
	fn print_kasiski_examination_result_words() {
		let mut out = Vec::new();
		let map: HashMap<Vec<u16>, HashSet<usize>> = [
			(vec![17, 223], set![2, 6, 9]),
			(vec![3, 17, 223, 4, 2], set![1, 8]),
			(vec![223, 255, 4], set![1, 2, 3, 5])].iter().cloned().collect();
		print_kasiski_examination_result(map, &mut out).unwrap();
		let mut expected = Vec::new();
		writeln!(expected, "Words: 3").unwrap();
		writeln!(expected, "{{1, 8}}: [3, 17, 223, 4, 2]").unwrap();
		writeln!(expected, "{{1, 2, 3, 5}}: [223, 255, 4]").unwrap();
		writeln!(expected, "{{2, 6, 9}}: [17, 223]").unwrap();
		println!("{:?}", std::str::from_utf8(&out));
		println!("{:?}", std::str::from_utf8(&expected));
		assert_eq!(out, expected);
	}
}
