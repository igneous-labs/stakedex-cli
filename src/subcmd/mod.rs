use clap::Subcommand;

mod create_fee_acc;
mod fund_sol_bridge;

pub use create_fee_acc::*;
pub use fund_sol_bridge::*;

#[derive(Debug, Subcommand)]
pub enum Subcmd {
    CreateFeeAcc(CreateFeeAccArgs),
    FundSolBridge(FundSolBridgeArgs),
}

pub trait SubcmdExec {
    fn process_cmd(&self, args: &crate::Args);
}

impl SubcmdExec for Subcmd {
    fn process_cmd(&self, args: &crate::Args) {
        match self {
            Self::CreateFeeAcc(a) => a.process_cmd(args),
            Self::FundSolBridge(a) => a.process_cmd(args),
        }
    }
}
