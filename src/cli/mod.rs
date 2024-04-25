pub mod csv;

use crate::cli::csv::{CsvOpts, OutPutFormat};
use crate::CmdExec;
use clap::Parser;
use std::path::Path;
use std::str::FromStr;

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
            SubCommand::Csv(opts) => {
                let output = if let Some(output) = opts.output {
                    output.clone()
                } else {
                    format!("output.{}", opts.format)
                };
                crate::process_csv(&opts.input, output, opts.format)
            }
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

fn parse_format(format: &str) -> Result<OutPutFormat, anyhow::Error> {
    format.parse::<OutPutFormat>()
}

//From OutPutFormat 转化为 &'static str
//From 与 Into 是一对trait 实现了From 就可以使用Into
impl From<OutPutFormat> for &'static str {
    fn from(format: OutPutFormat) -> Self {
        match format {
            OutPutFormat::Json => "json",
            OutPutFormat::Yaml => "yaml",
        }
    }
}

impl FromStr for OutPutFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutPutFormat::Json),
            "yaml" => Ok(OutPutFormat::Yaml),
            _ => anyhow::bail!("Unsupported format!"),
        }
    }
}
