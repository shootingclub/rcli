// rcli csv -i input.csv -o output.json --header -d ','

use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    if let Some(name) = opts.name.as_ref() {
        println!("name: {}", name);
    }

    if let Some(config_path) = opts.config.as_deref() {
        println!("config: {}", config_path.display());
    }

    match opts.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    match opts.cmd {
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output)?,
    }
    Ok(())
}
