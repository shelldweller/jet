use std::path::PathBuf;
use structopt::StructOpt;
use crate::enums::SerdeFormat;

#[derive(StructOpt, Debug)]
#[structopt(name = "jet")]
pub struct Opt {
    // with option
    #[structopt(short, long, default_value = "", help = "JSON path expression, e.g., `response.results`")]
    pub with: String,

    // map option
    #[structopt(short, long, default_value = "", help = "Space separated JSON path expressions, e.g., `some.path path.to.array[*].foo`")]
    pub map: String,

    // filter option
    #[structopt(short, long, default_value = "", help = "Filter expression, e.g., `some.element == \"foo\"`")]
    pub filter: String,

    // input format
    #[structopt(short, long, default_value = "json" )]
    pub input_format: SerdeFormat,

    // output format
    #[structopt(short, long, default_value = "json" )]
    pub output_format: SerdeFormat,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    pub files: Vec<PathBuf>,
}
