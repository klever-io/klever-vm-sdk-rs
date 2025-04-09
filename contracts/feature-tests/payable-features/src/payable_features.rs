#![no_std]
#![allow(clippy::type_complexity)]

use klever_sc::imports::*;

/// Contract that only tests the call value features,
/// i.e. the framework/Arwen functionality for accepting KLV and KDA payments.
#[klever_sc::contract]
pub trait PayableFeatures {
    #[init]
    fn init(&self) {}

    #[view]
    #[payable("*")]
    fn echo_call_value(
        &self,
    ) -> MultiValue2<BigUint, ManagedVec<Self::Api, KdaTokenPayment<Self::Api>>> {
        (
            self.call_value().klv_value().clone_value(),
            self.call_value().all_kda_transfers_no_klv().clone_value(),
        )
            .into()
    }

    #[endpoint]
    #[payable("*")]
    fn payment_multiple(
        &self,
        #[payment_multi] payments: ManagedRef<'static, ManagedVec<KdaTokenPayment<Self::Api>>>,
    ) -> ManagedVec<KdaTokenPayment<Self::Api>> {
        payments.clone_value()
    }

    #[endpoint]
    #[payable("*")]
    fn payment_array_3(&self) -> MultiValue3<KdaTokenPayment, KdaTokenPayment, KdaTokenPayment> {
        let [payment_a, payment_b, payment_c] = self.call_value().multi_kda();
        (payment_a, payment_b, payment_c).into()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_any_1(
        &self,
        #[payment_amount] payment: BigUint,
        #[payment_token] token: TokenIdentifier,
    ) -> MultiValue2<BigUint, TokenIdentifier> {
        (payment, token).into()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_any_2(&self, #[payment] payment: BigUint) -> MultiValue2<BigUint, TokenIdentifier> {
        let token = self.call_value().klv_or_single_kda().token_identifier;
        (payment, token).into()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_any_3(
        &self,
        #[payment_token] token: TokenIdentifier,
    ) -> MultiValue2<BigUint, TokenIdentifier> {
        let payment = self.call_value().klv_or_single_kda();
        (payment.amount, token).into()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_any_4(&self) -> MultiValue2<BigUint, TokenIdentifier> {
        let payment = self.call_value().klv_or_single_kda();
        (payment.amount, payment.token_identifier).into()
    }

    #[endpoint]
    #[payable("KLV")]
    fn payable_klv_1(
        &self,
        #[payment_token] token: TokenIdentifier,
    ) -> MultiValue2<BigUint, TokenIdentifier> {
        let payment = self.call_value().klv_value().clone_value();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("KLV")]
    fn payable_klv_2(&self, #[payment] payment: BigUint) -> MultiValue2<BigUint, TokenIdentifier> {
        let token = self.call_value().klv_or_single_kda().token_identifier;
        (payment, token).into()
    }

    #[endpoint]
    #[payable("KLV")]
    fn payable_klv_3(
        &self,
        #[payment_token] token: TokenIdentifier,
    ) -> MultiValue2<BigUint, TokenIdentifier> {
        let payment = self.call_value().klv_value().clone_value();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("KLV")]
    fn payable_klv_4(&self) -> MultiValue2<BigUint, TokenIdentifier> {
        let payment = self.call_value().klv_value();
        let token = self.call_value().klv_or_single_kda().token_identifier;
        (payment.clone_value(), token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_1(
        &self,
        #[payment] payment: BigUint,
        #[payment_token] token: TokenIdentifier,
    ) -> MultiValue2<BigUint, TokenIdentifier> {
        (payment, token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_2(
        &self,
        #[payment] payment: BigUint,
    ) -> MultiValue2<BigUint, TokenIdentifier> {
        let token = self.call_value().single_kda().token_identifier;
        (payment, token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_3(
        &self,
        #[payment_token] token: TokenIdentifier,
    ) -> MultiValue2<BigUint, TokenIdentifier> {
        let payment = self.call_value().single_kda();
        (payment.amount, token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_4(&self) -> MultiValue2<BigUint, TokenIdentifier> {
        let payment = self.call_value().single_kda().amount;
        let token = self.call_value().single_kda().token_identifier;
        (payment, token).into()
    }
}
