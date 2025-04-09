use klever_sc::types::{BoxedBytes, KdaTokenPayment, ManagedBuffer, TokenIdentifier};
use klever_sc_scenario::{
    api::StaticApi, klever_sc, managed_klv_token_id,
    managed_test_util::check_managed_top_encode_decode, managed_token_id, managed_token_id_wrapped,
};

#[test]
fn test_klv() {
    assert!(TokenIdentifier::<StaticApi>::klv().is_klv());
}

#[test]
fn test_codec() {
    check_managed_top_encode_decode(
        TokenIdentifier::<StaticApi>::klv(),
        TokenIdentifier::<StaticApi>::KLV_REPRESENTATION,
    );

    let expected = BoxedBytes::from_concat(&[
        &[0, 0, 0, 3],
        &TokenIdentifier::<StaticApi>::KLV_REPRESENTATION[..],
    ]);
    check_managed_top_encode_decode(
        vec![TokenIdentifier::<StaticApi>::klv()],
        expected.as_slice(),
    );
}

#[test]
#[rustfmt::skip]
fn test_is_valid_kda_identifier() {
    // valid identifier
    assert!(TokenIdentifier::<StaticApi>::from("ALC-6258").is_valid_kda_identifier());

    // valid identifier with numbers in ticker
    assert!(TokenIdentifier::<StaticApi>::from("ALC123-6258").is_valid_kda_identifier());

    // valid ticker only numbers
    assert!(TokenIdentifier::<StaticApi>::from("12345-6258").is_valid_kda_identifier());

    // missing dash
    assert!(!TokenIdentifier::<StaticApi>::from("ALC6258").is_valid_kda_identifier());

    // wrong dash position
    assert!(!TokenIdentifier::<StaticApi>::from("AL-C6258").is_valid_kda_identifier());

    // lowercase ticker
    assert!(!TokenIdentifier::<StaticApi>::from("alc-6258").is_valid_kda_identifier());

    // lowercase random chars
    assert!(!TokenIdentifier::<StaticApi>::from("ALC-62d2").is_valid_kda_identifier());

    // too many random chars
    assert!(!TokenIdentifier::<StaticApi>::from("ALC-6258D2").is_valid_kda_identifier());

    // ticker too short
    assert!(!TokenIdentifier::<StaticApi>::from("AL-6258").is_valid_kda_identifier());

    // ticker too long
    assert!(!TokenIdentifier::<StaticApi>::from("ALCCCCCCCCC-6258").is_valid_kda_identifier());
}

#[test]
#[rustfmt::skip]
fn test_ticker() {
    // valid identifier
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("ALC-6258").ticker(),
        ManagedBuffer::<StaticApi>::from("ALC"),
    );

    // valid identifier with numbers in ticker
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("ALC123-6258").ticker(),
        ManagedBuffer::<StaticApi>::from("ALC123"),
    );

    // valid ticker only numbers
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("12345-6258").ticker(),
        ManagedBuffer::<StaticApi>::from("12345"),
    );

    // missing dash
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("ALC6258").ticker(),
        ManagedBuffer::<StaticApi>::from("AL"),
    );

    // wrong dash position
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("AL-C6258").ticker(),
        ManagedBuffer::<StaticApi>::from("AL-"),
    );

    // lowercase ticker
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("alc-6258").ticker(),
        ManagedBuffer::<StaticApi>::from("alc"),
    );

    // uppercase random chars
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("ALC-6258").ticker(),
        ManagedBuffer::<StaticApi>::from("ALC"),
    );

    // ticker too short
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("AL-6258").ticker(),
        ManagedBuffer::<StaticApi>::from("AL"),
    );

    // ticker too long
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("ALCCCCCCCCC-6258").ticker(),
        ManagedBuffer::<StaticApi>::from("ALCCCCCCCCC"),
    );
}

#[test]
fn test_is_valid_klv_or_kda() {
    // klv is always valid
    assert!(TokenIdentifier::<StaticApi>::klv().is_valid());

    // valid kda
    assert!(TokenIdentifier::<StaticApi>::from("ALC-6258").is_valid());

    // invalid kda, see above
    assert!(!TokenIdentifier::<StaticApi>::from("ALCCCCCCCCC-6258").is_valid());
}

#[test]
fn test_token_identifier_eq() {
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("KDA-0000"),
        TokenIdentifier::<StaticApi>::from("KDA-0000")
    );
    assert_ne!(
        TokenIdentifier::<StaticApi>::from("KDA-0001"),
        TokenIdentifier::<StaticApi>::from("KDA-0002")
    );

    assert_eq!(
        TokenIdentifier::from("KDA-0003"),
        TokenIdentifier::<StaticApi>::from("KDA-0003")
    );
    assert_ne!(
        TokenIdentifier::<StaticApi>::klv(),
        TokenIdentifier::<StaticApi>::from("ANYTHING-1234")
    );
    assert_eq!(
        TokenIdentifier::<StaticApi>::klv(),
        TokenIdentifier::<StaticApi>::from("KLV")
    );
}

#[test]
fn test_payment_eq() {
    assert_eq!(
        KdaTokenPayment::<StaticApi>::new("PAY-0000".into(), 0, 1000u32.into()),
        KdaTokenPayment::<StaticApi>::new("PAY-0000".into(), 0, 1000u32.into()),
    );
    assert_ne!(
        KdaTokenPayment::<StaticApi>::new("PAY-0001".into(), 0, 1000u32.into()),
        KdaTokenPayment::<StaticApi>::new("PAY-0002".into(), 0, 1000u32.into()),
    );
    assert_eq!(
        KdaTokenPayment::<StaticApi>::no_payment(),
        KdaTokenPayment::<StaticApi>::no_payment(),
    );
    assert_eq!(
        KdaTokenPayment::<StaticApi>::new(TokenIdentifier::from("KDAPAY-0000"), 0, 1000u32.into(),),
        KdaTokenPayment::<StaticApi>::new(TokenIdentifier::from("KDAPAY-0000"), 0, 1000u32.into(),),
    );
    assert_ne!(
        KdaTokenPayment::<StaticApi>::new(TokenIdentifier::from("KDAPAY-0001"), 0, 1000u32.into(),),
        KdaTokenPayment::<StaticApi>::new(TokenIdentifier::from("KDAPAY-0002"), 0, 1000u32.into(),),
    );
    assert_ne!(
        KdaTokenPayment::<StaticApi>::new(TokenIdentifier::from("KDAPAY-0001"), 0, 1000u32.into(),),
        KdaTokenPayment::<StaticApi>::no_payment(),
    );
}

#[test]
fn test_managed_token_id_macro() {
    assert_eq!(managed_klv_token_id!(), TokenIdentifier::<StaticApi>::klv());
    assert_eq!(
        managed_token_id!(b"ALC-6258"),
        TokenIdentifier::<StaticApi>::from("ALC-6258")
    );
    assert_eq!(
        managed_token_id_wrapped!(b"ALC-6258"),
        TokenIdentifier::<StaticApi>::from("ALC-6258")
    )
}
