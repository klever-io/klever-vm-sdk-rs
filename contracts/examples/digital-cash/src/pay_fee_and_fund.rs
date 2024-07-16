use klever_sc::imports::*;

use crate::{constants::*, helpers, storage};

#[klever_sc::module]
pub trait PayFeeAndFund: storage::StorageModule + helpers::HelpersModule {
    fn call_value_no_klv(&self) -> ManagedVec<KdaTokenPayment> {   
        let mut payments: ManagedVec<KdaTokenPayment> = ManagedVec::new();
        self.call_value().all_kda_transfers().iter().for_each(|payment| {
            if payment.token_identifier != TokenIdentifier::klv() {
                payments.push(payment.clone());
            }
        });

        payments
    }
    #[endpoint(payFeeAndFundKDA)]
    #[payable("*")]
    fn pay_fee_and_fund_kda(&self, address: ManagedAddress, valability: u64) {
        let mut payments = self.call_value_no_klv();
        let fee = KdaTokenPayment::from(payments.get(0));
        let caller_address = self.blockchain().get_caller();
        self.update_fees(caller_address, &address, fee);

        payments.remove(0);

        self.make_fund(0u64.into(), payments, address, valability)
    }
    #[endpoint(payFeeAndFundKLV)]
    #[payable("KLV")]
    fn pay_fee_and_fund_klv(&self, address: ManagedAddress, valability: u64) {
        let mut fund = self.call_value().klv_value().clone_value();
        let fee_value = self.fee(&TokenIdentifier::klv()).get();
        require!(fund > fee_value, "payment not covering fees");

        fund -= fee_value.clone();
        let fee = KdaTokenPayment::new(TokenIdentifier::klv(), 0, fee_value);
        let caller_address = self.blockchain().get_caller();
        self.update_fees(caller_address, &address, fee);

        self.make_fund(fund, ManagedVec::new(), address, valability);
    }

    #[endpoint]
    #[payable("*")]
    fn fund(&self, address: ManagedAddress, valability: u64) {
        require!(!self.deposit(&address).is_empty(), FEES_NOT_COVERED_ERR_MSG);
        let deposit_mapper = self.deposit(&address).get();
        let depositor = deposit_mapper.depositor_address;
        require!(
            self.blockchain().get_caller() == depositor,
            "invalid depositor"
        );
        let deposited_fee_token = deposit_mapper.fees.value;
        let fee_amount = self.fee(&deposited_fee_token.token_identifier).get();
        let klv_payment = self.call_value().klv_value().clone_value();
        let kda_payment = self.call_value_no_klv();

        let num_tokens = self.get_num_token_transfers(&klv_payment, &kda_payment);
        self.check_fees_cover_number_of_tokens(num_tokens, fee_amount, deposited_fee_token.amount);

        self.make_fund(klv_payment, kda_payment, address, valability);
    }

    #[endpoint(depositFees)]
    #[payable("KLV")]
    fn deposit_fees(&self, address: &ManagedAddress) {
        let payment = self.call_value().klv_or_single_kda();
        let caller_address = self.blockchain().get_caller();
        self.update_fees(caller_address, address, payment);
    }
}
