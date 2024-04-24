mod csv;

use crate::cli::csv::CsvOpts;
use crate::CmdExec;
use clap::Parser;
use std::path::Path;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
}

impl CmdExec for SubCommand {
    fn execute(self) -> anyhow::Result<()> {
        match self {
            SubCommand::Csv(opts) => crate::process_csv(&opts.input, &opts.output),
        }
    }
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}
