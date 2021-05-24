use std::cmp::PartialOrd;
use std::env;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::AddAssign;
use std::ops::Shl;
mod analytics;
mod input;
mod types;
mod arguments;


fn main_type<
	T: Copy + AddAssign + Eq + Hash + Debug + Shl + From<u8> +
	   From<<T as Shl>::Output> + types::NumBytes + Display + PartialOrd
>(input: &Vec<u8>, action: &arguments::Action) {
	let vec = input::convert_vec::<T>(&input).unwrap(); // TODO unwrap
	match action.method {
		arguments::AnalyzeMethod::None => (),
		arguments::AnalyzeMethod::MinMax => {
			let res = analytics::min_max(&vec).unwrap(); // TODO unwrap
			println!("Min: {}, Max: {}", res.0, res.1);
		},
		arguments::AnalyzeMethod::FrequencyAnalysis => {
			let res = analytics::frequency_analysis(&vec);
			for (i, j) in res {
				println!("{}: {}", j, i);
			}
		},
		arguments::AnalyzeMethod::KasiskiExamination(l) => {
			let res = analytics::kasiski_examination(&vec, l);
			println!("Res size: {}", res.len());
			for (i, j) in res {
				println!("{:?}: {:?}", j, i);
			}
		},
	};
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let action = match arguments::parse_args(&args) {
		Ok(l) => l,
		Err(s) => {
			println!("{}", s);
			return;
		},
	};
	if action.help {
		println!("Help message"); // TODO
		return;
	}
	let input = input::read_file(&action.filename).unwrap(); // TODO unwrap
	match action.size {
		arguments::Sizes::U8 => main_type::<u8>(&input, &action),
		arguments::Sizes::U16 => main_type::<u16>(&input, &action),
		arguments::Sizes::U32 => main_type::<u32>(&input, &action),
		arguments::Sizes::U64 => main_type::<u64>(&input, &action),
		arguments::Sizes::U128 => main_type::<u128>(&input, &action),
	}
}
