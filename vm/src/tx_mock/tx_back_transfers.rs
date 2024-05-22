use num_bigint::BigUint;

use crate::{tx_execution::BuiltinFunctionContainer, types::VMAddress};

use super::{TxResult, TxTokenTransfer, TxInput, call_tx_input, CallType};

#[derive(Default)]
pub struct BackTransfers {
    pub call_value: BigUint,
    pub kda_transfers: Vec<TxTokenTransfer>,
}

impl BackTransfers {
    pub fn empty() -> Self {
        BackTransfers::default()
    }

    pub fn new_from_result(
        own_address: &VMAddress,
        result: &TxResult,
        builtin_functions: &BuiltinFunctionContainer,
    ) -> Self {
        let mut bt = BackTransfers::default();

        if result.result_status != 0 {
            return bt;
        }

        for call in &result.all_calls {
            // TODO: refactor, check type

            if call.endpoint_name.is_empty() {
                bt.call_value += &call.call_value;
                continue;
            }

            let tx_input = call_tx_input(call, CallType::BackTransfer);
            let mut token_transfers = builtin_functions.extract_token_transfers(&tx_input);
            if &token_transfers.real_recipient == own_address {
                bt.kda_transfers.append(&mut token_transfers.transfers);
            }
        }

        bt
    }

    pub fn merge(&mut self, other: &BackTransfers) {
        self.call_value += &other.call_value;
        self.kda_transfers.extend_from_slice(&other.kda_transfers);
    }
}