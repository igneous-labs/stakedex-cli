use clap::Subcommand;

mod create_fee_acc;
mod fund_sol_bridge;
mod record_dex;

pub use create_fee_acc::*;
pub use fund_sol_bridge::*;
pub use record_dex::*;

#[derive(Debug, Subcommand)]
pub enum Subcmd {
    CreateFeeAcc(CreateFeeAccArgs),
    FundSolBridge(FundSolBridgeArgs),
    RecordDex(RecordDexArgs),
}

pub trait SubcmdExec {
    fn process_cmd(&self, args: &crate::Args);
}

impl SubcmdExec for Subcmd {
    fn process_cmd(&self, args: &crate::Args) {
        match self {
            Self::CreateFeeAcc(a) => a.process_cmd(args),
            Self::FundSolBridge(a) => a.process_cmd(args),
            Self::RecordDex(a) => a.process_cmd(args),
        }
    }
}
