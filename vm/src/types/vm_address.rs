use super::H256;

use core::fmt::Debug;

const SC_ADDRESS_NUM_LEADING_ZEROS: u8 = 8;

pub const NUM_INT_CHARACTERS_FOR_ADDRESS: usize = 10;
pub const VM_TYPE_LEN: usize = 2;
pub const DEFAULT_VM_TYPE: &[u8] = &[0, 0];

/// Address type being used in the VM only.
///
/// Its implementation is similar to that of the heap Address in the framework,
/// but we have a separate implementation for the VM, because it is a separate component.
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct VMAddress(H256);

impl VMAddress {
    pub const fn new(bytes: [u8; 32]) -> Self {
        VMAddress(H256::new(bytes))
    }

    pub fn generate_mock_address(creator_address: &[u8], creator_nonce: u64) -> Self {
        let mut result = [0x00; 32];

        result[10] = 0x11;
        result[11] = 0x11;
        result[12] = 0x11;
        result[13] = 0x11;

        result[14..29].copy_from_slice(&creator_address[..15]);
        result[29] = creator_nonce as u8;
        result[30..].copy_from_slice(&creator_address[30..]);

        let start_index = NUM_INT_CHARACTERS_FOR_ADDRESS - VM_TYPE_LEN;
        result[start_index..(start_index + DEFAULT_VM_TYPE.len())].copy_from_slice(DEFAULT_VM_TYPE);

        VMAddress::from(result)
    }
}

impl From<H256> for VMAddress {
    fn from(hash: H256) -> Self {
        VMAddress(hash)
    }
}

impl From<VMAddress> for H256 {
    fn from(address: VMAddress) -> Self {
        address.0
    }
}

impl<'a> From<&'a VMAddress> for &'a H256 {
    fn from(address: &'a VMAddress) -> Self {
        &address.0
    }
}

impl From<[u8; 32]> for VMAddress {
    fn from(arr: [u8; 32]) -> Self {
        VMAddress(H256::from(arr))
    }
}

impl From<&[u8; 32]> for VMAddress {
    fn from(bytes: &[u8; 32]) -> Self {
        VMAddress(H256::from(bytes))
    }
}

impl From<&mut [u8; 32]> for VMAddress {
    fn from(bytes: &mut [u8; 32]) -> Self {
        VMAddress(H256::from(bytes))
    }
}

impl From<Box<[u8; 32]>> for VMAddress {
    fn from(bytes: Box<[u8; 32]>) -> Self {
        VMAddress(H256::from(bytes))
    }
}

impl VMAddress {
    pub fn from_slice(slice: &[u8]) -> Self {
        VMAddress(H256::from_slice(slice))
    }
}

impl From<VMAddress> for [u8; 32] {
    fn from(addr: VMAddress) -> Self {
        addr.0.into()
    }
}

impl AsRef<[u8]> for VMAddress {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsMut<[u8]> for VMAddress {
    fn as_mut(&mut self) -> &mut [u8] {
        self.0.as_mut()
    }
}

impl VMAddress {
    /// Returns a new address of 32 zeros.
    /// Allocates directly in heap.
    /// Minimal resulting wasm code (14 bytes if not inlined).
    pub fn zero() -> Self {
        VMAddress(H256::zero())
    }

    /// Extracts a byte slice containing the entire fixed hash.
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn as_array(&self) -> &[u8; 32] {
        self.0.as_array()
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn is_smart_contract_address(&self) -> bool {
        self.as_bytes()
            .iter()
            .take(SC_ADDRESS_NUM_LEADING_ZEROS.into())
            .all(|item| item == &0u8)
    }
}

#[cfg(test)]
mod tests {
    use crate::{display_util::address_hex, types::VMAddress};

    #[test]
    fn generate_mock_address_test() {
        let creator_address = VMAddress::new([
            111, 119, 110, 101, 114, 95, 95, 95, 95, 95, 95, 95, 95, 95, 95, 95, 95, 95, 95, 95,
            95, 95, 95, 95, 95, 95, 95, 95, 95, 95, 95, 95,
        ]);
        let mock_address = VMAddress::generate_mock_address(&creator_address.to_vec(), 1u64);
        assert_eq!(
            address_hex(&mock_address),
            "0x00000000000000000000111111116f776e65725f5f5f5f5f5f5f5f5f5f015f5f"
        );
    }
}
