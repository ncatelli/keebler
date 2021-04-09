use keebler::*;
use parcel::prelude::v1::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();

    match args_len {
        2 => read_file(&args[1]).expect("Unable to open file"),
        _ => {
            println!("Usage: readelf [file]");
            process::exit(64);
        }
    }
}

fn read_file(filename: &str) -> Result<(), String> {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = Vec::new();
    match f.read_to_end(&mut contents) {
        Ok(_) => {
            let ident = EiIdentParser.parse(&contents).unwrap().unwrap();
            println!("{:#?}", ident);
            Ok(())
        }
        Err(error) => Err(format!("error: {}", error)),
    }
}
