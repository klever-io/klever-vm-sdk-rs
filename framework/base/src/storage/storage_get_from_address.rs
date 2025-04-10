use crate::{
    api::{
        const_handles, use_raw_handle, ErrorApi, ManagedBufferApiImpl, ManagedTypeApi,
        StaticVarApiImpl, StorageReadApi, StorageReadApiImpl,
    },
    codec::*,
    types::{
        BigInt, BigUint, ManagedAddress, ManagedBuffer, ManagedBufferNestedDecodeInput, ManagedRef,
        ManagedType,
    },
};
use alloc::boxed::Box;
use unwrap_infallible::UnwrapInfallible;

use super::{StorageGetErrorHandler, StorageKey};

struct StorageGetFromAddressInput<'k, A>
where
    A: StorageReadApi + ManagedTypeApi + ErrorApi + 'static,
{
    addr: ManagedRef<'k, A, ManagedAddress<A>>,
    key: ManagedRef<'k, A, StorageKey<A>>,
}

impl<'k, A> StorageGetFromAddressInput<'k, A>
where
    A: StorageReadApi + ManagedTypeApi + ErrorApi + 'static,
{
    #[inline]
    fn new(
        addr: ManagedRef<'k, A, ManagedAddress<A>>,
        key: ManagedRef<'k, A, StorageKey<A>>,
    ) -> Self {
        StorageGetFromAddressInput { addr, key }
    }

    fn to_managed_buffer(&self) -> ManagedBuffer<A> {
        let mbuf_handle: A::ManagedBufferHandle =
            use_raw_handle(A::static_var_api_impl().next_handle());
        A::storage_read_api_impl().storage_load_from_address(
            self.addr.get_handle(),
            self.key.buffer.get_handle(),
            mbuf_handle.clone(),
        );

        ManagedBuffer::from_handle(mbuf_handle)
    }

    fn to_big_uint(&self) -> BigUint<A> {
        BigUint::from_bytes_be_buffer(&self.to_managed_buffer())
    }

    fn to_big_int(&self) -> BigInt<A> {
        BigInt::from_signed_bytes_be_buffer(&self.to_managed_buffer())
    }

    fn load_len_managed_buffer(&self) -> usize {
        let value_handle: A::ManagedBufferHandle = use_raw_handle(const_handles::MBUF_TEMPORARY_1);
        A::storage_read_api_impl().storage_load_from_address(
            self.addr.get_handle(),
            self.key.buffer.get_handle(),
            value_handle.clone(),
        );

        A::managed_type_impl().mb_len(value_handle)
    }
}

impl<A> TopDecodeInput for StorageGetFromAddressInput<'_, A>
where
    A: StorageReadApi + ManagedTypeApi + ErrorApi + 'static,
{
    type NestedBuffer = ManagedBufferNestedDecodeInput<A>;

    fn byte_len(&self) -> usize {
        self.load_len_managed_buffer()
    }

    fn into_boxed_slice_u8(self) -> Box<[u8]> {
        self.to_managed_buffer().to_boxed_bytes().into_box()
    }

    #[inline]
    fn into_max_size_buffer_align_right<H, const MAX_LEN: usize>(
        self,
        buffer: &mut [u8; MAX_LEN],
        h: H,
    ) -> Result<usize, H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        self.to_managed_buffer()
            .into_max_size_buffer_align_right(buffer, h)
    }

    #[inline]
    fn into_i64<H>(self, h: H) -> Result<i64, H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        self.to_managed_buffer().into_i64(h)
    }

    #[inline]
    fn supports_specialized_type<T: TryStaticCast>() -> bool {
        T::type_eq::<ManagedBuffer<A>>() || T::type_eq::<BigUint<A>>() || T::type_eq::<BigInt<A>>()
    }

    #[inline]
    fn into_specialized<T, H>(self, h: H) -> Result<T, H::HandledErr>
    where
        T: TryStaticCast,
        H: DecodeErrorHandler,
    {
        if let Some(result) = try_execute_then_cast(|| self.to_managed_buffer()) {
            Ok(result)
        } else if let Some(result) = try_execute_then_cast(|| self.to_big_uint()) {
            Ok(result)
        } else if let Some(result) = try_execute_then_cast(|| self.to_big_int()) {
            Ok(result)
        } else {
            Err(h.handle_error(DecodeError::UNSUPPORTED_OPERATION))
        }
    }

    fn into_nested_buffer(self) -> Self::NestedBuffer {
        ManagedBufferNestedDecodeInput::new(self.to_managed_buffer())
    }
}

pub fn storage_get_from_address<A, T>(
    addr: ManagedRef<'_, A, ManagedAddress<A>>,
    key: ManagedRef<'_, A, StorageKey<A>>,
) -> T
where
    T: TopDecode,
    A: StorageReadApi + ManagedTypeApi + ErrorApi,
{
    T::top_decode_or_handle_err(
        StorageGetFromAddressInput::new(addr, key),
        StorageGetErrorHandler::<A>::default(),
    )
    .unwrap_infallible()
}

/// Useful for storage mappers.
/// Also calls to it generated by macro.
pub fn storage_get_len_from_address<A>(
    addr: ManagedRef<'_, A, ManagedAddress<A>>,
    key: ManagedRef<'_, A, StorageKey<A>>,
) -> usize
where
    A: StorageReadApi + ManagedTypeApi + ErrorApi,
{
    let input = StorageGetFromAddressInput::new(addr, key);
    input.load_len_managed_buffer()
}
