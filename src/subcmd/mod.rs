use clap::Subcommand;

mod create_fee_acc;

pub use create_fee_acc::*;

#[derive(Debug, Subcommand)]
pub enum Subcmd {
    CreateFeeAcc(CreateFeeAccArgs),
}

pub trait SubcmdExec {
    fn process_cmd(&self, args: &crate::Args);
}

impl SubcmdExec for Subcmd {
    fn process_cmd(&self, args: &crate::Args) {
        match self {
            Self::CreateFeeAcc(a) => a.process_cmd(args),
        }
    }
}
