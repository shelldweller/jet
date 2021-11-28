use std::io::BufReader;
use std::fs::File;
use serde_json::{Value};

use crate::cli::Opt;
use crate::enums::SerdeFormat;
use crate::readers::{JsonReader, JsonLineReader};


pub fn run_jet(args: Opt) {
    for path in args.files {
        println!("Will read from {:?}", path);
        let input_file = File::open(path).unwrap();
        let buf_reader = Box::new(BufReader::new(input_file));

        let reader: Box<dyn Iterator<Item = Result<Value, String>>> = match args.input_format {
            SerdeFormat::Json => Box::new(JsonReader::new(buf_reader)),
            SerdeFormat::JsonLine => Box::new(JsonLineReader::new(buf_reader)),
        };
        for document in reader {
            println!("PROCESSING {:?}", document);
        }
    }
}
