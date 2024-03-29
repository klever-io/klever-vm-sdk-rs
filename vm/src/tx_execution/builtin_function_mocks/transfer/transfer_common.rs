use crate::{
    tx_execution::{
        builtin_function_mocks::builtin_func_trait::BuiltinFunctionKdaTransferInfo,
        BlockchainVMRef,
    },
    tx_mock::{
        BlockchainUpdate, TxCache, TxFunctionName, TxInput, TxLog, TxResult, TxTokenTransfer,
    },
    types::{top_decode_u64, VMAddress},
};
use num_bigint::BigUint;
use num_traits::Zero;

pub(super) struct ParsedTransferBuiltinFunCall {
    pub destination: VMAddress,
    pub raw_kda_transfers: Vec<RawKdaTransfer>,
    pub func_name: TxFunctionName,
    pub args: Vec<Vec<u8>>,
}

pub(super) struct RawKdaTransfer {
    pub token_identifier: Vec<u8>,
    pub nonce_bytes: Vec<u8>,
    pub value_bytes: Vec<u8>,
}

pub(super) fn process_raw_kda_transfer(raw_kda_transfer: RawKdaTransfer) -> TxTokenTransfer {
    TxTokenTransfer {
        token_identifier: raw_kda_transfer.token_identifier,
        nonce: top_decode_u64(raw_kda_transfer.nonce_bytes.as_slice()),
        value: BigUint::from_bytes_be(raw_kda_transfer.value_bytes.as_slice()),
    }
}

fn process_raw_kda_transfers(raw_kda_transfers: Vec<RawKdaTransfer>) -> Vec<TxTokenTransfer> {
    raw_kda_transfers
        .into_iter()
        .map(process_raw_kda_transfer)
        .collect()
}

pub(super) fn extract_transfer_info(
    parsed_tx: ParsedTransferBuiltinFunCall,
) -> BuiltinFunctionKdaTransferInfo {
    BuiltinFunctionKdaTransferInfo {
        real_recipient: parsed_tx.destination,
        transfers: process_raw_kda_transfers(parsed_tx.raw_kda_transfers),
    }
}

pub(super) fn execute_transfer_builtin_func<F>(
    vm: &BlockchainVMRef,
    parsed_tx: ParsedTransferBuiltinFunCall,
    builtin_function_name: &str,
    tx_input: TxInput,
    tx_cache: TxCache,
    f: F,
) -> (TxResult, BlockchainUpdate)
where
    F: FnOnce(),
{
    let mut builtin_logs = Vec::new();
    for raw_kda_transfer in &parsed_tx.raw_kda_transfers {
        builtin_logs.push(TxLog {
            address: tx_input.from.clone(),
            endpoint: builtin_function_name.into(),
            topics: vec![
                raw_kda_transfer.token_identifier.clone(),
                raw_kda_transfer.nonce_bytes.clone(),
                raw_kda_transfer.value_bytes.clone(),
                parsed_tx.destination.to_vec(),
            ],
            data: vec![],
        });
    }

    let exec_input = TxInput {
        from: tx_input.from,
        to: parsed_tx.destination,
        klv_value: BigUint::zero(),
        kda_values: process_raw_kda_transfers(parsed_tx.raw_kda_transfers),
        func_name: parsed_tx.func_name,
        args: parsed_tx.args,
        gas_limit: tx_input.gas_limit,
        gas_price: tx_input.gas_price,
        tx_hash: tx_input.tx_hash,
        ..Default::default()
    };

    let (mut tx_result, blockchain_updates) = vm.default_execution(exec_input, tx_cache, f);

    // prepends kda log
    tx_result.result_logs = [builtin_logs.as_slice(), tx_result.result_logs.as_slice()].concat();

    (tx_result, blockchain_updates)
}
