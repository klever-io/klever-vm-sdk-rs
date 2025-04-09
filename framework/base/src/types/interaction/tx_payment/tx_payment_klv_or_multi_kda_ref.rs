use crate::{
    proxy_imports::KlvOrMultiKdaPaymentRefs,
    types::{BigUint, ManagedAddress, TxFrom, TxToSpecified},
};

use super::{FullPaymentData, FunctionCall, Klv, TxEnv, TxPayment};

impl<Env> TxPayment<Env> for KlvOrMultiKdaPaymentRefs<'_, Env::Api>
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
            KlvOrMultiKdaPaymentRefs::Klv(klv_amount) => {
                Klv(klv_amount).perform_transfer_execute(env, to, gas_limit, fc);
            },
            KlvOrMultiKdaPaymentRefs::MultiKda(multi_kda_payment) => {
                multi_kda_payment.perform_transfer_execute(env, to, gas_limit, fc);
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
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, FunctionCall<Env::Api>) -> R,
    {
        match self {
            KlvOrMultiKdaPaymentRefs::Klv(klv_amount) => {
                Klv(klv_amount).with_normalized(env, from, to, fc, f)
            },
            KlvOrMultiKdaPaymentRefs::MultiKda(multi_kda_payment) => {
                multi_kda_payment.with_normalized(env, from, to, fc, f)
            },
        }
    }

    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api> {
        match self {
            KlvOrMultiKdaPaymentRefs::Klv(klv_amount) => {
                Klv(klv_amount).into_full_payment_data(env)
            },
            KlvOrMultiKdaPaymentRefs::MultiKda(multi_kda_payment) => {
                multi_kda_payment.into_full_payment_data(env)
            },
        }
    }
}
