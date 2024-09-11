use num_bigint::BigUint;

use crate::{
    tx_execution::is_system_sc_address, tx_mock::TxPanic, types::VMAddress,
    world_mock::KdaInstanceMetadata,
};

use super::TxCache;

impl TxCache {
    pub fn subtract_klv_balance(
        &self,
        address: &VMAddress,
        call_value: &BigUint,
    ) -> Result<(), TxPanic> {
        self.with_account_mut(address, |account| {
            if call_value > &account.klv_balance {
                return Err(TxPanic::vm_error("failed transfer (insufficient funds)"));
            }
            account.klv_balance -= call_value;
            Ok(())
        })
    }

    pub fn subtract_tx_gas(&self, address: &VMAddress, gas_limit: u64, gas_price: u64) {
        self.with_account_mut(address, |account| {
            let gas_cost = BigUint::from(gas_limit) * BigUint::from(gas_price);
            assert!(
                account.klv_balance >= gas_cost,
                "Not enough balance to pay gas upfront"
            );
            account.klv_balance -= &gas_cost;
        });
    }

    pub fn increase_klv_balance(&self, address: &VMAddress, amount: &BigUint) {
        self.with_account_mut(address, |account| {
            account.klv_balance += amount;
        });
    }

    pub fn subtract_kda_balance(
        &self,
        address: &VMAddress,
        kda_token_identifier: &[u8],
        nonce: u64,
        value: &BigUint,
    ) -> Result<KdaInstanceMetadata, TxPanic> {
        self.with_account_mut(address, |account| {
            let kda_data_map = &mut account.kda;
            let kda_data = kda_data_map
                .get_mut_by_identifier(kda_token_identifier)
                .ok_or_else(err_insufficient_funds)?;

            let kda_instances = &mut kda_data.instances;
            let kda_instance = kda_instances
                .get_mut_by_nonce(nonce)
                .ok_or_else(err_insufficient_funds)?;

            let kda_balance = &mut kda_instance.balance;
            if &*kda_balance < value {
                return Err(err_insufficient_funds());
            }

            *kda_balance -= value;

            Ok(kda_instance.metadata.clone())
        })
    }

    pub fn increase_kda_balance(
        &self,
        address: &VMAddress,
        kda_token_identifier: &[u8],
        nonce: u64,
        value: &BigUint,
        kda_metadata: KdaInstanceMetadata,
    ) {
        self.with_account_mut(address, |account| {
            account
                .kda
                .increase_balance(kda_token_identifier.to_vec(), nonce, value, kda_metadata);
        });
    }

    pub fn transfer_klv_balance(
        &self,
        from: &VMAddress,
        to: &VMAddress,
        value: &BigUint,
    ) -> Result<(), TxPanic> {
        if !is_system_sc_address(from) {
            self.subtract_klv_balance(from, value)?;
        }
        if !is_system_sc_address(to) {
            self.increase_klv_balance(to, value);
        }
        Ok(())
    }

    pub fn transfer_kda_balance(
        &self,
        from: &VMAddress,
        to: &VMAddress,
        kda_token_identifier: &[u8],
        nonce: u64,
        value: &BigUint,
    ) -> Result<(), TxPanic> {
        if !is_system_sc_address(from) && !is_system_sc_address(to) {
            let metadata = self.subtract_kda_balance(from, kda_token_identifier, nonce, value)?;
            self.increase_kda_balance(to, kda_token_identifier, nonce, value, metadata);
        }
        Ok(())
    }
}

fn err_insufficient_funds() -> TxPanic {
    TxPanic::vm_error("insufficient funds")
}
