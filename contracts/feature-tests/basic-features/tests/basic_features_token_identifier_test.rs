use klever_sc::types::{ManagedBuffer, TokenIdentifier};
use klever_sc_scenario::{api::StaticApi, *};

use basic_features::token_identifier_features::TokenIdentifierFeatures;

#[test]
fn test_token_identifier_klv() {
    let bf = basic_features::contract_obj::<StaticApi>();
    let result = bf.token_identifier_klv();
    assert_eq!(TokenIdentifier::klv(), result);
}

/// This just tests the contract syntax.
/// For a complete suite of test cases, see `klever-sc-scenario/tests/managed_token_identifier_test.rs`.
#[test]
fn test_token_identifier_is_valid() {
    let bf = basic_features::contract_obj::<StaticApi>();
    let result = bf.token_identifier_is_valid_1(TokenIdentifier::from(&b"ALC-6258"[..]));
    assert!(result);
    let result = bf.token_identifier_is_valid_1(TokenIdentifier::from(&b"AL-C6258"[..]));
    assert!(!result);
    let result = bf.token_identifier_is_valid_2(ManagedBuffer::from(&b"12345-6258"[..]));
    assert!(result);
    let result = bf.token_identifier_is_valid_2(ManagedBuffer::from(&b"ALCCCCCCCCC-6258"[..]));
    assert!(!result);
}

#[test]
fn test_token_identifier_compare() {
    let token_id = TokenIdentifier::<StaticApi>::from(&b"ALC-6258d2"[..]);
    let kda_token_id = token_id.clone();
    let wrong_kda_token_id = TokenIdentifier::from(&b"AAA-111111"[..]);
    let klv_token_id = TokenIdentifier::klv();

    assert!(token_id == kda_token_id);
    assert!(kda_token_id == token_id);

    assert!(token_id != klv_token_id);
    assert!(klv_token_id != token_id);

    assert!(token_id != wrong_kda_token_id);
    assert!(wrong_kda_token_id != token_id);
}
