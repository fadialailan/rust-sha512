mod operations;
mod hash;
mod tests;

fn main() {

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: {} [path to file to calculate checksum for]", args[0]);
        std::process::exit(1);
    }
    let file_name:String = args[1].clone();
    let unsafe_file = std::fs::read(&file_name);
    match unsafe_file {
        Ok(mut safe_file) => {
			let result = hash::sha512(&mut safe_file);
			operations::print_hex(&result);
        }
        Err(error) => eprintln!("error reading {}, {}", &file_name, &error),
    }
}
