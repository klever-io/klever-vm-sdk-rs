use klever_sc_derive::ManagedVecItem;

use crate::{
    codec,
    codec::derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
};

const KDA_TYPE_FUNGIBLE: &[u8] = b"FungibleKDA";
const KDA_TYPE_NON_FUNGIBLE: &[u8] = b"NonFungibleKDA";
const KDA_TYPE_SEMI_FUNGIBLE: &[u8] = b"SemiFungibleKDA";
const KDA_TYPE_INVALID: &[u8] = &[];

use crate as klever_sc; // needed by the TypeAbi generated code
use crate::derive::type_abi;

// Note: In the current implementation, SemiFungible is never returned

#[type_abi]
#[derive(
    TopDecode, TopEncode, NestedDecode, NestedEncode, Clone, PartialEq, Eq, Debug, ManagedVecItem,
)]
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
