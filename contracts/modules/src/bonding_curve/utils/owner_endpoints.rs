use klever_sc::imports::*;

use klever_sc::contract_base::ManagedSerializer;

use crate::bonding_curve::{
    curves::curve_function::CurveFunction,
    utils::{
        events, storage,
        structs::{BondingCurve, TokenOwnershipData},
    },
};

use super::structs::CurveArguments;

#[klever_sc::module]
pub trait OwnerEndpointsModule: storage::StorageModule + events::EventsModule {
    fn set_bonding_curve<T>(
        &self,
        identifier: TokenIdentifier,
        function: T,
        sell_availability: bool,
    ) where
        T: CurveFunction<Self::Api>
            + TopEncode
            + TopDecode
            + NestedEncode
            + NestedDecode
            + TypeAbi
            + PartialEq
            + Default,
    {
        require!(
            !self.token_details(&identifier).is_empty(),
            "Token is not issued yet!"
        );

        let caller = self.blockchain().get_caller();

        let details = self.token_details(&identifier).get();
        require!(
            details.owner == caller,
            "The price function can only be set by the seller."
        );
        self.bonding_curve(&identifier).update(|buffer| {
            let serializer = ManagedSerializer::new();

            let mut bonding_curve: BondingCurve<Self::Api, T> =
                serializer.top_decode_from_managed_buffer(buffer);
            bonding_curve.curve = function;
            bonding_curve.sell_availability = sell_availability;
            *buffer = serializer.top_encode_to_managed_buffer(&bonding_curve);
        });
    }

    fn deposit<T>(&self, payment_token: OptionalValue<TokenIdentifier>)
    where
        T: CurveFunction<Self::Api>
            + TopEncode
            + TopDecode
            + NestedEncode
            + NestedDecode
            + TypeAbi
            + PartialEq
            + Default,
    {
        let (identifier, nonce, amount) = self.call_value().single_kda().into_tuple();
        let caller = self.blockchain().get_caller();
        let mut set_payment = TokenIdentifier::klv();

        if self.bonding_curve(&identifier).is_empty() {
            match payment_token {
                OptionalValue::Some(token) => set_payment = token,
                OptionalValue::None => {
                    sc_panic!("Expected provided accepted_payment for the token");
                },
            };
        }
        if self.token_details(&identifier).is_empty() {
            let nonces = ManagedVec::from_single_item(nonce);
            self.token_details(&identifier).set(&TokenOwnershipData {
                token_nonces: nonces,
                owner: caller.clone(),
            });
        } else {
            let mut details = self.token_details(&identifier).get();
            require!(
                details.owner == caller,
                "The token was already deposited by another address"
            );
            if !details.token_nonces.contains(&nonce) {
                details.token_nonces.push(nonce);
                self.token_details(&identifier).set(&details);
            }
        }

        self.set_curve_storage::<T>(&identifier, amount.clone(), set_payment);
        self.owned_tokens(&caller).insert(identifier.clone());
        self.nonce_amount(&identifier, nonce)
            .update(|current_amount| *current_amount += amount);
    }

    fn claim<T>(&self)
    where
        T: CurveFunction<Self::Api>
            + TopEncode
            + TopDecode
            + NestedEncode
            + NestedDecode
            + TypeAbi
            + PartialEq
            + Default,
    {
        let caller = self.blockchain().get_caller();
        require!(
            !self.owned_tokens(&caller).is_empty(),
            "You have nothing to claim"
        );

        let mut tokens_to_claim = ManagedVec::<Self::Api, KdaTokenPayment<Self::Api>>::new();
        let mut klv_to_claim = BigUint::zero();
        let serializer = ManagedSerializer::new();
        for token in self.owned_tokens(&caller).iter() {
            let nonces = self.token_details(&token).get().token_nonces;
            for nonce in &nonces {
                tokens_to_claim.push(KdaTokenPayment::new(
                    token.clone(),
                    nonce,
                    self.nonce_amount(&token, nonce).get(),
                ));

                self.nonce_amount(&token, nonce).clear();
            }

            let bonding_curve: BondingCurve<Self::Api, T> =
                serializer.top_decode_from_managed_buffer(&self.bonding_curve(&token).get());

            if !bonding_curve.payment.token_identifier.is_klv() {
                tokens_to_claim.push(KdaTokenPayment::new(
                    bonding_curve.payment.token_identifier,
                    bonding_curve.payment.token_nonce,
                    bonding_curve.payment.amount,
                ));
            } else {
                klv_to_claim += bonding_curve.payment.amount;
            }

            self.token_details(&token).clear();
            self.bonding_curve(&token).clear();
        }
        self.owned_tokens(&caller).clear();
        self.tx().to(&caller).multi_kda(tokens_to_claim).transfer();
        if klv_to_claim > BigUint::zero() {
            self.tx().to(&caller).klv(&klv_to_claim).transfer();
        }
    }

    fn set_curve_storage<T>(
        &self,
        identifier: &TokenIdentifier,
        amount: BigUint,
        payment_token_identifier: TokenIdentifier,
    ) where
        T: CurveFunction<Self::Api>
            + TopEncode
            + TopDecode
            + NestedEncode
            + NestedDecode
            + TypeAbi
            + PartialEq
            + Default,
    {
        let mut curve: T = T::default();
        let mut arguments;
        let payment;
        let sell_availability: bool;
        let serializer = ManagedSerializer::new();

        if self.bonding_curve(identifier).is_empty() {
            arguments = CurveArguments {
                available_supply: amount.clone(),
                balance: amount,
            };
            payment = KdaTokenPayment::new(payment_token_identifier, 0, BigUint::zero());
            sell_availability = false;
        } else {
            let bonding_curve: BondingCurve<Self::Api, T> =
                serializer.top_decode_from_managed_buffer(&self.bonding_curve(identifier).get());

            payment = bonding_curve.payment;
            curve = bonding_curve.curve;
            arguments = bonding_curve.arguments;
            arguments.balance += &amount;
            arguments.available_supply += amount;
            sell_availability = bonding_curve.sell_availability;
        }
        let encoded_curve = serializer.top_encode_to_managed_buffer(&BondingCurve {
            curve,
            arguments,
            sell_availability,
            payment,
        });
        self.bonding_curve(identifier).set(encoded_curve);
    }
}
