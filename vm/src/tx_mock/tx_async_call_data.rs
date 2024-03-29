use crate::{
    tx_execution::BuiltinFunctionContainer,
    tx_mock::{TxInput, TxResult},
    types::{top_encode_u64, VMAddress, H256},
};

use num_bigint::BigUint;

use super::{CallbackPayments, Promise, TxFunctionName};

#[derive(Debug, Clone)]

// TODO: ...
pub struct CallTxData {
    pub from: VMAddress,
    pub to: VMAddress,
    pub call_value: BigUint,
    pub endpoint_name: TxFunctionName,
    pub arguments: Vec<Vec<u8>>,
    pub tx_hash: H256,
}

pub fn call_tx_input(call_data: &CallTxData) -> TxInput {
    TxInput {
        from: call_data.from.clone(),
        to: call_data.to.clone(),
        klv_value: call_data.call_value.clone(),
        kda_values: Vec::new(),
        func_name: call_data.endpoint_name.clone(),
        args: call_data.arguments.clone(),
        gas_limit: 1000,
        gas_price: 0,
        tx_hash: call_data.tx_hash.clone(),
        ..Default::default()
    }
}

fn result_status_bytes(result_status: u64) -> Vec<u8> {
    if result_status == 0 {
        vec![0x00]
    } else {
        top_encode_u64(result_status)
    }
}

pub fn callback_tx_input(
    data: &CallTxData,
    result: &TxResult,
    builtin_functions: &BuiltinFunctionContainer,
) -> TxInput {
    let mut args: Vec<Vec<u8>> = vec![result_status_bytes(result.result_status)];
    if result.result_status == 0 {
        args.extend_from_slice(result.result_values.as_slice());
    } else {
        args.push(result.result_message.clone().into_bytes());
    }
    let callback_payments =
        extract_callback_payments(&data.from, result, builtin_functions);
    TxInput {
        from: data.to.clone(),
        to: data.from.clone(),
        klv_value: 0u32.into(),
        kda_values: Vec::new(),
        func_name: TxFunctionName::CALLBACK,
        args,
        gas_limit: 1000,
        gas_price: 0,
        tx_hash: data.tx_hash.clone(),
        callback_payments,
        ..Default::default()
    }
}

fn extract_callback_payments(
    callback_contract_address: &VMAddress,
    result: &TxResult,
    builtin_functions: &BuiltinFunctionContainer,
) -> CallbackPayments {
    let mut callback_payments = CallbackPayments::default();
    for call in &result.all_calls {
        let tx_input = call_tx_input(call);
        let token_transfers = builtin_functions.extract_token_transfers(&tx_input);
        if &token_transfers.real_recipient == callback_contract_address {
            if !token_transfers.is_empty() {
                callback_payments.kda_values = token_transfers.transfers;
            } else {
                callback_payments.klv_value = call.call_value.clone();
            }
            break;
        }
    }
    callback_payments
}

pub fn promise_tx_input(
    address: &VMAddress,
    promise: &Promise,
    result: &TxResult,
) -> TxInput {
    let mut args: Vec<Vec<u8>> = Vec::new();
    let serialized_bytes = result.result_status.to_be_bytes().to_vec();
    args.push(serialized_bytes);
    let callback_name = if result.result_status == 0 {
        args.extend_from_slice(result.result_values.as_slice());
        promise.success_callback.clone()
    } else {
        args.push(result.result_message.clone().into_bytes());
        promise.error_callback.clone()
    };

    TxInput {
        from: promise.call.from.clone(),
        to: address.clone(),
        klv_value: 0u32.into(),
        kda_values: Vec::new(),
        func_name: callback_name,
        args,
        gas_limit: 1000,
        gas_price: 0,
        tx_hash: promise.call.tx_hash.clone(),
        promise_callback_closure_data: promise.callback_closure_data.clone(),
        ..Default::default()
    }
}

pub fn merge_results(mut original: TxResult, mut new: TxResult) -> TxResult {
    if original.result_status == 0 {
        original.result_values.append(&mut new.result_values);
        original.result_logs.append(&mut new.result_logs);
        original.result_message = new.result_message;
        original
    } else {
        new
    }
}
