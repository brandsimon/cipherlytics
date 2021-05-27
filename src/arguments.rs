// TODO: use argument parsing library

#[derive(PartialEq, Debug)]
pub enum AnalyzeMethod {
	None,
	MinMax,
	FrequencyAnalysis,
	KasiskiExamination(usize),
}

#[derive(PartialEq, Debug)]
pub enum Sizes {
	U8,
	U16,
	U32,
	U64,
	U128,
}

#[derive(PartialEq, Debug)]
pub struct Action {
	pub keep_every: usize,
	pub skip_first: usize,
	pub method: AnalyzeMethod,
	pub size: Sizes,
	pub filename: String,
	pub help: bool,
}

const DEFAULT_KASISKI_LEN: usize = 5;
const DEFAULT_SIZE: &str = "1";
const DEFAULT_KEEP_EVERY: &str = "1";
const DEFAULT_SKIP_FIRST: &str = "0";
const STR_MIN_MAX: &str = "min_max";
const STR_FREQUENCY_ANALYSIS: &str = "frequency_analysis";
const STR_KASISKI_EXAMINATION: &str = "kasiski_examination";
const STR_MIN_LENGTH: &str = "--min-length";
const STR_BYTES: &str = "--bytes";
const STR_KEEP_EVERY: &str = "--keep-every";
const STR_SKIP_FIRST: &str = "--skip-first";

fn parse_sizes(arg: Option<&String>) -> Result<Sizes, String> {
	let error = Err("Bytes parameter is invalid".to_string());
	let s = match arg {
		Some(t) => t,
		_ => {
			return error;
		}
	};
	return match &*s.to_string() {
		"1" => Ok(Sizes::U8),
		"2" => Ok(Sizes::U16),
		"4" => Ok(Sizes::U32),
		"8" => Ok(Sizes::U64),
		"16" => Ok(Sizes::U128),
		_ => error,
	};
}

fn parse_usize(arg: Option<&String>, error: &String) -> Result<usize, String> {
	match arg.map(|s| s.parse::<usize>()) {
		Some(Ok(l)) => {
			return Ok(l);
		},
		_ => {
			return Err(error.clone());
		},
	};
}

fn parse_kasiski_examination_params(args: &Vec<String>, pos: usize) ->
		Result<(AnalyzeMethod, usize), String> {
	if pos >= args.len() {
		return Ok((AnalyzeMethod::KasiskiExamination(DEFAULT_KASISKI_LEN), 0));
	}
	let arg = &args[pos];
	return match Some(&*arg.to_string()) {
		Some(STR_MIN_LENGTH) => {
			let m = parse_usize(
				args.get(pos + 1),
				"min-length is invalid".to_string())?;
			Ok((AnalyzeMethod::KasiskiExamination(m), 2))
		},
		_ => Ok((AnalyzeMethod::KasiskiExamination(DEFAULT_KASISKI_LEN), 0)),
	};
}

fn parse_optionals(args: &Vec<String>) -> Result<(Sizes, usize, usize, usize), String> {
	let mut size: Sizes = parse_sizes(Some(&DEFAULT_SIZE.to_string()))?;
	let keep_every_error = format!("{} is invalid", STR_KEEP_EVERY);
	let mut keep_every = parse_usize(Some(&DEFAULT_KEEP_EVERY.to_string()), &keep_every_error)?;
	let skip_first_error = format!("{} is invalid", STR_SKIP_FIRST);
	let mut skip_first = parse_usize(Some(&DEFAULT_SKIP_FIRST.to_string()), &skip_first_error)?;

	let mut pos = 1; // Skip binary
	while pos < args.len() {
		let arg = &args[pos];
		match Some(&*arg.to_string()) {
			Some(STR_BYTES) => {
				size = parse_sizes(args.get(pos + 1))?;
				pos += 1;
			},
			Some(STR_KEEP_EVERY) => {
				keep_every = parse_usize(args.get(pos + 1), &keep_every_error)?;
				pos += 1;
			},
			Some(STR_SKIP_FIRST) => {
				skip_first = parse_usize(args.get(pos + 1), &skip_first_error)?;
				pos += 1;
			},
			_ => {
				break;
			},
		}
		pos += 1;
	}
	return Ok((size, skip_first, keep_every, pos));
}

pub fn parse_args(args: &Vec<String>) -> Result<Action, String> {
	let mut method: AnalyzeMethod = AnalyzeMethod::None;
	let mut file: Option<String> = None;
	let mut help = false;
	let mut method_set_count = 0;

	let (size, skip_first, keep_every, mut pos) = parse_optionals(&args)?;
	while pos < args.len() {
		let arg = &args[pos];
		match Some(&*arg.to_string()) {
			Some(STR_MIN_MAX) => {
				method = AnalyzeMethod::MinMax;
				method_set_count += 1;
			},
			Some(STR_FREQUENCY_ANALYSIS) => {
				method = AnalyzeMethod::FrequencyAnalysis;
				method_set_count += 1;
			},
			Some(STR_KASISKI_EXAMINATION) => {
				let (m, a) = parse_kasiski_examination_params(args, pos + 1)?;
				method = m;
				pos += a;
				method_set_count += 1;
			},
			Some("-h") => {
				help = true;
			},
			Some("--help") => {
				help = true;
			},
			_ => {
				file = Some(arg.to_string());
				if pos != args.len() - 1 {
					return Err(format!(
						"Unkown parameter {}, file must be provided last",
						arg));
				}
			},
		}
		pos += 1;
	}
	if method_set_count != 1 && !help {
		return Err("Method needs to be provided once".to_string());
	}
	if help {
		file = Some("".to_string());
	}
	return match file {
		Some(f) => Ok(Action {
			skip_first: skip_first,
			keep_every: keep_every,
			method: method,
			filename: f,
			size: size,
			help: help,
		}),
		_ => Err("No file specified".to_string()),
	}
}

pub fn help(exe: &String) {
	println!(
		"Analyze ciphertext with classical cryptanalysis\n\
		\n\
		Usage: {exe} [--bytes BYTES] METHOD [METHOD PARAMETERS] FILE\n\
		\t-h|--help: Print this help message\n\
		\t{bytes}:   How many bytes to group together.\n\
		\t           1, 2, 4, 8, 16 (Default: {size}\n\
		\n\
		Methods:\n\
		\t{min_max}                 Show range of bytes\n\
		\t{frequency_analysis}      Count occurence of bytes\n\
		\t{kasiski_examination}     Show duplicate words\n\
		\t\t{min_length}    Minimum word length, Default: {kasiski_len}",
		exe=exe,
		kasiski_len=DEFAULT_KASISKI_LEN,
		frequency_analysis=STR_FREQUENCY_ANALYSIS,
		min_max=STR_MIN_MAX,
		min_length=STR_MIN_LENGTH,
		kasiski_examination=STR_KASISKI_EXAMINATION,
		bytes=STR_BYTES,
		size=DEFAULT_SIZE);
}

#[cfg(test)]
mod tests {
	use crate::arguments;

	#[test]
	fn parse_sizes() {
		assert_eq!(
			arguments::parse_sizes(Some(&"1".to_string())).unwrap(),
			arguments::Sizes::U8);
		assert_eq!(
			arguments::parse_sizes(Some(&"2".to_string())).unwrap(),
			arguments::Sizes::U16);
		assert_eq!(
			arguments::parse_sizes(Some(&"4".to_string())).unwrap(),
			arguments::Sizes::U32);
		assert_eq!(
			arguments::parse_sizes(Some(&"8".to_string())).unwrap(),
			arguments::Sizes::U64);
		assert_eq!(
			arguments::parse_sizes(Some(&"16".to_string())).unwrap(),
			arguments::Sizes::U128);
		assert_eq!(
			arguments::parse_sizes(None),
			Err("Bytes parameter is invalid".to_string()));
		assert_eq!(
			arguments::parse_sizes(Some(&"som".to_string())),
			Err("Bytes parameter is invalid".to_string()));
	}

	#[test]
	fn parse_usize() {
		fn e() -> String {
			return "err".to_string();
		}
		assert_eq!(
			arguments::parse_usize(Some(&"0".to_string()), &e()).unwrap(),
			0);
		assert_eq!(
			arguments::parse_usize(Some(&"187".to_string()), &e()).unwrap(),
			187);
		assert_eq!(
			arguments::parse_usize(Some(&"-1".to_string()), &e()),
			Err(e()));
		assert_eq!(
			arguments::parse_usize(Some(&"tes".to_string()), &e()),
			Err(e()));
		assert_eq!(
			arguments::parse_usize(None, &e()),
			Err(e()));
	}

	fn vec_str_conv(input: Vec<&str>) -> Vec<String> {
		let mut result = Vec::with_capacity(input.len());
		for i in input {
			result.push(i.to_string());
		}
		return result;
	}

	#[test]
	fn parse_kasiski_examination_params() {
		let v = vec_str_conv(vec!["a", "--min-length", "72"]);
		assert_eq!(
			arguments::parse_kasiski_examination_params(&v, 1),
			Ok((arguments::AnalyzeMethod::KasiskiExamination(72), 2)));
		assert_eq!(
			arguments::parse_kasiski_examination_params(&v, 2),
			Ok((arguments::AnalyzeMethod::KasiskiExamination(5), 0)));
		assert_eq!(
			arguments::parse_kasiski_examination_params(&v, 4),
			Ok((arguments::AnalyzeMethod::KasiskiExamination(5), 0)));
		let err_v = vec_str_conv(vec!["--min-length", "b"]);
		assert_eq!(
			arguments::parse_kasiski_examination_params(&err_v, 0),
			Err("min-length is invalid".to_string()));
	}

	#[test]
	fn parse_optionals_all() {
		assert_eq!(
			arguments::parse_optionals(&vec_str_conv(vec![
				"", "--bytes", "8", "--skip-first", "3", "--keep-every", "2"])),
			Ok((arguments::Sizes::U64, 3, 2, 7)));
	}

	#[test]
	fn parse_optionals_partial() {
		assert_eq!(
			arguments::parse_optionals(&vec_str_conv(vec![
				"", "--skip-first", "3", "method", "--keep-every", "2"])),
			Ok((arguments::Sizes::U8, 3, 1, 3)));
	}

	#[test]
	fn parse_optionals_defaults() {
		assert_eq!(
			arguments::parse_optionals(&vec_str_conv(vec![
				"", "method", "--skip-first", "3", "--keep-every", "2"])),
			Ok((arguments::Sizes::U8, 0, 1, 1)));
	}

	#[test]
	fn parse_args() {
		assert_eq!(
			arguments::parse_args(&vec_str_conv(vec![])),
			Err("Method needs to be provided once".to_string()));
		assert_eq!(
			arguments::parse_args(&vec_str_conv(vec!["", "--bytes", "4", "a"])),
			Err("Method needs to be provided once".to_string()));
		assert_eq!(
			arguments::parse_args(&vec_str_conv(
				vec!["", "--bytes", "2", "min_max", "f", "g"])),
			Err("Unkown parameter f, file must be provided last".to_string()));
		assert_eq!(
			arguments::parse_args(&vec_str_conv(
				vec!["", "--bytes", "1", "min_max", "file"])),
			Ok(arguments::Action {
				skip_first: 0,
				keep_every: 1,
				method: arguments::AnalyzeMethod::MinMax,
				size: arguments::Sizes::U8,
				filename: "file".to_string(),
				help: false }));
		assert_eq!(
			arguments::parse_args(&vec_str_conv(
				vec!["", "--bytes", "2", "frequency_analysis", "da"])),
			Ok(arguments::Action {
				skip_first: 0,
				keep_every: 1,
				method: arguments::AnalyzeMethod::FrequencyAnalysis,
				size: arguments::Sizes::U16,
				filename: "da".to_string(),
				help: false }));
		assert_eq!(
			arguments::parse_args(&vec_str_conv(
				vec!["", "--bytes", "4", "kasiski_examination",
				     "--min-length", "8", "input"])),
			Ok(arguments::Action {
				skip_first: 0,
				keep_every: 1,
				method: arguments::AnalyzeMethod::KasiskiExamination(8),
				size: arguments::Sizes::U32,
				filename: "input".to_string(),
				help: false }));
		assert_eq!(
			arguments::parse_args(&vec_str_conv(
				vec!["", "--bytes", "8", "--keep-every", "4",
				     "--skip-first", "2", "kasiski_examination", "in"])),
			Ok(arguments::Action {
				skip_first: 2,
				keep_every: 4,
				method: arguments::AnalyzeMethod::KasiskiExamination(5),
				size: arguments::Sizes::U64,
				filename: "in".to_string(),
				help: false }));
		assert_eq!(
			arguments::parse_args(&vec_str_conv(
				vec!["", "--bytes", "16", "kasiski_examination", "--help"])),
			Ok(arguments::Action {
				skip_first: 0,
				keep_every: 1,
				method: arguments::AnalyzeMethod::KasiskiExamination(5),
				size: arguments::Sizes::U128,
				filename: "".to_string(),
				help: true }));
		assert_eq!(
			arguments::parse_args(&vec_str_conv(
				vec!["", "-h"])),
			Ok(arguments::Action {
				skip_first: 0,
				keep_every: 1,
				method: arguments::AnalyzeMethod::None,
				size: arguments::Sizes::U8,
				filename: "".to_string(),
				help: true }));
	}
}
