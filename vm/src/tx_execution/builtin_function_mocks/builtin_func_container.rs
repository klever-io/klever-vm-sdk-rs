use super::{
    builtin_func_trait::BuiltinFunction,
    builtin_function_names::*,
    general::{ChangeOwner, DeleteUsername, SetUsername, UpgradeContract, KleverAssetTrigger},
    transfer::KDAMultiTransfer,
    BuiltinFunctionKdaTransferInfo,
};
use crate::{
    tx_execution::BlockchainVMRef,
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult}
};

/// Container for builtin function logic.
///
/// Currently has no data, but could conceivably be configurable in the future.
pub struct BuiltinFunctionContainer;

impl BuiltinFunctionContainer {
    /// If the call points to a builtin function, it executes it, otherwise calls the `or_else` closure.
    ///
    /// It also checks that the appropriate roles are set, where applicable.
    pub fn execute_builtin_function_or_else<F, Else>(
        &self,
        vm: &BlockchainVMRef,
        tx_input: TxInput,
        tx_cache: TxCache,
        f: F,
        or_else: Else,
    ) -> (TxResult, BlockchainUpdate)
    where
        F: FnOnce(),
        Else: FnOnce(TxInput, TxCache, F) -> (TxResult, BlockchainUpdate),
    {
        BuiltinFunctionCall::new(vm, tx_input, tx_cache).execute_or_else(f, or_else)
    }

    /// Provides data on the builtin functions that perform KDA token transfers.
    pub fn extract_token_transfers(&self, tx_input: &TxInput) -> BuiltinFunctionKdaTransferInfo {
        match tx_input.func_name.as_str() {
            KLEVER_TRANSFER_FUNC_NAME => bf_extract_transfers(KDAMultiTransfer, tx_input),
            _ => BuiltinFunctionKdaTransferInfo::empty(tx_input),
        }
    }
}

fn bf_extract_transfers<B>(builtin_func: B, tx_input: &TxInput) -> BuiltinFunctionKdaTransferInfo
where
    B: BuiltinFunction,
{
    builtin_func.extract_kda_transfers(tx_input)
}

/// Syntax helper for the big builtin function match in `execute_or_else`.
/// Thanks to it we do not need to write out the arguments for each match arm.
struct BuiltinFunctionCall<'a> {
    vm: &'a BlockchainVMRef,
    tx_input: TxInput,
    tx_cache: TxCache,
}

impl<'a> BuiltinFunctionCall<'a> {
    pub fn new(vm: &'a BlockchainVMRef, tx_input: TxInput, tx_cache: TxCache) -> Self {
        BuiltinFunctionCall {
            vm,
            tx_input,
            tx_cache,
        }
    }

    pub fn execute_or_else<F, Else>(self, f: F, or_else: Else) -> (TxResult, BlockchainUpdate)
    where
        F: FnOnce(),
        Else: FnOnce(TxInput, TxCache, F) -> (TxResult, BlockchainUpdate),
    {
        match self.tx_input.func_name.as_str() {
            KLEVER_ASSET_TRIGGER_FUNC_NAME => self.execute_bf(KleverAssetTrigger, f),
            KLEVER_TRANSFER_FUNC_NAME => self.execute_bf(KDAMultiTransfer, f),
            CHANGE_OWNER_BUILTIN_FUNC_NAME => self.execute_bf(ChangeOwner, f),
            SET_USERNAME_FUNC_NAME => self.execute_bf(SetUsername, f),
            DELETE_USERNAME_FUNC_NAME => self.execute_bf(DeleteUsername, f),
            UPGRADE_CONTRACT_FUNC_NAME => self.execute_bf(UpgradeContract, f),
            MIGRATE_USERNAME_FUNC_NAME => {
                panic!("builtin function {MIGRATE_USERNAME_FUNC_NAME} was dropped")
            },
            _ => or_else(self.tx_input, self.tx_cache, f),
        }
    }

    fn execute_bf<B, F>(self, builtin_func: B, f: F) -> (TxResult, BlockchainUpdate)
    where
        B: BuiltinFunction,
        F: FnOnce(),
    {
        builtin_func.execute(self.tx_input, self.tx_cache, self.vm, f)
    }
}
