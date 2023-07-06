use clap::Args;

use super::SubcmdExec;

#[derive(Args, Debug)]
#[command(long_about = "List all created fee token accounts for all xSOL mints")]
pub struct ListFeeAccsArgs;

impl SubcmdExec for ListFeeAccsArgs {
    fn process_cmd(&self, _args: &crate::Args) {}
}
