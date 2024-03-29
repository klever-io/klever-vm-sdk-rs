use crate::{
    api::ManagedTypeApi,
    types::{BigUint, MultiKdaPayment},
};

/// Holding back-transfer data, as retrieved from the VM.
pub struct BackTransfers<A>
where
    A: ManagedTypeApi,
{
    pub klv_amount: BigUint<A>,
    pub kda_payments: MultiKdaPayment<A>,
}