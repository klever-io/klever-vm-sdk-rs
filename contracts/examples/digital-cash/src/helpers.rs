use klever_sc::imports::*;

use crate::{
    constants::*,
    deposit_info::{DepositInfo, Fee},
    storage,
};
#[klever_sc::module]
pub trait HelpersModule: storage::StorageModule {
    fn send_fee_to_address(&self, fee: &KdaTokenPayment, address: &ManagedAddress) {
        if fee.token_identifier == TokenIdentifier::klv() {
            self.send().direct_klv(address, &fee.amount);
        } else {
            let kda_fee = fee.clone();
            self.send()
                .direct_kda(address, &kda_fee.token_identifier, 0, &kda_fee.amount);
        }
    }

    fn get_num_token_transfers(
        &self,
        klv_value: &BigUint,
        kda_transfers: &ManagedVec<KdaTokenPayment>,
    ) -> usize {
        let mut amount = kda_transfers.len();
        if klv_value > &0 {
            amount += 1;
        }

        amount
    }

    fn get_expiration_round(&self, valability: u64) -> u64 {
        let valability_rounds = valability / SECONDS_PER_ROUND;
        self.blockchain().get_block_round() + valability_rounds
    }

    fn get_fee_for_token(&self, token: &TokenIdentifier) -> BigUint {
        require!(
            self.whitelisted_fee_tokens().contains(token),
            "invalid fee toke provided"
        );
        let fee_token = self.fee(token);
        fee_token.get()
    }

    fn make_fund(
        &self,
        klv_payment: BigUint,
        kda_payment: ManagedVec<KdaTokenPayment>,
        address: ManagedAddress,
        valability: u64,
    ) {
        let deposit_mapper = self.deposit(&address);

        deposit_mapper.update(|deposit| {
            require!(
                deposit.klv_funds == 0 && deposit.kda_funds.is_empty(),
                "key already used"
            );
            let num_tokens = self.get_num_token_transfers(&klv_payment, &kda_payment);
            deposit.fees.num_token_to_transfer += num_tokens;
            deposit.valability = valability;
            deposit.expiration_round = self.get_expiration_round(valability);
            deposit.kda_funds = kda_payment;
            deposit.klv_funds = klv_payment;
        });
    }

    fn check_fees_cover_number_of_tokens(
        &self,
        num_tokens: usize,
        fee: BigUint,
        paid_fee: BigUint,
    ) {
        require!(num_tokens > 0, "amount must be greater than 0");
        require!(
            fee * num_tokens as u64 <= paid_fee,
            CANNOT_DEPOSIT_FUNDS_ERR_MSG
        );
    }

    fn update_fees(
        &self,
        caller_address: ManagedAddress,
        address: &ManagedAddress,
        payment: KdaTokenPayment,
    ) {
        self.get_fee_for_token(&payment.token_identifier);
        let deposit_mapper = self.deposit(address);
        if !deposit_mapper.is_empty() {
            deposit_mapper.update(|deposit| {
                require!(
                    deposit.depositor_address == caller_address,
                    "invalid depositor address"
                );
                require!(
                    deposit.fees.value.token_identifier == payment.token_identifier,
                    "can only have 1 type of token as fee"
                );
                deposit.fees.value.amount += payment.amount;
            });
            return;
        }

        let new_deposit = DepositInfo {
            depositor_address: caller_address,
            kda_funds: ManagedVec::new(),
            klv_funds: BigUint::zero(),
            valability: 0,
            expiration_round: 0,
            fees: Fee {
                num_token_to_transfer: 0,
                value: payment,
            },
        };
        deposit_mapper.set(new_deposit);
    }
}
