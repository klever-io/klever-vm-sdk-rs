use crate::{
    api::ManagedTypeApi,
    types::{
        BigUint, KdaTokenPaymentMultiValue, KdaTokenType, ManagedVecItem,
        TokenIdentifier,
    },
};

use super::ManagedVec;

use crate as klever_sc; // needed by the codec and TypeAbi generated code
use crate::{
    codec::{
        self,
        derive::{NestedEncode, TopEncode},
        IntoMultiValue, NestedDecode, TopDecode,
    },
    derive::TypeAbi,
};

#[derive(TopEncode, NestedEncode, TypeAbi, Clone, PartialEq, Eq, Debug)]
pub struct KdaTokenPayment<M: ManagedTypeApi> {
    pub token_identifier: TokenIdentifier<M>,
    pub token_nonce: u64,
    pub amount: BigUint<M>,
}

/// Alias for a list of payments.
pub type MultiKdaPayment<Api> = ManagedVec<Api, KdaTokenPayment<Api>>;

impl<M: ManagedTypeApi> KdaTokenPayment<M> {
    #[inline]
    pub fn new(
        token_identifier: TokenIdentifier<M>,
        token_nonce: u64,
        amount: BigUint<M>,
    ) -> Self {
        KdaTokenPayment {
            token_identifier,
            token_nonce,
            amount,
        }
    }

    pub fn no_payment() -> Self {
        KdaTokenPayment {
            token_identifier: TokenIdentifier::klv(),
            token_nonce: 0,
            amount: BigUint::zero(),
        }
    }

    pub fn token_type(&self) -> KdaTokenType {
            if self.token_nonce == 0 {
                KdaTokenType::Fungible
            } else if self.amount == 1u64 {
                KdaTokenType::NonFungible
            } else  {
                KdaTokenType::SemiFungible
            } 
    }

    #[inline]
    pub fn into_tuple(
        self,
    ) -> (
        TokenIdentifier<M>,
        u64,
        BigUint<M>,
    ) {
        (
            self.token_identifier,
            self.token_nonce,
            self.amount,
        )
    }

    #[inline]
    pub fn into_tuple_with_royalties(
        self,
    ) -> (
        TokenIdentifier<M>,
        u64,
        BigUint<M>,
    ) {
        (
            self.token_identifier,
            self.token_nonce,
            self.amount,
        )
    }
}

impl<M: ManagedTypeApi>
    From<(
        TokenIdentifier<M>,
        u64,
        BigUint<M>,
    )> for KdaTokenPayment<M>
{
    #[inline]
    fn from(
        value: (
            TokenIdentifier<M>,
            u64,
            BigUint<M>,
        ),
    ) -> Self {
        let (token_identifier, token_nonce, amount) = value;
        Self::new(token_identifier, token_nonce, amount)
    }
}

impl<M: ManagedTypeApi> TopDecode for KdaTokenPayment<M> {
    fn top_decode_or_handle_err<I, H>(top_input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: codec::TopDecodeInput,
        H: codec::DecodeErrorHandler,
    {
        let mut nested_buffer = top_input.into_nested_buffer();
        let result = Self::dep_decode_or_handle_err(&mut nested_buffer, h)?;
        if !codec::NestedDecodeInput::is_depleted(&nested_buffer) {
            return Err(h.handle_error(codec::DecodeError::INPUT_TOO_LONG));
        }
        Ok(result)
    }
}

impl<M: ManagedTypeApi> NestedDecode for KdaTokenPayment<M> {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: codec::NestedDecodeInput,
        H: codec::DecodeErrorHandler,
    {
        Self::regular_dep_decode_or_handle_err(input, h)
    }
}

impl<M: ManagedTypeApi> KdaTokenPayment<M> {
    #[doc(hidden)]
    pub fn regular_dep_decode_or_handle_err<I, H>(
        input: &mut I,
        h: H,
    ) -> Result<Self, H::HandledErr>
    where
        I: codec::NestedDecodeInput,
        H: codec::DecodeErrorHandler,
    {
        Ok(KdaTokenPayment {
            token_identifier: TokenIdentifier::<M>::dep_decode_or_handle_err(input, h)?,
            token_nonce: <u64>::dep_decode_or_handle_err(input, h)?,
            amount: BigUint::<M>::dep_decode_or_handle_err(input, h)?,
        })
    }
}

fn managed_vec_item_from_slice<T>(arr: &[u8], index: &mut usize) -> T
where
    T: ManagedVecItem,
{
    ManagedVecItem::from_byte_reader(|bytes| {
        let size = T::PAYLOAD_SIZE;
        bytes.copy_from_slice(&arr[*index..*index + size]);
        *index += size;
    })
}

fn managed_vec_item_to_slice<T>(arr: &mut [u8], index: &mut usize, item: &T)
where
    T: ManagedVecItem,
{
    ManagedVecItem::to_byte_writer(item, |bytes| {
        let size = T::PAYLOAD_SIZE;
        arr[*index..*index + size].copy_from_slice(bytes);
        *index += size;
    });
}

impl<M: ManagedTypeApi> IntoMultiValue for KdaTokenPayment<M> {
    type MultiValue = KdaTokenPaymentMultiValue<M>;

    #[inline]
    fn into_multi_value(self) -> Self::MultiValue {
        self.into()
    }
}

impl<M: ManagedTypeApi> ManagedVecItem for KdaTokenPayment<M> {
    const PAYLOAD_SIZE: usize = 16;
    const SKIPS_RESERIALIZATION: bool = false;
    type Ref<'a> = Self;

    fn from_byte_reader<Reader: FnMut(&mut [u8])>(mut reader: Reader) -> Self {
        let mut arr: [u8; 16] = [0u8; 16];
        reader(&mut arr[..]);
        let mut index = 0;

        let token_identifier = managed_vec_item_from_slice(&arr, &mut index);
        let token_nonce = managed_vec_item_from_slice(&arr, &mut index);
        let amount = managed_vec_item_from_slice(&arr, &mut index);

        KdaTokenPayment {
            token_identifier,
            token_nonce,
            amount,
        }
    }

    unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
        reader: Reader,
    ) -> Self::Ref<'a> {
        Self::from_byte_reader(reader)
    }

    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, mut writer: Writer) -> R {
        let mut arr: [u8; 16] = [0u8; 16];
        let mut index = 0;

        managed_vec_item_to_slice(&mut arr, &mut index, &self.token_identifier);
        managed_vec_item_to_slice(&mut arr, &mut index, &self.token_nonce);
        managed_vec_item_to_slice(&mut arr, &mut index, &self.amount);

        writer(&arr[..])
    }
}
