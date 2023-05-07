use std::env;

use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Config {
    pub(crate) reference_date: String,
    pub(crate) target_days: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            reference_date: Local::now().format("%Y-%m-%d").to_string(),
            target_days: 5,
        }
    }
}

impl Config {
    pub(crate) fn reference_date(&self) -> NaiveDate {
        NaiveDate::parse_from_str(&self.reference_date, "%Y-%m-%d")
            .expect("Fail to convert reference_date to NaiveDate.")
    }
    pub(crate) fn target_days(&self) -> usize {
        self.target_days
    }
}

/// 既定の設定情報名称を返す
fn default_config_name() -> (&'static str, Option<&'static str>) {
    let app_name = "sample-csv-generator";
    let config_name = None;
    (app_name, config_name)
}

/// 設定情報を読み込んで `Config` 構造体を返す
pub(crate) fn load_config() -> Config {
    let (app_name, config_name) = default_config_name();
    confy::load(app_name, config_name).expect("Fail to load config.")
}

/// 設定ファイルのパスや、設定内容を表示する
pub(crate) fn print_config() {
    let (app_name, config_name) = default_config_name();
    let cfg_path = confy::get_configuration_file_path(app_name, config_name)
        .expect("Fail to load config file path.");
    let cfg: Config = confy::load(app_name, config_name).expect("Fail to load config.");
    println!("Config File Path: {:#?}\n{:#?}", cfg_path, cfg);

    let pwd = env::current_dir().expect("Fail to get current directory.");
    println!("Currnet Directory: {:#?}", pwd);
}
