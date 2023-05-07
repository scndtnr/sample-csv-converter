mod mastar_a;
mod source_record;
mod source_record_dto;
mod source_records;
mod transaction_b;
mod transaction_c;

pub(super) use mastar_a::MasterARecords;
pub(super) use source_record::SourceRecord;
pub(super) use source_record_dto::SourceRecordDto;
pub(super) use source_records::SourceRecords;
pub(super) use transaction_b::TransactionB;
pub(super) use transaction_c::TransactionC;

use anyhow::Result;
use chrono::NaiveDate;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, derive_new::new)]
pub(super) struct FileGenerator<'a, 'b> {
    source_records: &'a SourceRecords,
    reference_date: NaiveDate,
    current_day: usize,
    day_dir_path: &'b Path,
}
impl<'a, 'b> FileGenerator<'a, 'b> {
    /// サンプルデータ用CSVをまとめて生成する
    pub(super) fn write_files(&self) -> Result<()> {
        self.write_master_a_csv()?;
        Ok(())
    }

    /// レコードデータからファイルを作成する
    fn write<T: serde::Serialize>(
        records: &[T],
        dir_path: &Path,
        file_name: &str,
    ) -> Result<PathBuf> {
        let file_path = dir_path.join(file_name);
        let mut writer = csv::Writer::from_path(&file_path)?;
        for record in records {
            writer.serialize(record)?;
        }
        writer.flush()?;
        println!("Create File: {}", &file_path.to_string_lossy());
        Ok(file_path)
    }

    /// master_a データ用CSVを生成する
    fn write_master_a_csv(&self) -> Result<PathBuf> {
        let file_name = "master_a.csv";
        let records =
            MasterARecords::new(self.source_records, self.reference_date, self.current_day);
        Self::write(&records.0, self.day_dir_path, file_name)
    }
}
