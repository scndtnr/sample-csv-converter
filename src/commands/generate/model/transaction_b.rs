use chrono::{Duration, NaiveDate};

use super::{SourceRecord, SourceRecords};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub(crate) struct TransactionBRecord {
    customer_id: String,
    send_date: String,
    other_column: String,
}

impl TransactionBRecord {
    fn new(
        source_record: &SourceRecord,
        reference_date: NaiveDate,
        current_day: usize,
        send_day: usize,
    ) -> Self {
        Self {
            customer_id: source_record.customer_id().to_string(),
            send_date: (reference_date - Duration::days(current_day as i64)
                + Duration::days(send_day.to_owned() as i64))
            .format("%Y-%m-%d")
            .to_string(),
            other_column: "sample_data".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub(crate) struct TransactionBRecords(pub(crate) Vec<TransactionBRecord>);

impl TransactionBRecords {
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
                    .transaction_b_send_days()
                    .iter()
                    .map(|send_day| {
                        TransactionBRecord::new(
                            source_record,
                            reference_date,
                            current_day,
                            *send_day,
                        )
                    })
                    .collect::<Vec<TransactionBRecord>>()
            })
            .collect();
        Self(records)
    }
}
