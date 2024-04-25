use super::{parse_format, verify_input_file};
use clap::Parser;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, Copy)]
pub enum OutPutFormat {
    Json,
    Yaml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short = 'i', long, value_parser = verify_input_file)]
    pub input: String,

    #[arg(short = 'o', long)] // "output.json".into()
    pub output: Option<String>,

    #[arg(short = 'f', long, value_parser = parse_format, default_value = "json")]
    pub format: OutPutFormat,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

impl fmt::Display for OutPutFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
