use crate::tx_execution::{
    builtin_function_names::KLEVER_ASSET_TRIGGER_FUNC_NAME, BlockchainVMRef,
};

use crate::tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult};
use crate::types::{top_decode_u64, top_encode_u64, VMAddress};
use crate::world_mock::{KdaInstance, KdaInstanceMetadata};

use super::super::builtin_func_trait::BuiltinFunction;
use num_bigint::BigUint;

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
        let mut trigger_type = 0u8;
        if !tx_input.args[0].as_slice().is_empty() {
            trigger_type = tx_input.args[0].as_slice()[0];
        }

        let mut tx_result = TxResult {
            result_status: 0,
            ..Default::default()
        };

        match trigger_type {
            // Mint
            0 => {
                let token_identifier = tx_input.args[1].clone();
                let nonce = top_decode_u64(tx_input.args[2].as_slice());

                let amount = BigUint::from_bytes_be(tx_input.args[3].as_slice());
                let address =
                    VMAddress::from(<[u8; 32]>::try_from(tx_input.args[4].as_slice()).unwrap());

                // if amount is zero or 1, simulate as an NFT, unless nonce is > 0 (SFT)
                if nonce == 0 && amount.cmp(&BigUint::from(1u32)) <= std::cmp::Ordering::Equal {
                    let new_nonce: u64 = tx_cache.with_account_mut(&address, |account| {
                        let kda_data = account
                            .kda
                            .get_mut_by_identifier(&token_identifier)
                            .unwrap_or_else(|| {
                                panic!(
                                    "KDACreate unexpected token identifier {:?}",
                                    token_identifier
                                )
                            });

                        // get last royalty if any previous instance
                        let mut royalties = 0;
                        if !kda_data.instances.is_empty() {
                            royalties = kda_data
                                .instances
                                .get_by_nonce(kda_data.last_nonce)
                                .unwrap()
                                .metadata
                                .royalties;
                        }

                        kda_data.last_nonce += 1;
                        kda_data.instances.push_instance(KdaInstance {
                            balance: amount,
                            nonce: kda_data.last_nonce,
                            metadata: KdaInstanceMetadata {
                                name: vec![],
                                creator: Some(tx_input.from.clone()),
                                royalties,
                                hash: None,
                                uri: vec![],
                                attributes: vec![],
                                can_burn: true,
                            },
                        });

                        kda_data.last_nonce
                    });

                    tx_result.result_values = vec![top_encode_u64(new_nonce)];
                } else {
                    tx_cache.increase_kda_balance(
                        &address,
                        &token_identifier,
                        nonce,
                        &amount,
                        KdaInstanceMetadata::default(),
                    );
                };
            },
            // Burn
            1 => {
                let token_identifier = tx_input.args[1].clone();
                let nonce = top_decode_u64(tx_input.args[2].as_slice());
                let amount = BigUint::from_bytes_be(tx_input.args[3].as_slice());

                let _ = tx_cache.subtract_kda_balance(
                    &tx_input.from,
                    &token_identifier,
                    nonce,
                    &amount,
                );
            },
            // Update Metadata
            8 => {
                let token_identifier = tx_input.args[1].as_slice();
                let nonce = top_decode_u64(tx_input.args[2].as_slice());
                let address =
                    VMAddress::from(<[u8; 32]>::try_from(tx_input.args[3].as_slice()).unwrap());
                // let mime = tx_input.args[4].as_slice();
                let metadata = tx_input.args[5].as_slice();
                // let name = tx_input.args[6].as_slice();

                let new_attribute_bytes = metadata.to_vec();

                tx_cache.with_account_mut(&address, |account| {
                    account
                        .kda
                        .update_attributes(token_identifier, nonce, new_attribute_bytes);
                });
            },
            _ => {
                panic!("Invalid trigger type");
            },
        }

        (tx_result, tx_cache.into_blockchain_updates())
    }
}
