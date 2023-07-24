use std::{path::PathBuf, time::Duration};

use clap::Args;
use tokio::{runtime::Runtime, time::sleep};

use super::SubcmdExec;

mod consts;
mod db;

#[derive(Args, Debug)]
#[command(
    long_about = "Index historical successful stakedex transactions into a sqlite DB, from newest to oldest."
)]
pub struct IndexArgs {
    #[arg(
        help = "Path to sqlite file to save data to",
        default_value = "stakedex.sqlite"
    )]
    pub sqlite_file: Option<PathBuf>,
}

impl SubcmdExec for IndexArgs {
    fn process_cmd(&self, _args: &crate::Args) {
        let rt = Runtime::new().unwrap();
        rt.block_on(async move {
            sleep(Duration::from_secs(1)).await;
            println!("done");
        });
        rt.shutdown_timeout(Duration::from_secs(5));
    }
}
