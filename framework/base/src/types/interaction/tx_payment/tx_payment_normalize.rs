use crate::types::{BigUint, KdaTokenPayment, KlvOrMultiKdaPayment, ManagedAddress, MultiKdaPayment, TxFrom, TxToSpecified};
use crate::types::{Klv, TxKlvValue};

use super::{
    FunctionCall, TxEnv, TxPayment,
};

/// Defines how a payment transforms a transaction,
/// e.g. from KDA transfer to KDATransfer builtin function.
pub trait TxPaymentNormalize<Env, From, To>: TxPayment<Env>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxToSpecified<Env>,
{
    fn with_normalized<F, R>(
        self,
        env: &Env,
        from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, &FunctionCall<Env::Api>) -> R;
}

impl<Env, From, To> TxPaymentNormalize<Env, From, To> for ()
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxToSpecified<Env>,
{
    fn with_normalized<F, R>(
        self,
        env: &Env,
        _from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, &FunctionCall<Env::Api>) -> R,
    {
        to.with_address_ref(env, |to_addr| f(to_addr, &BigUint::zero(), &fc))
    }
}

impl<Env, From, To, KlvValue> TxPaymentNormalize<Env, From, To> for Klv<KlvValue>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxToSpecified<Env>,
    KlvValue: TxKlvValue<Env>,
{
    fn with_normalized<F, R>(
        self,
        env: &Env,
        _from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, &FunctionCall<Env::Api>) -> R,
    {
        to.with_address_ref(env, |to_addr| {
            self.0
                .with_klv_value(|klv_value| f(to_addr, klv_value, &fc))
        })
    }
}

impl<Env, From, To> TxPaymentNormalize<Env, From, To> for KdaTokenPayment<Env::Api>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxToSpecified<Env>,
{
    fn with_normalized<F, R>(
        self,
        env: &Env,
        from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, &FunctionCall<Env::Api>) -> R,
    {
        to.with_address_ref(env, |to_addr| {
            let fc_conv = fc.convert_to_transfer_kda_call(to_addr, self);
            f(&from.resolve_address(env), &BigUint::zero(), &fc_conv)
        })
    }
}

impl<Env, From, To> TxPaymentNormalize<Env, From, To> for MultiKdaPayment<Env::Api>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxToSpecified<Env>,
{
    fn with_normalized<F, R>(
        self,
        env: &Env,
        from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, &FunctionCall<Env::Api>) -> R,
    {
        match self.len() {
            0 => ().with_normalized(env, from, to, fc, f),
            1 => self.get(0).with_normalized(env, from, to, fc, f),
            _ => to.with_address_ref(env, |to_addr| {
                let fc_conv = fc.convert_to_multi_transfer_kda_call(to_addr, self);
                f(&from.resolve_address(env), &BigUint::zero(), &fc_conv)
            }),
        }
    }
}

impl<Env, From, To> TxPaymentNormalize<Env, From, To> for KlvOrMultiKdaPayment<Env::Api>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxToSpecified<Env>,
{
    fn with_normalized<F, R>(
        self,
        env: &Env,
        from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, &FunctionCall<Env::Api>) -> R,
    {
        match self {
            KlvOrMultiKdaPayment::Klv(klv_amount) => {
                Klv(klv_amount).with_normalized(env, from, to, fc, f)
            },
            KlvOrMultiKdaPayment::MultiKda(multi_kda_payment) => {
                multi_kda_payment.with_normalized(env, from, to, fc, f)
            },
        }
    }
}
