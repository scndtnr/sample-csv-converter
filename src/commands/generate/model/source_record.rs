/// 入力CSVのデータ（レコード）
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, derive_new::new)]
pub(crate) struct SourceRecord {
    customer_id: String,
    master_a_due_day: usize,
    transaction_b_send_days: Vec<usize>,
    transaction_c_promise_days: Vec<usize>,
}

impl SourceRecord {
    pub(crate) fn customer_id(&self) -> &str {
        &self.customer_id
    }
    pub(crate) fn master_a_due_day(&self) -> usize {
        self.master_a_due_day
    }
    pub(crate) fn transaction_b_send_days(&self) -> &[usize] {
        &self.transaction_b_send_days
    }
    pub(crate) fn transaction_c_promise_days(&self) -> &[usize] {
        &self.transaction_c_promise_days
    }
}
