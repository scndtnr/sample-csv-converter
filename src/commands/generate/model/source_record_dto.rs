use chrono::NaiveDate;

use super::SourceRecord;

/// 入力CSVの生データ（レコード）
///
/// ```csv
///
///
/// customer_id,master_a_due_day,transaction_b_send_days,transaction_c_promise_days
/// c001,27,"4,5","17,18"
/// ```
///
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, derive_new::new, serde::Deserialize)]
pub(crate) struct SourceRecordDto {
    customer_id: String,
    master_a_due_day: usize,
    transaction_b_send_days: String,
    transaction_c_promise_days: String,
}

impl From<SourceRecordDto> for SourceRecord {
    fn from(dto: SourceRecordDto) -> Self {
        Self::new(
            dto.customer_id,
            dto.master_a_due_day,
            dto.transaction_b_send_days
                .split(',')
                .map(|day| day.parse().expect("Fail to parse day_str to usize."))
                .collect(),
            dto.transaction_c_promise_days
                .split(',')
                .map(|day| day.parse().expect("Fail to parse day_str to usize."))
                .collect(),
        )
    }
}
