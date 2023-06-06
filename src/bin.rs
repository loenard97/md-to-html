use std::env;
use std::fs;
use std::io::Write;
use std::process;

use md_to_html::Converter;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Error: need input and output file path");
        process::exit(1);
    }
    let file_md = &args[1];
    let file_html = &args[2];

    let file = fs::File::open(file_md).unwrap();
    let md_file = Converter::from_file(file);

    let mut output = fs::File::create(file_html).unwrap();
    _ = write!(output, "{}", md_file.to_html());
}
