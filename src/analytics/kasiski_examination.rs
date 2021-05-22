use std::collections::HashMap;
use std::ops::AddAssign;
use std::hash::Hash;
use std::collections::HashSet;


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
		for start1 in word_starts {
			for start2 in word_starts {
				if start1 != start2 {
					let check1 = start1 + min_length;
					let check2 = start2 + min_length;
					let l = find_common_length(vec, check1, check2);
					let total_length = min_length + l;
					let l_word = param_to_word(vec, *start1, total_length);
					result.entry(l_word).or_insert(HashSet::new()).insert(*start1);
				}
			}
		}
	}
	return result;
}

#[cfg(test)]
mod tests {
	use super::kasiski_examination;
	use super::param_to_word;
	use super::find_common_length;
	use std::collections::HashMap;
	use std::collections::HashSet;

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
}
