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
pub(crate) struct TransactionB {
    customer_id: String,
    send_date: String,
    other_column: String,
}
