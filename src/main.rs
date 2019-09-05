extern crate hound;
use std::i16;
use std::env;

fn block_char(s:f32, inv:i8) -> String {
	let mut i;
	if s.is_infinite() || s < -36.0 {
		i = 0;
	} else if s < -24.0 {
		i = 1;
	} else if s < -18.0 {
		i = 2;
	} else if s < -12.0 {
		i = 3;
	} else if s < -9.0 {
		i = 4;
	} else if s < -6.0 {
		i = 5;
	} else if s < -3.0 {
		i = 6;
	} else if s < -1.5 {
		i = 7;
	} else {
		i = 8;
	}

	if i < 0 {
		i = 0
	}
	if inv == 1 {
		if i == 0 {
			return String::from(" ");
		}
		i = 8 - i
	}
	
	let mut bytes = String::from("\u{2581}").into_bytes();
	let base = bytes[2];
	bytes[2] = base + i as u8;
	let block = String::from_utf8(bytes).unwrap();
	if inv == 0 {
		block
	} else {
		format!("\x1b[7m{}\x1b[0m", block)
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let path = &args[1];
	let mut reader = hound::WavReader::open(path).unwrap();
	let mut max_samp = 0.0;
	let mut min_samp = 0.0;
	let mut i = 0;
	println!("{}", path);
	println!();
	reader.samples::<i16>().for_each(|x| {
		let sample = x.unwrap() as f32 / ( i16::MAX as f32);
		if sample > max_samp { max_samp = sample; }
		if sample < min_samp { min_samp = sample; }
		if i % 5 == 4 {
			let min_db = 20.0*(-min_samp).log10();
			let max_db = 20.0*max_samp.log10();
			print!("{}\x1b[D\x1b[B", block_char(min_db, 0));
			print!("{}\x1b[A",       block_char(max_db, 1));
			min_samp = 0.0;
			max_samp = 0.0;
		}
		i = i + 1
	});
	println!();
	println!();
	println!();
}
