use clap::{Args, Parser, Subcommand};

/// コマンドライン引数のパース用構造体
#[derive(Debug, Clone, Parser)]
#[clap(
    name = "Sample CSV Generator",
    version = "0.1.0",
    author = "zumi",
    about = "A CLI tool that generates multiple sample data CSV files based on the input CSV file."
)]
#[clap(propagate_version = true)]
pub(crate) struct Opts {
    #[clap(subcommand)]
    command: Commands,
}

impl Opts {
    pub(super) fn command(&self) -> &Commands {
        &self.command
    }
}

#[derive(Debug, Clone, Subcommand)]
pub(super) enum Commands {
    /// 与えられたCSVを元に、サンプルデータ用CSVを生成する
    Generate(GenerateArgs),
    /// 設定情報を出力する
    Config,
}

#[derive(Debug, Clone, Args)]
pub(super) struct GenerateArgs {
    #[clap(help = "入力として扱うCSVのpathを指定する")]
    pub(super) src: String,
    #[clap(help = "CSV出力先ディレクトリのpathを指定する")]
    pub(super) dst: String,
}
