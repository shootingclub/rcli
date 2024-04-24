pub mod cli;
mod process;

pub use cli::*;
pub use process::*;


pub trait CmdExec {
    fn execute(self) -> anyhow::Result<()>;
}