use clap::Subcommand;

mod create_fee_acc;
mod fund_sol_bridge;
mod list_fee_accs;

pub use create_fee_acc::*;
pub use fund_sol_bridge::*;
pub use list_fee_accs::*;

#[derive(Debug, Subcommand)]
pub enum Subcmd {
    CreateFeeAcc(CreateFeeAccArgs),
    FundSolBridge(FundSolBridgeArgs),
    ListFeeAccs(ListFeeAccsArgs),
}

pub trait SubcmdExec {
    fn process_cmd(&self, args: &crate::Args);
}

impl SubcmdExec for Subcmd {
    fn process_cmd(&self, args: &crate::Args) {
        match self {
            Self::CreateFeeAcc(a) => a.process_cmd(args),
            Self::FundSolBridge(a) => a.process_cmd(args),
            Self::ListFeeAccs(a) => a.process_cmd(args),
        }
    }
}
