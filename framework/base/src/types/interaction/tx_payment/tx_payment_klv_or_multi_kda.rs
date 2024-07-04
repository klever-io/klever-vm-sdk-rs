use crate::types::{
    BigUint, ManagedAddress
    , TxFrom, TxToSpecified,
};
use crate::types::{Klv, KlvOrMultiKdaPayment};

use super::{FullPaymentData, FunctionCall, TxEnv, TxPayment};

impl<Env> TxPayment<Env> for KlvOrMultiKdaPayment<Env::Api>
where
    Env: TxEnv,
{
    fn is_no_payment(&self, _env: &Env) -> bool {
        self.is_empty()
    }

    fn perform_transfer_execute(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        match self {
            KlvOrMultiKdaPayment::Klv(klv_amount) => {
                Klv(klv_amount).perform_transfer_execute(env, to, gas_limit, fc)
            },
            KlvOrMultiKdaPayment::MultiKda(multi_kda_payment) => {
                multi_kda_payment.perform_transfer_execute(env, to, gas_limit, fc)
            },
        }
    }

    fn with_normalized<From, To, F, R>(
        self,
        env: &Env,
        from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        From: TxFrom<Env>,
        To: TxToSpecified<Env>,
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

    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api> {
        match self {
            KlvOrMultiKdaPayment::Klv(klv_amount) => {
                TxPayment::<Env>::into_full_payment_data(Klv(klv_amount), env)
            },
            KlvOrMultiKdaPayment::MultiKda(multi_kda_payment) => {
                TxPayment::<Env>::into_full_payment_data(multi_kda_payment, env)
            },
        }
    }
}

impl<Env> TxPayment<Env> for &KlvOrMultiKdaPayment<Env::Api>
where
    Env: TxEnv,
{
    fn is_no_payment(&self, _env: &Env) -> bool {
        self.is_empty()
    }

    fn perform_transfer_execute(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        match self {
            KlvOrMultiKdaPayment::Klv(klv_amount) => {
                Klv(klv_amount).perform_transfer_execute(env, to, gas_limit, fc)
            },
            KlvOrMultiKdaPayment::MultiKda(multi_kda_payment) => {
                multi_kda_payment.perform_transfer_execute(env, to, gas_limit, fc)
            },
        }
    }

    fn with_normalized<From, To, F, R>(
        self,
        env: &Env,
        from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        From: TxFrom<Env>,
        To: TxToSpecified<Env>,
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

    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api> {
        match self {
            KlvOrMultiKdaPayment::Klv(klv_amount) => {
                TxPayment::<Env>::into_full_payment_data(Klv(klv_amount), env)
            },
            KlvOrMultiKdaPayment::MultiKda(multi_kda_payment) => {
                TxPayment::<Env>::into_full_payment_data(multi_kda_payment, env)
            },
        }
    }
}
