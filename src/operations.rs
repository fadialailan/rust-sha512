const MODULUS: usize = 1024 / 8;

const CONGRUENT_VALUE: usize = 896 / 8;
const PADDING_START: u8 = 0b10000000;

const H: [u64; 8] = [
    0x6a09e667f3bcc908,
    0xbb67ae8584caa73b,
    0x3c6ef372fe94f82b,
    0xa54ff53a5f1d36f1,
    0x510e527fade682d1,
    0x9b05688c2b3e6c1f,
    0x1f83d9abfb41bd6b,
	0x1f83d9abfb41bd6b,
];

pub fn padding(data: &mut Vec<u8>) -> () {
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

pub fn parsing(data: &Vec<u8>) -> Vec<u128> {
	const BLOCK_SIZE:usize = 128/8;
	let mut output:Vec::<u128> = Vec::new();
	for i in (0..data.len()).step_by(BLOCK_SIZE) {
		output.push(u128::from_be_bytes(data[i..i+BLOCK_SIZE].try_into().unwrap()));
	}

	return output;

}
