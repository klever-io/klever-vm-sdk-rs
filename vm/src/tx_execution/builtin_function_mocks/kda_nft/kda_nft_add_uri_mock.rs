use crate::{
    tx_execution::{builtin_function_names::KDA_NFT_ADD_URI_FUNC_NAME, BlockchainVMRef},
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxLog, TxResult},
    types::{top_decode_u64, top_encode_u64},
};

use super::super::builtin_func_trait::BuiltinFunction;

pub struct KDANftAddUri;

impl BuiltinFunction for KDANftAddUri {
    fn name(&self) -> &str {
        KDA_NFT_ADD_URI_FUNC_NAME
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
        if tx_input.args.len() < 3 {
            let err_result = TxResult::from_vm_error("KDANFTAddURI expects at least 3 arguments");
            return (err_result, BlockchainUpdate::empty());
        }

        let token_identifier = tx_input.args[0].clone();
        let nonce = top_decode_u64(tx_input.args[1].as_slice());
        let mut new_uris = tx_input.args[2..].to_vec();

        tx_cache.with_account_mut(&tx_input.from, |account| {
            account
                .kda
                .add_uris(token_identifier.as_slice(), nonce, new_uris.clone());
        });

        let mut topics = vec![
            token_identifier.to_vec(),
            top_encode_u64(nonce),
            Vec::new(), // value = 0
        ];
        topics.append(&mut new_uris);
        let kda_nft_create_log = TxLog {
            address: tx_input.from,
            endpoint: KDA_NFT_ADD_URI_FUNC_NAME.into(),
            topics,
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
