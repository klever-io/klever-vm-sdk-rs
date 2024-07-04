use crate::{
    contract_base::SendRawWrapper,
    types::{BigUint, ManagedAddress, ManagedVec},
};
use crate::types::{AnnotatedValue, ManagedBuffer, TokenIdentifier, TxFrom, TxToSpecified};

use super::{
    AnnotatedKlvPayment, FullPaymentData, FunctionCall, TxKlvValue, TxEnv, TxPayment,
    TxPaymentKlvOnly,
};

/// Indicates the KLV payment in a transaction.
pub struct Klv<KlvValue>(pub KlvValue);

pub type KlvPayment<Api> = Klv<BigUint<Api>>;

impl<Env, KlvValue> TxPayment<Env> for Klv<KlvValue>
where
    Env: TxEnv,
    KlvValue: TxKlvValue<Env>,
{
    fn is_no_payment(&self, env: &Env) -> bool {
        self.0.with_value_ref(env, |klv_value| klv_value == &0u32)
    }

    fn perform_transfer_execute(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        self.0.with_value_ref(env, |klv_value| {
            let _ = SendRawWrapper::<Env::Api>::new().transfer_kda_execute(
                to,
                &TokenIdentifier::klv(),
                klv_value,
                gas_limit,
                &fc.function_name,
                &fc.arg_buffer,
            );
        })
    }

    fn with_normalized<From, To, F, R>(
        self,
        env: &Env,
        _from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        From: TxFrom<Env>,
        To: TxToSpecified<Env>,
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, &FunctionCall<Env::Api>) -> R,
    {
        to.with_address_ref(env, |to_addr| {
            self.0
                .with_value_ref(env, |klv_value| f(to_addr, klv_value, &fc))
        })
    }

    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api> {
        FullPaymentData {
            klv: Some(AnnotatedKlvPayment::new_klv(self.0.into_value(env))),
            multi_kda: ManagedVec::new(),
        }
    }
}

impl<Env, KlvValue> AnnotatedValue<Env, BigUint<Env::Api>> for Klv<KlvValue>
where
    Env: TxEnv,
    KlvValue: TxKlvValue<Env>,
{
    fn annotation(&self, env: &Env) -> ManagedBuffer<Env::Api> {
        self.0.annotation(env)
    }

    fn to_value(&self, env: &Env) -> BigUint<Env::Api> {
        self.0.to_value(env)
    }

    fn into_value(self, env: &Env) -> BigUint<Env::Api> {
        self.0.into_value(env)
    }

    fn with_value_ref<F, R>(&self, env: &Env, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R,
    {
        self.0.with_value_ref(env, f)
    }
}

impl<Env, KlvValue> TxPaymentKlvOnly<Env> for Klv<KlvValue>
where
    Env: TxEnv,
    KlvValue: TxKlvValue<Env>,
{
}
