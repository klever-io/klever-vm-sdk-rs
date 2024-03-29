use crowdfunding::crowdfunding_helpers::{name_to_id, discount_percentage_fee};
use klever_sc::types::{BigUint, ManagedBuffer};
use klever_sc_scenario::api::StaticApi;

#[test]
fn test_name_to_id() {
    let buffer = &ManagedBuffer::<StaticApi>::from("hello world");
    let converted = name_to_id(buffer);
    let expected = ManagedBuffer::<StaticApi>::from("hello-world");

    assert_eq!(expected, converted);

    let buffer = &ManagedBuffer::<StaticApi>::from("my BIG Cause Is-this!!!@@@");
    let converted = name_to_id(buffer);
    let expected = ManagedBuffer::<StaticApi>::from("my-big-cause-is-this");

    assert_eq!(expected, converted);
}

#[test]
fn test_discount_percentage_fee() {
    let amount = &BigUint::<StaticApi>::from(1000u32);
    let fee = 6u32;
    let converted = discount_percentage_fee(amount, fee);
    let expected = &BigUint::<StaticApi>::from(1000 - 60u32);

    assert_eq!(expected.to_u64(), converted.to_u64());

    let amount = &BigUint::<StaticApi>::from(1000u32);
    let fee = 0u32;
    let converted = discount_percentage_fee(amount, fee);
    let expected = &BigUint::<StaticApi>::from(1000u32);

    assert_eq!(expected.to_u64(), converted.to_u64());
}