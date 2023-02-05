
pub fn padding(data: &mut Vec<u8>) -> () {

	const MODULUS: usize = 1024 / 8;

	const CONGRUENT_VALUE: usize = 896 / 8;
	const PADDING_START: u8 = 0b10000000;

	let original_length = data.len();
	let mode_size = original_length % MODULUS;

	if mode_size < CONGRUENT_VALUE {
		data.resize(original_length + CONGRUENT_VALUE - mode_size, 0);
	} else {
		data.resize(original_length + MODULUS - (mode_size - CONGRUENT_VALUE), 0);
    }
	data[original_length] = PADDING_START;
	data.extend_from_slice(&u128::to_be_bytes(original_length as u128 * 8));
}

pub fn parsing(data: &Vec<u8>) -> Vec<u64> {
	const BLOCK_SIZE:usize = 64/8;
	let mut output:Vec::<u64> = Vec::new();
	for i in (0..data.len()).step_by(BLOCK_SIZE) {
		output.push(u64::from_be_bytes(data[i..i+BLOCK_SIZE].try_into().unwrap()));
	}

	return output;

}

pub fn print_hex(data:&[u64]) {
	for i in 0..data.len() {
		print!("{:016x}", data[i]);
	}
	println!("");
}

pub mod sha {
	pub fn ch(x:u64, y:u64, z:u64) -> u64 {
		return (x & y) ^ ( (!x) & z);
	}

	pub fn maj(x:u64, y:u64, z:u64) -> u64 {
		return (x | y) ^ (x | z) ^ (y | z);
	}

	pub fn bsigma0(x:u64) -> u64 {
		return x.rotate_right(28) ^ x.rotate_right(34) ^ x.rotate_right(39);
	}

	pub fn bsigma1(x:u64) -> u64 {
		return x.rotate_right(14) ^ x.rotate_right(18) ^ x.rotate_right(41);
	}

	pub fn sigma0(x:u64) -> u64 {
		return x.rotate_right(1) ^ x.rotate_right(8) ^ (x >> 7);
	}

	pub fn sigma1(x:u64) -> u64 {
		return x.rotate_right(19) ^ x.rotate_right(61) ^ (x >> 6);
	}


}
