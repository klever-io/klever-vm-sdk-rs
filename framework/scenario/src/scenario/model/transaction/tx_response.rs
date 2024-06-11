use klever_chain_vm::crypto_functions::keccak256;
use crate::klever_sc::types::Address;
use klever_chain_vm::tx_mock::TxResult;
use klever_vm_sdk::data::transaction::{
    ApiLogs, ApiSmartContractResult, Events,
};
use klever_vm_sdk::utils::base64_decode;

use super::{
    decode_scr_data_or_panic, is_out_scr, process_topics_error, Log, TxExpect, TxResponseStatus,
};

const LOG_IDENTIFIER_SIGNAL_ERROR: &str = "signalError";

const SYSTEM_SC_BECH32: &str = "klv1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u";

#[derive(Debug, Default, Clone)]
/// The response of a transaction.
pub struct TxResponse {
    /// The output of the transaction.
    pub out: Vec<Vec<u8>>,
    /// The address of the newly deployed smart contract.
    pub new_deployed_address: Option<Address>,
    /// The identifier of the newly issued token.
    pub new_issued_token_identifier: Option<String>,
    /// The status of the transaction.
    pub tx_error: TxResponseStatus,
    /// The logs of the transaction.
    pub logs: Vec<Log>,
    /// The gas used by the transaction.
    pub gas: u64,
    /// The refund of the transaction.
    pub refund: u64,
    /// The smart contract results of the transaction.
    pub api_scrs: Vec<ApiSmartContractResult>,
    /// The api logs of the transaction.
    pub api_logs: Option<ApiLogs>,
}

impl TxResponse {
    /// Creates a [`TxResponse`] from a [`TxResult`].
    pub fn from_tx_result(tx_result: TxResult) -> Self {
        TxResponse {
            out: tx_result.result_values,
            tx_error: TxResponseStatus {
                status: tx_result.result_status,
                message: tx_result.result_message,
            },
            ..Default::default()
        }
    }

    /// Creates a [`TxResponse`] from raw results.
    pub fn from_raw_results(raw_results: Vec<Vec<u8>>) -> Self {
        TxResponse {
            out: raw_results,
            ..Default::default()
        }
    }

    /// Creates a scenario "expect" field based on the real response.
    ///
    /// Useful for creating traces that also check the results come out always the same.
    pub fn to_expect(&self) -> TxExpect {
        if self.tx_error.is_success() {
            let mut tx_expect = TxExpect::ok();
            if self.out.is_empty() {
                tx_expect = tx_expect.no_result();
            } else {
                for raw_result in &self.out {
                    let result_hex_string = format!("0x{}", hex::encode(raw_result));
                    tx_expect = tx_expect.result(result_hex_string.as_str());
                }
            }
            tx_expect
        } else {
            TxExpect::err(
                self.tx_error.status,
                format!("str:{}", self.tx_error.message),
            )
        }
    }

    /// Checks if the transaction was successful.
    pub fn is_success(&self) -> bool {
        self.tx_error.is_success()
    }

    fn process_signal_error(&self) -> TxResponseStatus {
        if let Some(event) = self.find_log(LOG_IDENTIFIER_SIGNAL_ERROR) {
            let topics = event.topics.as_ref();
            if let Some(error) = process_topics_error(topics) {
                return TxResponseStatus::signal_error(&error);
            }

            let error_raw = base64_decode(topics.unwrap().get(1).unwrap());
            let error = String::from_utf8(error_raw).unwrap();
            return TxResponseStatus::signal_error(&error);
        }

        TxResponseStatus::default()
    }

    fn process(
        self,
        sender_address: [u8; 32],
        nonce: u64
    ) -> Self {
        self.process_out()
            .process_new_deployed_address(sender_address, nonce)
            .process_new_issued_token_identifier()
    }

    fn process_out(mut self) -> Self {
        let out_scr = self.api_scrs.iter().find(is_out_scr);

        if let Some(out_scr) = out_scr {
            self.out = decode_scr_data_or_panic(&out_scr.data);
        }

        self
    }

    fn process_new_deployed_address(
        mut self,
        sender_address_bytes: [u8; 32],
        nonce: u64
    ) -> Self {
        let sender_nonce_bytes = nonce.to_le_bytes();
        let mut bytes_to_hash: Vec<u8> = Vec::new();
        bytes_to_hash.extend_from_slice(&sender_address_bytes);
        bytes_to_hash.extend_from_slice(&sender_nonce_bytes);

        let address_keccak = keccak256(&bytes_to_hash);

        let mut address = [0u8; 32];

        address[0..8].copy_from_slice(&[0u8; 8]);
        address[8..10].copy_from_slice(&[5, 0]);
        address[10..30].copy_from_slice(&address_keccak[10..30]);
        address[30..32].copy_from_slice(&sender_address_bytes[30..32]);

        self.new_deployed_address = Some(Address::from(address));

        self
    }

    fn process_new_issued_token_identifier(mut self) -> Self {
        for scr in self.api_scrs.iter() {
            if scr.sender.to_string() != SYSTEM_SC_BECH32 {
                continue
            }

            let Some(prev_tx) = self.api_scrs.iter().find(|e| e.hash == scr.prev_tx_hash) else {
                continue
            };

            // check if the previous transaction is a token issue, if not, continue
            match prev_tx.data.split('@').next().unwrap_or("") {
                "issue" => "Fungible",
                "issueSemiFungible" => "SemiFungible",
                "issueNonFungible" => "NonFungible",
                "registerMetaKDA" => "MetaKDA",
                _ => continue,
            };

            if scr.data.starts_with("KDATransfer@") {
                let encoded_tid = scr.data.split('@').nth(1);
                if encoded_tid.is_none() {
                    return self;
                }


                break;
            } else if scr.data.starts_with("@00@") {
                let encoded_tid = scr.data.split('@').nth(2);
                if encoded_tid.is_none() {
                    return self;
                }

                self.new_issued_token_identifier = Some(String::from_utf8(hex::decode(encoded_tid.unwrap()).unwrap()).unwrap());

                break;
            }
        }

        self
    }

    fn find_log(&self, log_identifier: &str) -> Option<&Events> {
        if let Some(logs) = &self.api_logs {
            logs.events
                .iter()
                .find(|event| event.identifier == log_identifier)
        } else {
            None
        }
    }
}
