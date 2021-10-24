use std::process;
use std::fs;
use clap::{crate_version, App, Arg};

mod pkg;
use pkg::printer;
use pkg::printer::MsgLevel;

mod compression;
use crate::compression::compressor;

#[derive(Debug)]
struct Config<'a> {
    input_text: String,
    output_file: &'a str,
    compression_alg: &'a str,
}

fn main() {
    let matches = App::new("Text Compression Utility")
        .version(crate_version!())
        .author("wychu")
        .about("Simple implementation of Text Compression")
        .after_help("Welcome!")
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
            Arg::with_name("compAlg")
                .help("The compression algorithm that's going to be used on the Text file")
                .short("a")
                .long("alg")
                .takes_value(true)
                .default_value("huffman")
        )
        .get_matches();

    let input_text = if let Some(input_file) = matches.value_of("inputFile") {
        fs::read_to_string(input_file).expect(format!("Fail to read file {}", input_file).as_str())
    } else {
        printer::print(MsgLevel::Error, "Failed with unknown Error");
        process::exit(1);
    };

    let output_file = if let Some(o) = matches.value_of("outputFile") {   
        o
    } else {
        printer::print(MsgLevel::Error, "Failed with unknown Error");
        process::exit(1);
    };

    let alg = if let Some(ca) = matches.value_of("compAlg") {
        ca
    } else {
        printer::print(MsgLevel::Error, "Failed with unknown Error");
        process::exit(1);
    };

    let config = Config {
        input_text: input_text,
        output_file: output_file,
        compression_alg: alg,
    };

    printer::print(MsgLevel::Info, format!("target file with size {} bytes and output to {}", config.input_text.len()*8, output_file).as_str());
}

