mod operations;
mod hash;


fn main() {

	let mut input = vec!['a' as u8];

	let result = hash::sha512(&mut input);

	operations::print_hex(&result);

    // let args: Vec<String> = std::env::args().collect();
    // if args.len() < 2 {
    //     eprintln!("input a file name");
    //     std::process::exit(1);
    // }
    // let file_name:String = args[1].clone();
    // let unsafe_file = std::fs::read(&file_name);
    // match unsafe_file {
    //     Ok(mut safe_file) => {
			
    //     }
    //     Err(error) => eprintln!("error reading {}, error:{}", &file_name, &error),
    // }
}
