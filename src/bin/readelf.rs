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
        Ok(_) => parse_and_print_formatted_header(&contents),
        Err(error) => Err(format!("error: {}", error)),
    }
}

fn parse_and_print_formatted_header(input: &[u8]) -> Result<(), String> {
    let ident = EiIdentParser.parse(input)?.unwrap();
    match (ident.ei_class, ident.ei_data) {
        (EiClass::ThirtyTwoBit, EiData::Little) => {
            let eh = ElfHeaderParser::<u64, LittleEndianDataEncoding>::new()
                .parse(&input)?
                .unwrap();

            print_formatted_header(ident, eh.file_header);
        }
        (EiClass::ThirtyTwoBit, EiData::Big) => {
            let eh = ElfHeaderParser::<u64, LittleEndianDataEncoding>::new()
                .parse(&input)?
                .unwrap();
            print_formatted_header(ident, eh.file_header);
        }
        (EiClass::SixtyFourBit, EiData::Little) => {
            let eh = FileHeaderParser::<u64, LittleEndianDataEncoding>::new()
                .parse(&input)?
                .unwrap();
            print_formatted_header(ident, eh);
        }
        (EiClass::SixtyFourBit, EiData::Big) => {
            let eh = ElfHeaderParser::<u64, LittleEndianDataEncoding>::new()
                .parse(&input)?
                .unwrap();
            print_formatted_header(ident, eh.file_header);
        }
    };

    Ok(())
}

fn print_formatted_header<A: std::fmt::LowerHex + std::fmt::Display>(
    ident: EiIdent,
    header: FileHeader<A>,
) {
    println!(
        "ELF Header:
  Class:                             {}
  Data:                              {}
  Version:                           {}
  OS/ABI:                            {}
  ABI Version:                       {}
  Type:                              {}
  Machine:                           {}
  Version:                           {}
  Entry point address:               0x{:x}
  Start of program headers:          {}
  Start of section headers:          {}
  Flags:                             0x{:x}
  Size of this header:              ` {} (bytes)
  Size of program headers:           {} (bytes)
  Number of program headers:         {}
  Size of section headers:           {} (bytes)
  Number of section headers:         {}
  Section header string table index: {}",
        ident.ei_class,
        ident.ei_data,
        ident.ei_version,
        ident.ei_osabi,
        ident.ei_abiversion,
        header.r#type,
        header.machine,
        header.version,
        header.entry_point,
        header.ph_offset,
        header.sh_offset,
        header.flags,
        header.eh_size,
        header.phent_size,
        header.phnum,
        header.shent_size,
        header.shnum,
        header.shstrndx
    );
}
