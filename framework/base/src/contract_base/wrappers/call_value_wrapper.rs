use core::marker::PhantomData;

use crate::{
    api::{
        const_handles, use_raw_handle, CallValueApi, CallValueApiImpl, ErrorApi, ErrorApiImpl,
        HandleConstraints, ManagedTypeApi, StaticVarApiImpl,
    },
    err_msg,
    types::{
        BigUint, KdaTokenPayment, KlvOrMultiKdaPayment, ManagedRef, ManagedVec, TokenIdentifier,
    },
};

#[derive(Default)]
pub struct CallValueWrapper<A>
where
    A: CallValueApi + ErrorApi + ManagedTypeApi,
{
    _phantom: PhantomData<A>,
}

impl<A> CallValueWrapper<A>
where
    A: CallValueApi + ErrorApi + ManagedTypeApi,
{
    pub fn new() -> Self {
        CallValueWrapper {
            _phantom: PhantomData,
        }
    }

    /// Retrieves the KLV call value from the VM.
    pub fn klv_value(&self) -> ManagedRef<'static, A, BigUint<A>> {
        let mut call_value_handle: A::BigIntHandle =
            use_raw_handle(A::static_var_api_impl().get_call_value_klv_handle());
        if call_value_handle == const_handles::UNINITIALIZED_HANDLE {
            call_value_handle = use_raw_handle(const_handles::CALL_VALUE_KLV);
            A::static_var_api_impl().set_call_value_klv_handle(call_value_handle.get_raw_handle());
            A::call_value_api_impl().load_klv_value(call_value_handle.clone());
        }
        unsafe { ManagedRef::wrap_handle(call_value_handle) }
    }

    /// Returns all KDA transfers that accompany this SC call.
    /// Will return 0 results if nothing was transferred.
    /// Fully managed underlying types, very efficient.
    pub fn all_kda_transfers(&self) -> ManagedRef<'static, A, ManagedVec<A, KdaTokenPayment<A>>> {
        let mut call_value_handle: A::ManagedBufferHandle =
            use_raw_handle(A::static_var_api_impl().get_call_value_multi_kda_handle());
        if call_value_handle == const_handles::UNINITIALIZED_HANDLE {
            call_value_handle = use_raw_handle(const_handles::CALL_VALUE_MULTI_KDA);
            A::static_var_api_impl()
                .set_call_value_multi_kda_handle(call_value_handle.get_raw_handle());
            A::call_value_api_impl().load_all_kda_transfers(call_value_handle.clone());
        }
        unsafe { ManagedRef::wrap_handle(call_value_handle) }
    }

    /// Returns all KDA transfers that accompany this SC call. No KLV transfers are included.
    /// Will return 0 results if nothing was transferred.
    /// Fully managed underlying types, very efficient.
    pub fn all_kda_transfers_no_klv(
        &self,
    ) -> ManagedRef<'static, A, ManagedVec<A, KdaTokenPayment<A>>> {
        let mut call_value_handle: A::ManagedBufferHandle =
            use_raw_handle(A::static_var_api_impl().get_call_value_multi_kda_no_klv_handle());
        if call_value_handle == const_handles::UNINITIALIZED_HANDLE {
            call_value_handle = use_raw_handle(const_handles::CALL_VALUE_MULTI_KDA_NO_KLV);
            A::static_var_api_impl()
                .set_call_value_multi_kda_no_klv_handle(call_value_handle.get_raw_handle());
            A::call_value_api_impl().load_all_kda_transfers_no_klv(call_value_handle.clone());
        }
        unsafe { ManagedRef::wrap_handle(call_value_handle) }
    }

    /// Verify and casts the received multi KDA transfer in to an array.
    ///
    /// Can be used to extract all payments in one line like this:
    ///
    /// `let [payment_a, payment_b, payment_c] = self.call_value().multi_kda();`.
    pub fn multi_kda<const N: usize>(&self) -> [KdaTokenPayment<A>; N] {
        self.all_kda_transfers_no_klv()
            .to_array_of_refs::<N>()
            .unwrap_or_else(|| {
                A::error_api_impl().signal_error(err_msg::INCORRECT_NUM_KDA_TRANSFERS.as_bytes())
            })
    }

    /// Expects precisely one KDA token transfer, fungible or not.
    ///
    /// Will return the received KDA payment.
    ///
    /// The amount cannot be 0, since that would not qualify as an KDA transfer.
    pub fn single_kda(&self) -> KdaTokenPayment<A> {
        let [payments] = self.multi_kda();
        payments
    }

    /// Expects precisely one fungible KDA token transfer.
    ///
    /// Returns the token ID and the amount for fungible KDA transfers.
    ///
    /// The amount cannot be 0, since that would not qualify as an KDA transfer.
    pub fn single_fungible_kda(&self) -> (TokenIdentifier<A>, BigUint<A>) {
        let payment = self.single_kda();
        if payment.token_nonce != 0 {
            A::error_api_impl().signal_error(err_msg::FUNGIBLE_TOKEN_EXPECTED_ERR_MSG.as_bytes());
        }
        (payment.token_identifier, payment.amount)
    }

    /// Accepts and returns either an KLV payment, or a single KDA token.
    ///
    /// Will halt execution if more than one KDA transfer was received.
    ///
    /// In case no transfer of value happen, it will return a payment of 0 KLV.
    pub fn klv_or_single_kda(&self) -> KdaTokenPayment<A> {
        let kda_transfers = self.all_kda_transfers_no_klv();
        match kda_transfers.len() {
            0 => KdaTokenPayment {
                token_identifier: TokenIdentifier::klv(),
                token_nonce: 0,
                amount: self.klv_value().clone_value(),
            },
            1 => kda_transfers.get(0),
            _ => A::error_api_impl().signal_error(err_msg::INCORRECT_NUM_KDA_TRANSFERS.as_bytes()),
        }
    }

    /// Accepts and returns either an KLV payment, or a single fungible KDA token.
    ///
    /// Will halt execution if more than one KDA transfer was received, or if the received KDA is non- or semi-fungible.
    ///
    /// Works similar to `klv_or_single_kda`,
    /// but checks the nonce to be 0 and returns a tuple of just token identifier and amount, for convenience.
    ///
    /// In case no transfer of value happen, it will return a payment of 0 KLV.
    pub fn klv_or_single_fungible_kda(&self) -> (TokenIdentifier<A>, BigUint<A>) {
        let payment = self.klv_or_single_kda();
        if payment.token_nonce != 0 {
            A::error_api_impl().signal_error(err_msg::FUNGIBLE_TOKEN_EXPECTED_ERR_MSG.as_bytes());
        }

        (payment.token_identifier, payment.amount)
    }

    /// Accepts any sort of payment, which is either:
    /// - KLV (can be zero in case of no payment whatsoever);
    /// - Multi-KDA (one or more KDA transfers).
    pub fn any_payment(&self) -> KlvOrMultiKdaPayment<A> {
        let kda_transfers = self.all_kda_transfers();
        if kda_transfers.is_empty() {
            KlvOrMultiKdaPayment::Klv(self.klv_value().clone_value())
        } else {
            KlvOrMultiKdaPayment::MultiKda(kda_transfers.clone_value())
        }
    }
}
