mod options;

use self::options::{Commands, Opts};
use clap::Parser;

#[derive(Debug, Clone)]
pub(super) struct Cui {
    opts: Opts,
}

impl Cui {
    pub(super) async fn new() -> Self {
        Self {
            opts: Opts::parse(),
        }
    }

    pub(super) async fn process(&self) {
        match self.opts.command().clone() {
            Commands::Generate(args) => {
                println!("{:#?}", args);
                crate::commands::generate_csv(args.src, args.dst)
                    .expect("Fail to generate csv files.");
            }
            Commands::Config => crate::commands::print_config(),
        }
    }
}
