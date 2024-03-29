use num_bigint::BigUint;

use crate::{
    tx_execution::{builtin_function_names::KDA_NFT_CREATE_FUNC_NAME, BlockchainVMRef},
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxLog, TxResult},
    types::{top_decode_u64, top_encode_u64},
    world_mock::{KdaInstance, KdaInstanceMetadata},
};

use super::super::builtin_func_trait::BuiltinFunction;

pub struct KDANftCreate;

impl BuiltinFunction for KDANftCreate {
    fn name(&self) -> &str {
        KDA_NFT_CREATE_FUNC_NAME
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
        if tx_input.args.len() < 7 {
            let err_result = TxResult::from_vm_error("KDANFTCreate too few arguments");
            return (err_result, BlockchainUpdate::empty());
        }
        assert!(
            tx_input.to == tx_input.from,
            "KDANFTCreate expects that to == from"
        );

        let token_identifier = tx_input.args[0].as_slice();
        let amount = BigUint::from_bytes_be(tx_input.args[1].as_slice());
        let name = tx_input.args[2].clone();
        let royalties = top_decode_u64(tx_input.args[3].as_slice());
        let hash = tx_input.args[4].clone();
        let attributes = tx_input.args[5].clone();
        let uris = tx_input.args[6..].to_vec();
        let can_burn = true;

        let new_nonce = tx_cache.with_account_mut(&tx_input.to, |account| {
            let kda_data = account
                .kda
                .get_mut_by_identifier(token_identifier)
                .unwrap_or_else(|| panic!("KDANFTCreate unexpected token identifier"));
            kda_data.last_nonce += 1;
            kda_data.instances.push_instance(KdaInstance {
                nonce: kda_data.last_nonce,
                balance: amount.clone(),
                metadata: KdaInstanceMetadata {
                    name,
                    creator: Some(tx_input.from.clone()),
                    royalties,
                    hash: Some(hash),
                    uri: uris,
                    attributes,
                    can_burn,
                },
            });

            kda_data.last_nonce
        });

        let kda_nft_create_log = TxLog {
            address: tx_input.from.clone(),
            endpoint: KDA_NFT_CREATE_FUNC_NAME.into(),
            topics: vec![
                token_identifier.to_vec(),
                top_encode_u64(new_nonce),
                amount.to_bytes_be(),
                Vec::new(), // in actuality this should contain the fully marshalled KDA data
            ],
            data: vec![],
        };

        let tx_result = TxResult {
            result_status: 0,
            result_values: vec![top_encode_u64(new_nonce)],
            result_logs: vec![kda_nft_create_log],
            ..Default::default()
        };

        (tx_result, tx_cache.into_blockchain_updates())
    }
}
