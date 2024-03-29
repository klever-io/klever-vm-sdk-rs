
use crate::tx_execution::{builtin_function_names::KLEVER_ASSET_TRIGGER_FUNC_NAME, BlockchainVMRef};

use crate::tx_mock::{BlockchainUpdate, TxCache, TxInput, TxLog, TxResult};
use crate::types::VMAddress;
use crate::world_mock::KdaInstanceMetadata;

use num_bigint::BigUint;
use super::super::builtin_func_trait::BuiltinFunction;

pub struct KleverAssetTrigger;

impl BuiltinFunction for KleverAssetTrigger {
    fn name(&self) -> &str {
        KLEVER_ASSET_TRIGGER_FUNC_NAME
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
        let mut trigger_type = 0;
        if !tx_input.args[0].as_slice().is_empty() {
            trigger_type = u32::from_be_bytes(<[u8; 4]>::try_from(tx_input.args[0].as_slice()).unwrap());
        }

            match trigger_type {
            0 => {
                let token_identifier = tx_input.args[1].clone();

                let mut nonce = 0;
                if !tx_input.args[2].as_slice().is_empty() {
                    nonce = u64::from_be_bytes(<[u8; 8]>::try_from(tx_input.args[2].as_slice()).unwrap());
                }
                
                let amount = BigUint::from_bytes_be(tx_input.args[3].as_slice());
                let address = VMAddress::from(<[u8; 32]>::try_from(tx_input.args[4].as_slice()).unwrap());

                tx_cache.increase_kda_balance(
                    &address,
                    &token_identifier,
                    nonce,
                    &amount,
                    KdaInstanceMetadata::default(),
                );
            },
            _ => {
                panic!("Invalid trigger type");
            }
        }

        (TxResult::empty(), tx_cache.into_blockchain_updates())
    }
}
