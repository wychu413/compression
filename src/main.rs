use std::error::Error;
use std::fs;
use clap::{crate_version, App, Arg};

mod pkg;
use pkg::printer;
use pkg::printer::MsgLevel;

mod compressor;
use compressor::compressor::Compressor;

#[derive(Debug)]
struct Config {
    // the algorithm to compress/decompress the data
    algo: String,
    // compress/decompress
    is_decompress: bool,
    // the input file name
    input_file: String,
    // the input data read from the input file
    input_data: Vec<u8>,
    // the output file name
    output_file: String,
}

fn main() {
    let matches = App::new("Text Compression Utility")
        .version(crate_version!())
        .author("wychu")
        .about("Simple implementation of Text Compression")
        .after_help("Welcome!")
        .arg(
            Arg::with_name("decompress")
                .help("Action: Compress")
                .short("d")
                .long("decompress")
                .takes_value(false)
                .required(true)
        )
        .arg(
            Arg::with_name("inputFile")
                .help("The input file name")
                .short("i")
                .long("input")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("outputFile")
                .help("The output file name")
                .short("o")
                .long("output")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("algo")
                .help("The compression algorithm that's going to be used on the Text file")
                .short("a")
                .long("algo")
                .takes_value(true)
                .default_value("huffman")
        )
        .get_matches();

    let input_file = matches.value_of("inputFile").unwrap();

    let output_file = matches.value_of("outputFile").unwrap();

    let algo = matches.value_of("algo").unwrap();

    let is_decompress = matches.is_present("decompress").unwrap();

    let buffer = fs::read(input_file)?;

    let config = Config {
        is_decompress: is_decompress,
        input_file: String::from(input_file),
        input_data: buffer,
        output_file: String::from(output_file),
        algo: algo,
    };
    printer::print(MsgLevel::Info, format!("target file with size {} bytes and output to {}", config.input_data.len()*8, output_file).as_str());
}










