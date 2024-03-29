use crate::codec::*;

const KDA_TYPE_FUNGIBLE: &[u8] = b"FungibleKDA";
const KDA_TYPE_NON_FUNGIBLE: &[u8] = b"NonFungibleKDA";
const KDA_TYPE_SEMI_FUNGIBLE: &[u8] = b"SemiFungibleKDA";
const KDA_TYPE_INVALID: &[u8] = &[];

use crate as klever_sc; // needed by the TypeAbi generated code
use crate::derive::TypeAbi;

// Note: In the current implementation, SemiFungible is never returned
#[derive(Clone, PartialEq, Eq, Debug, TypeAbi)]
pub enum KdaTokenType {
    Fungible,
    NonFungible,
    SemiFungible,
    Invalid,
}

impl KdaTokenType {
    pub fn based_on_token_nonce(token_nonce: u64) -> Self {
        if token_nonce == 0 {
            KdaTokenType::Fungible
        } else {
            KdaTokenType::NonFungible
        }
    }

    pub fn as_u8(&self) -> u8 {
        match self {
            Self::Fungible => 0,
            Self::NonFungible => 1,
            Self::SemiFungible => 2,
            Self::Invalid => 3,
        }
    }

    pub fn as_type_name(&self) -> &'static [u8] {
        match self {
            Self::Fungible => KDA_TYPE_FUNGIBLE,
            Self::NonFungible => KDA_TYPE_NON_FUNGIBLE,
            Self::SemiFungible => KDA_TYPE_SEMI_FUNGIBLE,
            Self::Invalid => KDA_TYPE_INVALID,
        }
    }
}

impl From<u8> for KdaTokenType {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Fungible,
            1 => Self::NonFungible,
            2 => Self::SemiFungible,
            _ => Self::Fungible,
        }
    }
}

impl<'a> From<&'a [u8]> for KdaTokenType {
    #[inline]
    fn from(byte_slice: &'a [u8]) -> Self {
        if byte_slice == KDA_TYPE_FUNGIBLE {
            Self::Fungible
        } else if byte_slice == KDA_TYPE_NON_FUNGIBLE {
            Self::NonFungible
        } else if byte_slice == KDA_TYPE_SEMI_FUNGIBLE {
            Self::SemiFungible
        } else {
            Self::Invalid
        }
    }
}

impl NestedEncode for KdaTokenType {
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.as_u8().dep_encode_or_handle_err(dest, h)
    }
}

impl TopEncode for KdaTokenType {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.as_u8().top_encode_or_handle_err(output, h)
    }
}

impl NestedDecode for KdaTokenType {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(Self::from(u8::dep_decode_or_handle_err(input, h)?))
    }
}

impl TopDecode for KdaTokenType {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(Self::from(u8::top_decode_or_handle_err(input, h)?))
    }
}
