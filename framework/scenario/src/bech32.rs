use bech32::{FromBase32, ToBase32, Variant};
use klever_sc::types::heap::Address;

pub fn decode(bech32_address: &str) -> Address {
    let (_, dest_address_bytes_u5, _) = bech32::decode(bech32_address)
        .unwrap_or_else(|err| panic!("bech32 decode error for {bech32_address}: {err}"));
    let dest_address_bytes = Vec::<u8>::from_base32(&dest_address_bytes_u5).unwrap();
    if dest_address_bytes.len() != 32 {
        panic!("Invalid address length after decoding")
    }

    Address::from_slice(&dest_address_bytes)
}

pub fn encode(address: &Address) -> String {
    bech32::encode("klv", address.as_bytes().to_base32(), Variant::Bech32)
        .expect("bech32 encode error")
}
