use super::KDALocalRoleFlags;
use crate::codec::{
    self,
    derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
};

const KDA_ROLE_NONE: &str = "";
const KDA_ROLE_MINT: &str = "KDARoleMint";
const KDA_ROLE_SET_ITO_PRICES: &str = "KDARoleSetItoPrices";
const KDA_ROLE_DEPOSIT: &str = "KDARoleDeposit";
const KDA_ROLE_TRANSFER: &str = "KDARoleTransfer";

#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, Clone, PartialEq, Eq, Debug, Copy)]
pub enum KDALocalRole {
    None,
    Mint,
    SetITOPrices,
    Deposit,
    Transfer,
}

impl KDALocalRole {
    pub fn as_u16(&self) -> u16 {
        match self {
            Self::None => 0,
            Self::Mint => 1,
            Self::SetITOPrices => 2,
            Self::Deposit => 3,
            Self::Transfer => 4,
        }
    }

    pub fn as_role_name(&self) -> &'static [u8] {
        self.name().as_bytes()
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::None => KDA_ROLE_NONE,
            Self::Mint => KDA_ROLE_MINT,
            Self::SetITOPrices => KDA_ROLE_SET_ITO_PRICES,
            Self::Deposit => KDA_ROLE_DEPOSIT,
            Self::Transfer => KDA_ROLE_TRANSFER,
        }
    }

    pub fn to_flag(&self) -> KDALocalRoleFlags {
        match self {
            Self::None => KDALocalRoleFlags::NONE,
            Self::Mint => KDALocalRoleFlags::MINT,
            Self::SetITOPrices => KDALocalRoleFlags::SET_ITO_PRICES,
            Self::Deposit => KDALocalRoleFlags::DEPOSIT,
            Self::Transfer => KDALocalRoleFlags::TRANSFER,
        }
    }
}

// TODO: can be done with macros, but I didn't find a public library that does it and is no_std
// we can implement it, it's easy
const ALL_ROLES: [KDALocalRole; 4] = [
    KDALocalRole::Mint,
    KDALocalRole::SetITOPrices,
    KDALocalRole::Deposit,
    KDALocalRole::Transfer,
];

impl KDALocalRole {
    pub fn iter_all() -> core::slice::Iter<'static, KDALocalRole> {
        ALL_ROLES.iter()
    }
}

impl From<u16> for KDALocalRole {
    #[inline]
    fn from(value: u16) -> Self {
        match value {
            1 => Self::Mint,
            2 => Self::SetITOPrices,
            3 => Self::Deposit,
            4 => Self::Transfer,
            _ => Self::None,
        }
    }
}

impl<'a> From<&'a [u8]> for KDALocalRole {
    #[inline]
    fn from(byte_slice: &'a [u8]) -> Self {
        if byte_slice == KDA_ROLE_MINT.as_bytes() {
            Self::Mint
        } else if byte_slice == KDA_ROLE_SET_ITO_PRICES.as_bytes() {
            Self::SetITOPrices
        } else if byte_slice == KDA_ROLE_DEPOSIT.as_bytes() {
            Self::Deposit
        } else if byte_slice == KDA_ROLE_TRANSFER.as_bytes() {
            Self::Transfer
        } else {
            Self::None
        }
    }
}
