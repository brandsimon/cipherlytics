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
	pub method: AnalyzeMethod,
	pub size: Sizes,
	pub filename: String,
	pub help: bool,
}

const DEFAULT_KASISKI_LEN: usize = 5;

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

fn parse_usize(arg: Option<&String>, error: String) -> Result<usize, String> {
	match arg.map(|s| s.parse::<usize>()) {
		Some(Ok(l)) => {
			return Ok(l);
		},
		_ => {
			return Err(error);
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
		Some("--min-length") => {
			let m = parse_usize(
				args.get(pos + 1),
				"min-length is invalid".to_string())?;
			Ok((AnalyzeMethod::KasiskiExamination(m), 2))
		},
		_ => Ok((AnalyzeMethod::KasiskiExamination(DEFAULT_KASISKI_LEN), 0)),
	};
}

pub fn parse_args(args: &Vec<String>) -> Result<Action, String> {
	let mut method: AnalyzeMethod = AnalyzeMethod::None;
	let mut file: Option<String> = None;
	let mut help = false;
	let mut method_set_count = 0;
	let mut size: Sizes = Sizes::U8;

	let mut pos = 1; // Skip binary
	while pos < args.len() {
		let arg = &args[pos];
		match Some(&*arg.to_string()) {
			Some("--bytes") => {
				pos += 1;
				size = parse_sizes(args.get(pos))?;
			},
			Some("min_max") => {
				method = AnalyzeMethod::MinMax;
				method_set_count += 1;
			},
			Some("frequency_analysis") => {
				method = AnalyzeMethod::FrequencyAnalysis;
				method_set_count += 1;
			},
			Some("kasiski_examination") => {
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
			method: method,
			filename: f,
			size: size,
			help: help,
		}),
		_ => Err("No file specified".to_string()),
	}
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
			arguments::parse_usize(Some(&"0".to_string()), e()).unwrap(),
			0);
		assert_eq!(
			arguments::parse_usize(Some(&"187".to_string()), e()).unwrap(),
			187);
		assert_eq!(
			arguments::parse_usize(Some(&"-1".to_string()), e()),
			Err(e()));
		assert_eq!(
			arguments::parse_usize(Some(&"tes".to_string()), e()),
			Err(e()));
		assert_eq!(
			arguments::parse_usize(None, e()),
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
				method: arguments::AnalyzeMethod::MinMax,
				size: arguments::Sizes::U8,
				filename: "file".to_string(),
				help: false }));
		assert_eq!(
			arguments::parse_args(&vec_str_conv(
				vec!["", "--bytes", "2", "frequency_analysis", "da"])),
			Ok(arguments::Action {
				method: arguments::AnalyzeMethod::FrequencyAnalysis,
				size: arguments::Sizes::U16,
				filename: "da".to_string(),
				help: false }));
		assert_eq!(
			arguments::parse_args(&vec_str_conv(
				vec!["", "--bytes", "4", "kasiski_examination",
				     "--min-length", "8", "input"])),
			Ok(arguments::Action {
				method: arguments::AnalyzeMethod::KasiskiExamination(8),
				size: arguments::Sizes::U32,
				filename: "input".to_string(),
				help: false }));
		assert_eq!(
			arguments::parse_args(&vec_str_conv(
				vec!["", "--bytes", "8", "kasiski_examination", "in"])),
			Ok(arguments::Action {
				method: arguments::AnalyzeMethod::KasiskiExamination(5),
				size: arguments::Sizes::U64,
				filename: "in".to_string(),
				help: false }));
		assert_eq!(
			arguments::parse_args(&vec_str_conv(
				vec!["", "--bytes", "16", "kasiski_examination", "--help"])),
			Ok(arguments::Action {
				method: arguments::AnalyzeMethod::KasiskiExamination(5),
				size: arguments::Sizes::U128,
				filename: "".to_string(),
				help: true }));
		assert_eq!(
			arguments::parse_args(&vec_str_conv(
				vec!["", "-h"])),
			Ok(arguments::Action {
				method: arguments::AnalyzeMethod::None,
				size: arguments::Sizes::U8,
				filename: "".to_string(),
				help: true }));
	}
}
