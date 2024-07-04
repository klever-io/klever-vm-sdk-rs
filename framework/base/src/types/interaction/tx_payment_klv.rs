use crate::{
    contract_base::SendRawWrapper,
    types::{
        BigUint, ManagedAddress, ManagedVec,
    },
};
use crate::types::TokenIdentifier;

use super::{
    AnnotatedKlvPayment, AnnotatedValue, FullPaymentData, FunctionCall, TxEnv, TxKlvValue, TxPayment
};

/// Indicates the KLV payment in a transaction.
pub struct Klv<KlvValue>(pub KlvValue);

pub type KlvPayment<Api> = Klv<BigUint<Api>>;

impl<Env, KlvValue> TxPayment<Env> for Klv<KlvValue>
where
    Env: TxEnv,
    KlvValue: TxKlvValue<Env>,
{
    fn is_no_payment(&self) -> bool {
        self.0.with_klv_value(|klv_value| klv_value == &0u32)
    }

    fn perform_transfer_execute(
        self,
        _env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        self.0.with_klv_value(|klv_value| {
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

    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api> {
        FullPaymentData {
            klv: Some(AnnotatedKlvPayment::new_klv(self.0.into_value(env))),
            multi_kda: ManagedVec::new(),
        }
    }
}

/// Marks a payment object that only contains KLV or nothing at all.
pub trait TxPaymentKlvOnly<Env>: TxPayment<Env>
where
    Env: TxEnv,
{
    fn with_klv_value<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R;

    fn into_klv_payment(self, env: &Env) -> BigUint<Env::Api>;
}

impl<Env> TxPaymentKlvOnly<Env> for ()
where
    Env: TxEnv,
{
    fn with_klv_value<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R,
    {
        f(&BigUint::zero())
    }

    fn into_klv_payment(self, _env: &Env) -> BigUint<Env::Api> {
        BigUint::zero()
    }
}

impl<Env, KlvValue> TxPaymentKlvOnly<Env> for Klv<KlvValue>
where
    Env: TxEnv,
    KlvValue: TxKlvValue<Env>,
{
    fn with_klv_value<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R,
    {
        self.0.with_klv_value(f)
    }

    fn into_klv_payment(self, env: &Env) -> BigUint<Env::Api> {
        self.0.into_value(env)
    }
}
