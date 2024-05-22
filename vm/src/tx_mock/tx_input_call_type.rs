#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallType {
    DirectCall,
    ExecuteOnDestContext,
    AsyncCall,
    TransferExecute,
    BackTransfer,
    UpgradeFromSource
}

impl CallType {
    pub fn to_log_bytes(&self) -> Vec<u8> {
        self.as_log_str().into()
    }

    fn as_log_str(&self) -> &'static str {
        match self {
            Self::DirectCall => "DirectCall",
            Self::ExecuteOnDestContext => "ExecuteOnDestContext",
            Self::TransferExecute => "TransferAndExecute",
            Self::BackTransfer => "BackTransfer",
            Self::UpgradeFromSource => "UpgradeFromSource",
            // Just for compatibility with legacy code. Should be removed within Async functions in the future.
            Self::AsyncCall => "AsyncCall",
        }
    }
}