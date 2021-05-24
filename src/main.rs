use std::env;
mod analytics;
mod input;
mod types;
mod arguments;

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
	let input = input::read_file(&action.filename).unwrap();
	// TODO: for u8 ... u128
	let vec = input::convert_vec::<u8>(&input).unwrap();

	match action.method {
		arguments::AnalyzeMethod::None => (),
		arguments::AnalyzeMethod::MinMax => {
			let res = analytics::min_max(&vec).unwrap();
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
