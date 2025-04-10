use crate::tx_execution::{
    builtin_function_names::KLEVER_SET_ACCOUNT_NAME_FUNC_NAME, BlockchainVMRef,
};

use crate::tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult};

use super::super::builtin_func_trait::BuiltinFunction;

pub struct SetAccountName;

impl BuiltinFunction for SetAccountName {
    fn name(&self) -> &str {
        KLEVER_SET_ACCOUNT_NAME_FUNC_NAME
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
        if tx_input.args.len() != 1 {
            return (
                TxResult::from_vm_error("SetAccountName expects 1 argument"),
                BlockchainUpdate::empty(),
            );
        }

        let username = tx_input.args[0].clone();
        let success = tx_cache.with_account_mut(&tx_input.to, |account| {
            if account.username.is_empty() {
                account.username = username;
                true
            } else {
                false
            }
        });

        if success {
            (TxResult::empty(), tx_cache.into_blockchain_updates())
        } else {
            (
                TxResult::from_vm_error("SetAccountName expects 1 argument"),
                BlockchainUpdate::empty(),
            )
        }
    }
}
