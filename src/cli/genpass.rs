use clap::Parser;

//默认 uppercase lowercase number 是默认为true 只有声明才会为false
//rcli genpass -l 21 --no-uppercase --no-lowercase --no-number`
#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, action = clap::ArgAction::SetFalse)]
    pub no_uppercase: bool,

    #[arg(long, action = clap::ArgAction::SetFalse)]
    pub no_lowercase: bool,

    #[arg(long, action = clap::ArgAction::SetFalse)]
    pub no_number: bool,

    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}
