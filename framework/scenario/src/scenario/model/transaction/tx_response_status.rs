#[derive(Debug, Default, Clone)]
/// The status of a transaction.
pub struct TxResponseStatus {
    /// The status of the transaction.
    pub status: u64,
    /// The message of the transaction.
    pub message: String,
}

impl TxResponseStatus {
    /// Checks if the transaction was successful.
    pub fn is_success(&self) -> bool {
        self.status == 0
    }
}

impl std::fmt::Display for TxResponseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_success() {
            write!(f, "transaction successful")
        } else {
            write!(
                f,
                "transaction failed: (status: {}, message: {})",
                self.status, self.message
            )
        }
    }
}
