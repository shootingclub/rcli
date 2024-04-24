mod csv;

use crate::cli::csv::CsvOpts;
use crate::CmdExec;
use clap::Parser;

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
        return match self {
            SubCommand::Csv(opts) => crate::process_csv(&opts.input, &opts.output),
        };
    }
}
