use crate::value_interpreter::*;
use bech32::FromBase32;
use sha3::{Digest, Keccak256};

pub const SC_ADDRESS_NUM_LEADING_ZEROS: usize = 8;

// Represents the number of zero bytes every smart contract address begins with.
// Its value is 10.
// 10 = 8 zeros for all SC addresses + 2 zeros as placeholder for the VM type.
pub const SC_ADDRESS_RESERVED_PREFIX_LENGTH: usize = SC_ADDRESS_NUM_LEADING_ZEROS + VM_TYPE_LENGTH;

pub fn keccak256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    let hash: [u8; 32] = hasher.finalize().into();
    hash.into()
}

fn create_address_from_prefix(prefix: &[u8], start_index: usize, length: usize) -> Vec<u8> {
    let mut result = Vec::with_capacity(32);
    result.resize(start_index, 0);
    if prefix.len() < length - start_index {
        result.extend_from_slice(prefix);
    } else {
        result.extend_from_slice(&prefix[..length - start_index]);
    }

    while result.len() < length {
        result.push(b'_');
    }
    result
}

/// Generates a 32-byte EOA address based on the input.
pub(crate) fn address_expression(input: &str) -> Vec<u8> {
    create_address_from_prefix(input.as_bytes(), 0, 32)
}

/// Generates a 32-byte smart contract address based on the input.
pub(crate) fn sc_address_expression(input: &str, vm_type: &VMIdentifier) -> Vec<u8> {
    let mut address =
        create_address_from_prefix(input.as_bytes(), SC_ADDRESS_RESERVED_PREFIX_LENGTH, 32);
    let mut vm = vm_type.vm_type;
    address[SC_ADDRESS_RESERVED_PREFIX_LENGTH - VM_TYPE_LENGTH..SC_ADDRESS_RESERVED_PREFIX_LENGTH]
        .swap_with_slice(&mut vm);
    address
}

pub(crate) fn bech32(input: &str) -> Vec<u8> {
    let (_, decoded, _) = bech32::decode(input).expect("bech32 decode error");
    Vec::<u8>::from_base32(&decoded).expect("bech32 base64 decode error")
}
