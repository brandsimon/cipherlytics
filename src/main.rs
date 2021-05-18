mod input;
mod types;

fn main() {
	let vec = input::read_file("src/main.rs_printable").unwrap();
	let vec2 = input::convert_vec::<u16>(&vec).unwrap();
	let (_head, body, _tail) = unsafe { vec2.align_to::<u8>() };
	println!("{}", String::from_utf8(vec).unwrap());
	println!("{}", String::from_utf8_lossy(body));
}
