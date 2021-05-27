use std::cmp::PartialOrd;
use std::env;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::AddAssign;
use std::ops::Shl;
use std::io;
mod analytics;
mod input;
mod types;
mod arguments;


fn main_type<
	T: Copy + AddAssign + Eq + Hash + Debug + Shl + From<u8> +
	   From<<T as Shl>::Output> + types::NumBytes + Display + PartialOrd
>(action: &arguments::Action) -> Result<(), io::Error> {
	let input = input::read_file(&action.filename)?;
	let conv_vec = input::convert_vec::<T>(&input)?;
	let vec = input::filter_input_vec(
		&conv_vec, action.keep_every, action.skip_first)?;
	let mut stdout = io::stdout();
	let mut out = io::Stdout::lock(&mut stdout);
	match action.method {
		arguments::AnalyzeMethod::None => (),
		arguments::AnalyzeMethod::MinMax => {
			let res = analytics::min_max(&vec)?;
			analytics::print_min_max_result(&res, &mut out)?;
		},
		arguments::AnalyzeMethod::FrequencyAnalysis => {
			let res = analytics::frequency_analysis(&vec);
			analytics::print_frequency_analysis_result(res, &mut out)?;
		},
		arguments::AnalyzeMethod::KasiskiExamination(l) => {
			let res = analytics::kasiski_examination(&vec, l);
			analytics::print_kasiski_examination_result(res, &mut out)?;
		},
	};
	return Ok(());
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
		arguments::help(&args[0]);
		return;
	}
	let result = match action.size {
		arguments::Sizes::U8 => main_type::<u8>(&action),
		arguments::Sizes::U16 => main_type::<u16>(&action),
		arguments::Sizes::U32 => main_type::<u32>(&action),
		arguments::Sizes::U64 => main_type::<u64>(&action),
		arguments::Sizes::U128 => main_type::<u128>(&action),
	};
	match result {
		Ok(()) => (),
		Err(l) => {
			println!("Error: {}", l);
		},
	}
}
