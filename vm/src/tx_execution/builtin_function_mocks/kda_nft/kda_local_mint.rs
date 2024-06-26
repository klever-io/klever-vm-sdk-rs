use num_bigint::BigUint;

use crate::tx_execution::{builtin_function_names::KDA_LOCAL_MINT_FUNC_NAME, BlockchainVMRef};

use crate::{
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxLog, TxResult},
    world_mock::KdaInstanceMetadata,
};

use super::super::builtin_func_trait::BuiltinFunction;

pub struct KDALocalMint;

impl BuiltinFunction for KDALocalMint {
    fn name(&self) -> &str {
        KDA_LOCAL_MINT_FUNC_NAME
    }

    fn execute<F>(
        &self,
        tx_input: TxInput,
        tx_cache: TxCache,
        _vm: &BlockchainVMRef,
        _f: F,
    ) -> (TxResult, BlockchainUpdate)
    where
        F: FnOnce(),
    {
        if tx_input.args.len() != 2 {
            let err_result = TxResult::from_vm_error("KDALocalMint expects 2 arguments");
            return (err_result, BlockchainUpdate::empty());
        }

        let token_identifier = tx_input.args[0].clone();
        let value = BigUint::from_bytes_be(tx_input.args[1].as_slice());

        tx_cache.increase_kda_balance(
            &tx_input.to,
            &token_identifier,
            0,
            &value,
            KdaInstanceMetadata::default(),
        );

        let kda_nft_create_log = TxLog {
            address: tx_input.from,
            endpoint: KDA_LOCAL_MINT_FUNC_NAME.into(),
            topics: vec![token_identifier.to_vec(), Vec::new(), value.to_bytes_be()],
            data: vec![],
        };

        let tx_result = TxResult {
            result_status: 0,
            result_logs: vec![kda_nft_create_log],
            ..Default::default()
        };

        (tx_result, tx_cache.into_blockchain_updates())
    }
}
