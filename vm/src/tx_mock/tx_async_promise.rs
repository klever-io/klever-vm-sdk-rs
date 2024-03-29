use super::{CallTxData, TxFunctionName};

#[derive(Clone, Debug)]
pub struct Promise {
    pub call: CallTxData,
    pub success_callback: TxFunctionName,
    pub error_callback: TxFunctionName,
    pub callback_closure_data: Vec<u8>,
}
