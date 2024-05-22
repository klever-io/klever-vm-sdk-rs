use crate::{
    tx_execution::{builtin_function_names::KLEVER_TRANSFER_FUNC_NAME, BlockchainVMRef},
    types::top_decode_u64,
};

use crate::{
    tx_execution::BuiltinFunctionKdaTransferInfo,
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult},
    types::VMAddress,
};
use crate::tx_execution::builtin_function_mocks::transfer::transfer_common::{adjust_call_type, push_func_name_if_necessary, push_transfer_bytes};
use crate::tx_mock::TxLog;
use crate::types::top_encode_u64;

use super::{
    super::BuiltinFunction,
    transfer_common::{
        execute_transfer_builtin_func, extract_transfer_info, ParsedTransferBuiltinFunCall,
        RawKdaTransfer,
    },
};

pub struct KDAMultiTransfer;

impl BuiltinFunction for KDAMultiTransfer {
    fn name(&self) -> &str {
        KLEVER_TRANSFER_FUNC_NAME
    }

    fn extract_kda_transfers(&self, tx_input: &TxInput) -> BuiltinFunctionKdaTransferInfo {
        if let Ok(parsed_tx) = try_parse_input(tx_input) {
            extract_transfer_info(parsed_tx)
        } else {
            BuiltinFunctionKdaTransferInfo::empty(tx_input)
        }
    }

    fn execute<F>(
        &self,
        tx_input: TxInput,
        tx_cache: TxCache,
        vm: &BlockchainVMRef,
        f: F,
    ) -> (TxResult, BlockchainUpdate)
    where
        F: FnOnce(),
    {
        match try_parse_input(&tx_input) {
            Ok(parsed_tx) => {
                let log = build_log(&tx_input, &parsed_tx);
                execute_transfer_builtin_func(vm, parsed_tx, tx_input, tx_cache, log, f)
            },
            Err(message) => {
                let err_result = TxResult::from_vm_error(message);
                (err_result, BlockchainUpdate::empty())
            },
        }
    }
}

fn build_log(tx_input: &TxInput, call: &ParsedTransferBuiltinFunCall) -> TxLog {
    let call_type = adjust_call_type(tx_input.call_type, call);
    let mut topics = Vec::new();
    push_transfer_bytes(&call.raw_kda_transfers, &mut topics);
    topics.push(call.destination.to_vec());

    let mut data = vec![
        call_type.to_log_bytes(),
        KLEVER_TRANSFER_FUNC_NAME.into(),
        call.destination.to_vec(),
        top_encode_u64(call.raw_kda_transfers.len() as u64),
    ];
    push_transfer_bytes(&call.raw_kda_transfers, &mut data);
    push_func_name_if_necessary(call_type, &call.func_name, &mut data);

    TxLog {
        address: tx_input.from.clone(),
        endpoint: KLEVER_TRANSFER_FUNC_NAME.into(),
        topics,
        data,
    }
}

fn try_parse_input(tx_input: &TxInput) -> Result<ParsedTransferBuiltinFunCall, &'static str> {
    if tx_input.args.len() < 2 {
        return Err("MultiKDANFTTransfer too few arguments");
    }

    let mut arg_index = 0;
    let destination_bytes = tx_input.args[arg_index].as_slice();
    let destination = VMAddress::from_slice(destination_bytes);
    arg_index += 1;
    let num_payments = top_decode_u64(tx_input.args[arg_index].as_slice()) as usize;
    arg_index += 1;

    if tx_input.args.len() < 2 + num_payments * 3 {
        return Err("MultiKDANFTTransfer too few arguments");
    }

    let mut raw_kda_transfers = Vec::new();
    for _ in 0..num_payments {
        let token_identifier = tx_input.args[arg_index].clone();
        arg_index += 1;
        let nonce_bytes = tx_input.args[arg_index].clone();
        arg_index += 1;
        let value_bytes = tx_input.args[arg_index].clone();
        arg_index += 1;

        raw_kda_transfers.push(RawKdaTransfer {
            token_identifier: token_identifier.clone(),
            nonce_bytes,
            value_bytes,
        });
    }

    let func_name = tx_input.func_name_from_arg_index(arg_index);
    arg_index += 1;
    let args = if tx_input.args.len() > arg_index {
        tx_input.args[arg_index..].to_vec()
    } else {
        Vec::new()
    };

    Ok(ParsedTransferBuiltinFunCall {
        destination,
        raw_kda_transfers,
        func_name,
        args,
    })
}
