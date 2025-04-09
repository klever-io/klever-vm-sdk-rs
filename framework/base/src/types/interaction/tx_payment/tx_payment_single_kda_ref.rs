use crate::{
    contract_base::SendRawWrapper,
    types::{BigUint, KdaTokenPaymentRefs, ManagedAddress, MultiKdaPayment, TxFrom, TxToSpecified},
};

use super::{FullPaymentData, FunctionCall, TxEnv, TxPayment};

impl<Env> TxPayment<Env> for KdaTokenPaymentRefs<'_, Env::Api>
where
    Env: TxEnv,
{
    #[inline]
    fn is_no_payment(&self, _env: &Env) -> bool {
        self.amount == &0u32
    }

    fn perform_transfer_execute(
        self,
        _env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        if self.token_nonce == 0 {
            // fungible KDA
            let _ = SendRawWrapper::<Env::Api>::new().transfer_kda_execute(
                to,
                self.token_identifier,
                self.amount,
                gas_limit,
                &fc.function_name,
                &fc.arg_buffer,
            );
        } else {
            // non-fungible/semi-fungible KDA
            let _ = SendRawWrapper::<Env::Api>::new().transfer_kda_nft_execute(
                to,
                self.token_identifier,
                self.token_nonce,
                self.amount,
                gas_limit,
                &fc.function_name,
                &fc.arg_buffer,
            );
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
        to.with_address_ref(env, |to_addr| {
            let fc_conv = fc.convert_to_transfer_kda_call(to_addr, self);
            f(&from.resolve_address(env), &*BigUint::zero_ref(), fc_conv)
        })
    }

    fn into_full_payment_data(self, _env: &Env) -> FullPaymentData<Env::Api> {
        FullPaymentData {
            klv: None,
            multi_kda: MultiKdaPayment::from_single_item(self.to_owned_payment()),
        }
    }
}
