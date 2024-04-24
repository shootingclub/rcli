pub mod cli;
mod process;

pub use cli::*;
use enum_dispatch::enum_dispatch;
pub use process::*;


#[enum_dispatch]
pub trait CmdExec {
    fn execute(self) -> anyhow::Result<()>;
}