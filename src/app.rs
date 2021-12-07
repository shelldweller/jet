use serde_json::{Value};
use std::fs::File;
use std::io::{BufRead, BufReader, self};
use std::path::PathBuf;

use crate::cli::Opt;
use crate::enums::SerdeFormat;
use crate::readers::{JsonReader, JsonLineReader};
use crate::writers::{JsonWriter, JsonLineWriter, Writer};


struct StdinIterator {
    already_open: bool
}

impl StdinIterator {
    fn new() -> Self {
        Self { already_open: false }
    }
}

impl Iterator for StdinIterator {
    type Item = Result<Box<dyn BufRead>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.already_open {
            None
        } else {
            self.already_open = true;
            Some(Ok(Box::new(BufReader::new(io::stdin()))))
        }
    }
}


struct FileIterator {
    files: Vec<PathBuf>,
    current_file: usize,
}

impl FileIterator {
    fn new(files: Vec<PathBuf>) -> Self {
        Self {
            files: files,
            current_file: 0
        }
    }
}

impl Iterator for FileIterator {
    type Item = Result<Box<dyn BufRead>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_file >= self.files.len() {
            return None;
        }
        let result = File::open(&self.files[self.current_file]);
        match result {
            Ok(f) => {
                self.current_file += 1;
                return Some(Ok(Box::new(BufReader::new(f))));
            },
            Err(error) => {
                let error_message = format!("{}: {}", self.files[self.current_file].to_string_lossy(), error);
                self.current_file += 1;
                return Some(Err(error_message));
            }
        };
    }
}



pub fn run_jet(args: Opt) {
    let input_sources: Box<dyn Iterator<Item = Result<Box<dyn BufRead>, String>>> = if args.files.len() == 0 {
        Box::new(StdinIterator::new())
    } else {
        Box::new(FileIterator::new(args.files))
    };

    let mut writer: Box<dyn Writer> = match args.output_format {
        SerdeFormat::Json => Box::new(JsonWriter::new(Box::new(io::stdout()))),
        SerdeFormat::JsonLine => Box::new(JsonLineWriter::new(Box::new(io::stdout()))),
    };

    for source in input_sources {
        if let Err(error) = source {
            eprintln!("{}", error);
            continue;
        };
        let input = source.unwrap();
        let reader: Box<dyn Iterator<Item = Result<Value, String>>> = match args.input_format {
            SerdeFormat::Json => Box::new(JsonReader::new(input)),
            SerdeFormat::JsonLine => Box::new(JsonLineReader::new(input)),
        };
        for item in reader {
            match item {
                Ok(document) => writer.write(document),
                Err(error) => eprintln!("{:?}", error),
            };
        }
    }
    writer.done();
}
