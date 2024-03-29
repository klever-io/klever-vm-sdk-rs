use num_bigint::BigUint;

use crate::{tx_execution::BuiltinFunctionContainer, types::VMAddress};

use super::{TxResult, TxTokenTransfer, TxInput};

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
        &mut self,
        own_address: &VMAddress,
        result: &TxResult,
        builtin_functions: &BuiltinFunctionContainer,
    ) {
        let mut bt = BackTransfers::default();

        for call in &result.all_calls {
            // TODO: refactor, check type

            if call.endpoint_name.is_empty() {
                bt.call_value += &call.call_value;
                continue;
            }

            let tx_input = TxInput{
                from: call.from.clone(),
                to: call.to.clone(),
                klv_value: call.call_value.clone(),
                kda_values: Vec::new(),
                func_name: call.endpoint_name.clone(),
                args: call.arguments.clone(),
                gas_limit: 1000,
                gas_price: 0,
                tx_hash: call.tx_hash.clone(),
                ..Default::default()
            };
            let mut token_transfers = builtin_functions.extract_token_transfers(&tx_input);
            if &token_transfers.real_recipient == own_address {
                bt.kda_transfers.append(&mut token_transfers.transfers);
            }
        }

        self.call_value = bt.call_value;
        self.kda_transfers = bt.kda_transfers;
    }
}