use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

type ParseError = &'static str;

#[derive(Debug)]
enum SerdeFormat {
    Json,
    JsonLine,
    Csv
}

impl FromStr for SerdeFormat {
    type Err = ParseError;
    fn from_str(day: &str) -> Result<Self, Self::Err> {
        match day {
            "json" => Ok(SerdeFormat::Json),
            "jsonline" => Ok(SerdeFormat::JsonLine),
            "csv" => Ok(SerdeFormat::Csv),
            _ => Err("Invalid format"),
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "jet")]
struct Opt {
    // with option
    #[structopt(short, long, default_value = "", help = "JSON path expression, e.g., `response.results`")]
    with: String,

    // map option
    #[structopt(short, long, default_value = "", help = "Space separated JSON path expressions, e.g., `some.path path.to.array[*].foo`")]
    map: String,

    // filter option
    #[structopt(short, long, default_value = "", help = "Filter expression, e.g., `some.element == \"foo\"`")]
    filter: String,

    // input format
    #[structopt(short, long, default_value = "json" )]
    input_format: SerdeFormat,

    // input format
    #[structopt(short, long, default_value = "json" )]
    output_format: SerdeFormat,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

pub fn run_cli() {
    let opt = Opt::from_args();
    // Init mapper
    // Init filter
    // Init reader
    // Init writer
    // For document in reader:
    //      if filter(document):
    //          mapped_doc = mapper(document)
    //          writer.write(mapped_doc)
    println!("{:#?}", opt);
}
