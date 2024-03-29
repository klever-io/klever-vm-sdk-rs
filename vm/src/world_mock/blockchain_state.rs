use std::{collections::HashMap, fmt::Debug};

use num_bigint::BigUint;
use num_traits::Zero;

use crate::{tx_mock::BlockchainUpdate, types::VMAddress};

use super::{AccountData, BlockInfo};

#[derive(Default, Clone)]
pub struct BlockchainState {
    pub accounts: HashMap<VMAddress, AccountData>,
    pub new_addresses: HashMap<(VMAddress, u64), VMAddress>,
    pub previous_block_info: BlockInfo,
    pub current_block_info: BlockInfo,
    pub new_token_identifiers: Vec<String>,
}

impl BlockchainState {
    pub fn commit_updates(&mut self, updates: BlockchainUpdate) {
        updates.apply(self);
    }

    pub fn account_exists(&self, address: &VMAddress) -> bool {
        self.accounts.contains_key(address)
    }

    pub fn increase_account_nonce(&mut self, address: &VMAddress) {
        let account = self.accounts.get_mut(address).unwrap_or_else(|| {
            panic!(
                "Account not found: {}",
                &std::str::from_utf8(address.as_ref()).unwrap()
            )
        });
        account.nonce += 1;
    }

    pub fn subtract_tx_gas(&mut self, address: &VMAddress, gas_limit: u64, gas_price: u64) {
        let account = self.accounts.get_mut(address).unwrap_or_else(|| {
            panic!(
                "Account not found: {}",
                &std::str::from_utf8(address.as_ref()).unwrap()
            )
        });
        let gas_cost = BigUint::from(gas_limit) * BigUint::from(gas_price);
        assert!(
            account.klv_balance >= gas_cost,
            "Not enough balance to pay gas upfront"
        );
        account.klv_balance -= &gas_cost;
    }

    pub fn put_new_token_identifier(&mut self, token_identifier: String) {
        self.new_token_identifiers.push(token_identifier)
    }

    pub fn get_new_token_identifiers(&self) -> Vec<String> {
        self.new_token_identifiers.clone()
    }

    pub fn update_new_token_identifiers(&mut self, token_identifiers: Vec<String>) {
        self.new_token_identifiers = token_identifiers;
    }
}

impl Debug for BlockchainState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BlockchainState")
            .field("accounts", &self.accounts)
            .field("new_addresses", &self.new_addresses)
            .field("current_block_info", &self.current_block_info)
            .finish()
    }
}
