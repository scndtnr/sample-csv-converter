use chrono::{Duration, NaiveDate};

use super::{SourceRecord, SourceRecords};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub(crate) struct TransactionCRecord {
    customer_id: String,
    promise_date: String,
    other_column: String,
}

impl TransactionCRecord {
    fn new(
        source_record: &SourceRecord,
        reference_date: NaiveDate,
        current_day: usize,
        promise_day: usize,
    ) -> Self {
        Self {
            customer_id: source_record.customer_id().to_string(),
            promise_date: (reference_date - Duration::days(current_day as i64)
                + Duration::days(promise_day.to_owned() as i64))
            .format("%Y-%m-%d")
            .to_string(),
            other_column: "sample_data".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub(crate) struct TransactionCRecords(pub(crate) Vec<TransactionCRecord>);

impl TransactionCRecords {
    pub(crate) fn new(
        source_records: &SourceRecords,
        reference_date: NaiveDate,
        current_day: usize,
    ) -> Self {
        let records = source_records
            .0
            .iter()
            .flat_map(|source_record| {
                source_record
                    .transaction_c_promise_days()
                    .iter()
                    .map(|promise_day| {
                        TransactionCRecord::new(
                            source_record,
                            reference_date,
                            current_day,
                            *promise_day,
                        )
                    })
                    .collect::<Vec<TransactionCRecord>>()
            })
            .collect();
        Self(records)
    }
}
