use chrono::{Duration, NaiveDate};

use super::SourceRecords;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    derive_new::new,
    serde::Deserialize,
    serde::Serialize,
)]
pub(crate) struct MasterARecord {
    customer_id: String,
    start_date: String,
    due_date: usize,
    other_column: String,
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
            .map(|record| {
                MasterARecord::new(
                    record.customer_id().to_string(),
                    (reference_date - Duration::days(current_day as i64))
                        .format("%Y-%m-%d")
                        .to_string(),
                    record.master_a_due_day(),
                    "sample_data".to_string(),
                )
            })
            .collect();
        Self(records)
    }
}
