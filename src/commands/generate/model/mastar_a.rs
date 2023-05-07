use chrono::{Duration, NaiveDate};

use super::{SourceRecord, SourceRecords};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub(crate) struct MasterARecord {
    customer_id: String,
    start_date: String,
    due_date: usize,
    other_column: String,
}

impl MasterARecord {
    fn new(source_record: &SourceRecord, reference_date: NaiveDate, current_day: usize) -> Self {
        Self {
            customer_id: source_record.customer_id().to_string(),
            start_date: (reference_date - Duration::days(current_day as i64))
                .format("%Y-%m-%d")
                .to_string(),
            due_date: source_record.master_a_due_day(),
            other_column: "sample_data".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub(crate) struct MasterARecords(pub(crate) Vec<MasterARecord>);

impl MasterARecords {
    pub(crate) fn new(
        source_records: &SourceRecords,
        reference_date: NaiveDate,
        current_day: usize,
    ) -> Self {
        let records = source_records
            .0
            .iter()
            .map(|source_record| MasterARecord::new(source_record, reference_date, current_day))
            .collect();
        Self(records)
    }
}
