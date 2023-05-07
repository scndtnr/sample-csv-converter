mod model;

use self::model::{FileGenerator, SourceRecordDto, SourceRecords};
use crate::commands::{config::load_config, convert_to_path};
use anyhow::Result;
use std::path::{Path, PathBuf};

pub(crate) fn generate_csv(src: String, dst: String, dry_run: bool) -> Result<()> {
    let cfg = load_config();
    let src_path = convert_to_path(&src);
    let output_dir_path = convert_to_path(&dst).join(&cfg.output_directory_name);

    // 入力用データを読み込む
    let source_records = load_source_records(src_path)?;
    println!("{:#?}", source_records);

    // 出力用ディレクトリを空にする
    delete_output_directory(&output_dir_path)?;
    create_output_directory(&output_dir_path)?;

    // 対象日付毎に出力用データを生成する
    for current_day in 1..=cfg.target_days() {
        let day_dir_path = create_day_directory(&output_dir_path, current_day)?;
        let file_generator = FileGenerator::new(
            &source_records,
            cfg.reference_date(),
            current_day,
            &day_dir_path,
        );
        file_generator.write_files()?;
    }
    Ok(())
}

/// 入力用データを読み込む
fn load_source_records(src_path: PathBuf) -> Result<SourceRecords> {
    // イテレータから直接ベクタに変換できないので、
    // forループ内でベクタにレコードを詰め込んでいる
    let mut source_records_dto = Vec::new();
    let mut reader = csv::Reader::from_path(src_path)?;
    for result in reader.deserialize() {
        let record: SourceRecordDto = result?;
        source_records_dto.push(record);
    }
    Ok(source_records_dto.into())
}

/// 出力用ディレクトリを削除する
fn delete_output_directory(output_dir_path: &Path) -> Result<()> {
    if output_dir_path.is_dir() {
        println!("Remove Directory: {}", output_dir_path.to_string_lossy());
        std::fs::remove_dir_all(output_dir_path)?
    }
    Ok(())
}

/// 出力用ディレクトリを作成する
fn create_output_directory(output_dir_path: &Path) -> Result<()> {
    if !output_dir_path.is_dir() {
        println!("Create Directory: {}", output_dir_path.to_string_lossy());
        std::fs::create_dir_all(output_dir_path)?
    }
    Ok(())
}

/// 出力用ディレクトリに日次ディレクトリを作成する
fn create_day_directory(output_dir_path: &Path, day: usize) -> Result<PathBuf> {
    let day_dir_path = output_dir_path.join(format!("Day{:02}", day));
    if !day_dir_path.is_dir() {
        println!("Create Directory: {}", day_dir_path.to_string_lossy());
        std::fs::create_dir_all(&day_dir_path)?
    }
    Ok(day_dir_path)
}
