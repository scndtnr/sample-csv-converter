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
pub(crate) struct TransactionC {
    customer_id: String,
    promise_date: String,
    other_column: String,
}
