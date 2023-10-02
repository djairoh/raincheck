pub mod config;
pub mod cli;

use crate::config::Config;
use crate::cli::Cli;
use clap::Parser;
use log::error;
use std::ffi::OsString;


fn main() {
    // CLI options
    let cli:Cli = Cli::parse();

    // logging initialisation
    std::env::set_var::<&str, OsString>("RUST_LOG", cli.log_level.into());
    if let Err(e) = env_logger::init() {
      error!("{e}");
      return
    }

    match confy::load::<Config>("raincheck", cli.config_file.as_str()) {
        Ok(cfg) => {
            println!("{}", cfg.api_key);
        },
        Err(e) => {
            error!("{e}");
        },
    }
}
