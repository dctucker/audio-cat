extern crate hound;
use std::i16;
use std::env;
use std::cmp::{min,max};

fn block_char(i:i8, inv:i8) -> String {
	if i <= 0 {
		if inv == 1 {
			return String::from("\u{2588}");
		} else {
			return String::from(" ");
		}
	}

	let mut bytes = String::from("\u{2580}").into_bytes();
	let base = bytes[2];
	bytes[2] = base + i as u8;
	let block = String::from_utf8(bytes).unwrap();
	if inv == 0 {
		block
	} else {
		format!("\x1b[7m{}\x1b[27m", block)
	}
}

fn main() {
	const HEIGHT : i8 = 6;
	let mut grid = vec![ vec![]; HEIGHT as usize ];

	let args: Vec<String> = env::args().collect();
	let path = &args[1];
	let mut reader = hound::WavReader::open(path).unwrap();
	let mut max_samp = 0.0;
	let mut min_samp = 0.0;
	let mut i = 0;

	println!("{}", path);
	println!();
	for n in 0..HEIGHT/2 as i8 {
		grid[n as usize].push(String::from("\x1b[36m"));
		grid[(n+(HEIGHT/2)) as usize].push(String::from("\x1b[36m"));
	}
	reader.samples::<i16>().for_each(|x| {
		let sample = x.unwrap() as f32 / ( i16::MAX as f32);
		if sample > max_samp { max_samp = sample; }
		if sample < min_samp { min_samp = sample; }
		if i % 5 == 4 {
			let min_i = -(min_samp * 4.0 * (HEIGHT as f32)) as i8;
			let max_i =  (max_samp * 4.0 * (HEIGHT as f32)) as i8;

			for n in 0..HEIGHT/2 as i8 {
				let sub = ((HEIGHT/2-1)-n) * 8 as i8;
				grid[n as usize].push(block_char(min(8,max(0,min_i-sub)), 0));
				let sub = n * 8 as i8;
				grid[(n+(HEIGHT/2)) as usize].push(block_char(8 - min(8,max(0,max_i-sub)), 1));
			}
			min_samp = 0.0;
			max_samp = 0.0;
		}
		i = i + 1
	});

	for line in &grid {
		for ch in line {
			print!("{}", ch);
		}
		println!();
	}

}
