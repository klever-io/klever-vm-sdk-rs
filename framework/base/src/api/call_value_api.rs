use super::{ErrorApiImpl, HandleTypeInfo, ManagedTypeApiImpl};

pub trait CallValueApi: HandleTypeInfo {
    type CallValueApiImpl: CallValueApiImpl
        + HandleTypeInfo<
            ManagedBufferHandle = Self::ManagedBufferHandle,
            BigIntHandle = Self::BigIntHandle,
            BigFloatHandle = Self::BigFloatHandle,
            EllipticCurveHandle = Self::EllipticCurveHandle,
        >;

    fn call_value_api_impl() -> Self::CallValueApiImpl;
}

pub trait CallValueApiImpl: ErrorApiImpl + ManagedTypeApiImpl + Sized {
    fn check_not_payable(&self);

    /// Retrieves the KLV call value from the VM.
    fn load_klv_value(&self, dest_handle: Self::BigIntHandle);

    /// Retrieves the call value from the VM by Token Name.
    /// Will return 0 in case of not existent.
    fn load_kda_value(
        &self,
        dest_handle: Self::BigIntHandle,
        token_id_handle: Self::ManagedBufferHandle,
    );

    /// Loads all KDA call values into a managed vec. Overwrites destination.
    fn load_all_kda_transfers(&self, dest_handle: Self::ManagedBufferHandle);

    /// Gets the total number of KDA transfers (Fungible/SFT/NFT).
    ///
    /// It is redundant, since the number can also be retrieved from `load_all_kda_transfers`,
    /// but it is easier and cheaper to call when the content of those transfers is of no interest.
    fn kda_num_transfers(&self) -> usize;
}
