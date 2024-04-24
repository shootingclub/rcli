use clap::Parser;
use rcli::{cli::Opts, CmdExec};
fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    return opts.cmd.execute();
}
