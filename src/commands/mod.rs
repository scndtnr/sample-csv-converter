mod config;
mod generate;

pub(super) use self::config::print_config;
use self::config::{load_config, Config};
pub(super) use self::generate::generate_csv;

use std::path::PathBuf;
use std::str::FromStr;

/// 文字列を `PathBuf` に変換する
fn convert_to_path(path_str: &str) -> PathBuf {
    PathBuf::from_str(path_str).expect("Fail to &str to PathBuf.")
}
