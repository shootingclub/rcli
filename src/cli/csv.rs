use super::verify_input_file;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short = 'i', long, value_parser = verify_input_file)]
    pub input: String,

    #[arg(short = 'o', long, default_value = "output.json")] // "output.json".into()
    pub output: String,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}
