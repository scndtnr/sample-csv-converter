use super::{SourceRecord, SourceRecordDto};

/// 入力CSVのデータ（テーブル）
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, derive_new::new)]
pub(crate) struct SourceRecords(pub(crate) Vec<SourceRecord>);

impl From<Vec<SourceRecordDto>> for SourceRecords {
    fn from(dtos: Vec<SourceRecordDto>) -> Self {
        Self::new(dtos.into_iter().map(Into::<SourceRecord>::into).collect())
    }
}
