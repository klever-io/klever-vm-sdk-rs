use klever_sc::{
    hex_literal::hex,
    types::{
        BigInt, BigUint, ManagedAddress, ManagedBuffer, ManagedByteArray, ManagedVec,
        TokenIdentifier,
    },
};
use klever_sc_scenario::api::StaticApi;

#[test]
fn test_big_uint_format() {
    let s = format!("{:?}", BigUint::<StaticApi>::from(0x1234u32));
    assert_eq!("BigUint { handle: -100, hex-value-be: \"1234\" }", s);
}

#[test]
fn test_big_int_format_1() {
    let s = format!("{:?}", BigInt::<StaticApi>::from(0x1234));
    assert_eq!("BigInt { handle: -100, hex-value-be: \"1234\" }", s);
}

#[test]
fn test_big_int_format_2() {
    let s = format!("{:?}", BigInt::<StaticApi>::from(-0x1234));
    assert_eq!("BigInt { handle: -100, hex-value-be: \"edcc\" }", s);
}

#[test]
fn test_managed_buffer() {
    let _ = klever_sc::hex_literal::hex!("abcd");
    let s = format!("{:?}", ManagedBuffer::<StaticApi>::from(&[0x12, 0x34]));
    assert_eq!("ManagedBuffer { handle: -100, hex-value: \"1234\" }", s);
}

#[test]
fn test_managed_byte_array() {
    let addr = hex!("01020304050607");
    let s = format!("{:?}", ManagedByteArray::<StaticApi, 7>::from(&addr));
    assert_eq!(
        "ManagedByteArray { handle: -100, size: 7, hex-value: \"01020304050607\" }",
        s
    );
}

#[test]
fn test_managed_address() {
    let addr = hex!("000000000000000000010000000000000000000000000000000000000002ffff");
    let s = format!("{:?}", ManagedAddress::<StaticApi>::from(&addr));
    assert_eq!("ManagedAddress { handle: -100, hex-value: \"000000000000000000010000000000000000000000000000000000000002ffff\" }", s);
}

#[test]
fn test_managed_address_pretty() {
    let addr = hex!("000000000000000000010000000000000000000000000000000000000002ffff");
    let s = format!("{:#?}", ManagedAddress::<StaticApi>::from(&addr));
    assert_eq!(
        "ManagedAddress {
    handle: -100,
    hex-value: \"000000000000000000010000000000000000000000000000000000000002ffff\",
}",
        s
    );
}

#[test]
fn test_managed_vec_format_biguint() {
    let mut mv = ManagedVec::<StaticApi, BigUint<StaticApi>>::new();
    mv.push(BigUint::from(1u32));
    mv.push(BigUint::from(2u32));
    let s = format!("{:?}", &mv);
    assert_eq!("[BigUint { handle: -101, hex-value-be: \"01\" }, BigUint { handle: -102, hex-value-be: \"02\" }]", s);
}

#[test]
fn test_managed_vec_format_klv_or_kda() {
    let mut mv = ManagedVec::<StaticApi, TokenIdentifier<StaticApi>>::new();
    mv.push(TokenIdentifier::klv());
    mv.push(TokenIdentifier::from("MYTOKEN-5678"));
    let s = format!("{:?}", &mv);
    assert_eq!(
        "[TokenIdentifier(\"KLV\"), TokenIdentifier(\"MYTOKEN-5678\")]",
        s
    );
}
