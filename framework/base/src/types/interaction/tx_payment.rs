pub use tx_payment_klv::{Klv, KlvPayment};
pub use tx_payment_klv_value::TxKlvValue;
pub use tx_payment_multi_kda::TxPaymentMultiKda;
pub use tx_payment_not_payable::NotPayable;

use crate::{
    api::ManagedTypeApi
    ,
    types::{
        ManagedAddress,
        MultiKdaPayment,
    },
};
use crate::types::{AnnotatedValue, BigUint, FunctionCall, ManagedBuffer, TxEnv, TxFrom, TxToSpecified};

mod tx_payment_klv;
mod tx_payment_klv_or_multi_kda;
mod tx_payment_klv_or_multi_kda_ref;
mod tx_payment_klv_value;
mod tx_payment_multi_kda;
mod tx_payment_none;
mod tx_payment_not_payable;
mod tx_payment_single_kda;
mod tx_payment_single_kda_ref;
mod tx_payment_single_kda_triple;

/// Describes a payment that is part of a transaction.
#[diagnostic::on_unimplemented(
    message = "Type `{Self}` cannot be used as payment (does not implement `TxPayment<{Env}>`)",
    label = "not a valid payment type",
    note = "there are multiple ways to specify the transaction payment, but `{Self}` is not one of them"
)]
pub trait TxPayment<Env>
where
    Env: TxEnv,
{
    /// Returns true if payment indicates transfer of either non-zero KLV or KDA amounts.
    fn is_no_payment(&self, env: &Env) -> bool;

    /// Transfer-execute calls have different APIs for different payments types.
    /// This method selects between them.
    fn perform_transfer_execute(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    );

    /// Converts an KDA call to a built-in function call, if necessary.
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
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, FunctionCall<Env::Api>) -> R;

    /// Payment data to be used by the testing framework. Will be refactored.
    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api>;
}

/// Marker trait that indicates that payment field contains no payment.
///
/// Implemented by `()` and `NotPayable`.
pub trait TxNoPayment<Env>: TxPayment<Env>
where
    Env: TxEnv,
{
}

/// Marks a payment object that only contains KLV or nothing at all.
pub trait TxPaymentKlvOnly<Env>: TxPayment<Env> + AnnotatedValue<Env, BigUint<Env::Api>>
where
    Env: TxEnv,
{
    #[inline]
    fn with_klv_value<F, R>(&self, env: &Env, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R,
    {
        self.with_value_ref(env, f)
    }

    fn into_klv_payment(self, env: &Env) -> BigUint<Env::Api> {
        self.into_value(env)
    }
}

#[derive(Clone)]
pub struct AnnotatedKlvPayment<Api>
where
    Api: ManagedTypeApi,
{
    pub value: BigUint<Api>,
    pub annotation: ManagedBuffer<Api>,
}

impl<Api> AnnotatedKlvPayment<Api>
where
    Api: ManagedTypeApi,
{
    pub fn new_klv(value: BigUint<Api>) -> Self {
        let annotation = value.to_display();
        AnnotatedKlvPayment { value, annotation }
    }
}

#[derive(Clone)]
pub struct FullPaymentData<Api>
where
    Api: ManagedTypeApi,
{
    pub klv: Option<AnnotatedKlvPayment<Api>>,
    pub multi_kda: MultiKdaPayment<Api>,
}

impl<Api> Default for FullPaymentData<Api>
where
    Api: ManagedTypeApi,
{
    fn default() -> Self {
        Self {
            klv: None,
            multi_kda: Default::default(),
        }
    }
}
