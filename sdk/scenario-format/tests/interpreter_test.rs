use klever_chain_scenario_format::{
    interpret_trait::InterpreterContext, value_interpreter::interpret_string,
};

const EMPTY: Vec<u8> = Vec::<u8>::new();

#[test]
fn test_bool() {
    let context = InterpreterContext::default();
    assert_eq!(vec![1], interpret_string("true", &context));
    assert_eq!(EMPTY, interpret_string("false", &context));
}

#[test]
fn test_string() {
    let context = InterpreterContext::default();
    assert_eq!(b"abcdefg".to_vec(), interpret_string("``abcdefg", &context));
    assert_eq!(EMPTY, interpret_string("``", &context));
    assert_eq!(b"`".to_vec(), interpret_string("```", &context));
    assert_eq!(b" ".to_vec(), interpret_string("`` ", &context));

    assert_eq!(b"abcdefg".to_vec(), interpret_string("''abcdefg", &context));
    assert_eq!(EMPTY, interpret_string("''", &context));
    assert_eq!(b"'".to_vec(), interpret_string("'''", &context));
    assert_eq!(b"``".to_vec(), interpret_string("''``", &context));

    assert_eq!(
        b"abcdefg".to_vec(),
        interpret_string("str:abcdefg", &context)
    );
    assert_eq!(EMPTY, interpret_string("str:", &context));
}

#[test]
fn test_address() {
    let context = InterpreterContext::default();
    assert_eq!(
        b"________________________________".to_vec(),
        interpret_string("address:", &context)
    );
    assert_eq!(
        b"a_______________________________".to_vec(),
        interpret_string("address:a", &context)
    );
    assert_eq!(
        b"a\x05______________________________".to_vec(),
        interpret_string("address:a\x05", &context)
    );
    assert_eq!(
        b"an_address______________________".to_vec(),
        interpret_string("address:an_address", &context)
    );
    assert_eq!(
        b"12345678901234567890123456789012".to_vec(),
        interpret_string("address:12345678901234567890123456789012", &context)
    );
    assert_eq!(
        b"12345678901234567890123456789012".to_vec(),
        interpret_string("address:123456789012345678901234567890123", &context)
    );
}

#[test]
fn test_sc_address() {
    let context = InterpreterContext::default();
    assert_eq!(
        b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00a_____________________".to_vec(),
        interpret_string("sc:a", &context)
    );
    assert_eq!(
        b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x001234567890123456789012".to_vec(),
        interpret_string("sc:12345678901234567890120s", &context)
    );
    // trims excess
    assert_eq!(
        b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x001234567890123456789012".to_vec(),
        interpret_string("sc:12345678901234567890120sx", &context)
    );
}

#[test]
fn test_unsigned_number() {
    let context = InterpreterContext::default();
    assert_eq!(vec![0x12, 0x34], interpret_string("0x1234", &context));
    assert_eq!(vec![0x00], interpret_string("0x0", &context));
    assert_eq!(vec![0x00], interpret_string("0x00", &context));
    assert_eq!(vec![0x00, 0x00], interpret_string("0x000", &context));
    assert_eq!(vec![0x00, 0x00], interpret_string("0x0000", &context));
    assert_eq!(
        vec![0x00, 0x00, 0xab],
        interpret_string("0x0000ab", &context)
    );
    assert_eq!(EMPTY, interpret_string("0x", &context));
    assert_eq!(EMPTY, interpret_string("0", &context));
    assert_eq!(vec![12], interpret_string("12", &context));
    assert_eq!(vec![0x01, 0x00], interpret_string("256", &context));
    assert_eq!(vec![0x01], interpret_string("0b1", &context));
    assert_eq!(vec![0x05], interpret_string("0b101", &context));
}

#[test]
fn test_signed_number() {
    let context = InterpreterContext::default();
    assert_eq!(vec![0xff], interpret_string("-1", &context));
    assert_eq!(vec![0xff], interpret_string("255", &context));
    assert_eq!(vec![0xff], interpret_string("0xff", &context));
    assert_eq!(vec![0x00, 0xff], interpret_string("+255", &context));
    assert_eq!(vec![0x00, 0xff], interpret_string("+0xff", &context));

    assert_eq!(vec![0xff, 0x00], interpret_string("-256", &context));
    assert_eq!(vec![0xfb], interpret_string("-0b101", &context));
}

#[test]
fn test_unsigned_fixed_width() {
    let context = InterpreterContext::default();
    assert_eq!(vec![0x00], interpret_string("u8:0", &context));
    assert_eq!(vec![0x00, 0x00], interpret_string("u16:0", &context));
    assert_eq!(
        vec![0x00, 0x00, 0x00, 0x00],
        interpret_string("u32:0", &context)
    );
    assert_eq!(
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        interpret_string("u64:0", &context)
    );
    assert_eq!(vec![0x12, 0x34], interpret_string("u16:0x1234", &context));
    assert_eq!(
        vec![0x00, 0x00, 0x12, 0x34],
        interpret_string("u32:0x1234", &context)
    );
    assert_eq!(vec![0x01, 0x00], interpret_string("u16:256", &context));
    assert_eq!(vec![0x01], interpret_string("u8:0b1", &context));
    assert_eq!(
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05],
        interpret_string("u64:0b101", &context)
    );
}

#[test]
fn test_signed_fixed_width() {
    let context = InterpreterContext::default();
    assert_eq!(vec![0x00], interpret_string("i8:0", &context));
    assert_eq!(vec![0x00, 0x00], interpret_string("i16:0", &context));
    assert_eq!(
        vec![0x00, 0x00, 0x00, 0x00],
        interpret_string("i32:0", &context)
    );
    assert_eq!(
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        interpret_string("i64:0", &context)
    );

    assert_eq!(vec![0xff], interpret_string("i8:-1", &context));
    assert_eq!(vec![0xff, 0xff], interpret_string("i16:-1", &context));
    assert_eq!(
        vec![0xff, 0xff, 0xff, 0xff],
        interpret_string("i32:-1", &context)
    );
    assert_eq!(
        vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
        interpret_string("i64:-1", &context)
    );
    assert_eq!(
        vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00],
        interpret_string("i64:-256", &context)
    );
    assert_eq!(vec![0xfb], interpret_string("i8:-0b101", &context));
}

#[test]
#[should_panic]
fn test_signed_fixed_width_panic_1() {
    let context = InterpreterContext::default();
    interpret_string("i8:+255", &context);
}

#[test]
#[should_panic]
fn test_signed_fixed_width_panic_2() {
    let context = InterpreterContext::default();
    interpret_string("i8:0xff", &context);
}

#[test]
#[should_panic]
fn test_signed_fixed_width_panic_3() {
    let context = InterpreterContext::default();
    interpret_string("i8:-255", &context);
}

#[test]
fn test_biguint_nested() {
    let context = InterpreterContext::default();
    assert_eq!(
        vec![0x00, 0x00, 0x00, 0x00],
        interpret_string("biguint:0", &context)
    );
    assert_eq!(
        vec![0x00, 0x00, 0x00, 0x01, 0x01],
        interpret_string("biguint:1", &context)
    );
    assert_eq!(
        vec![0x00, 0x00, 0x00, 0x02, 0x01, 0xFF],
        interpret_string("biguint:0x01FF", &context)
    );

    assert_eq!(
        vec![0x00, 0x00, 0x00, 0x05, 0x01, 0x02, 0x03, 0x04, 0x05],
        interpret_string("biguint:0x0102030405", &context)
    );

    assert_eq!(
        vec![0x00, 0x00, 0x00, 0x05, 0x01, 0x02, 0x03, 0x04, 0x05],
        interpret_string("nested:0x0102030405", &context)
    );

    assert_eq!(
        vec![0x00, 0x00, 0x00, 0x01, 0xFF],
        interpret_string("nested:-1", &context)
    );
}

#[test]
#[should_panic]
fn test_biguint_nested_neg() {
    let context = InterpreterContext::default();
    interpret_string("biguint:-1", &context);
}

#[test]
fn test_bech32() {
    let context = InterpreterContext::default();
    // alice
    assert_eq!(
        hex::decode("485d212a35410fdef731419f9380a9a2984d885388a807c297d3c2cb2c467cde").unwrap(),
        interpret_string(
            "bech32:klv1fpwjz234gy8aaae3gx0e8q9f52vymzzn3z5q0s5h60pvktzx0n0qwvtux5",
            &context
        )
    );

    // system SC
    assert_eq!(
        hex::decode("0000000000000000050045f737bf95f66403cad1f77d53d613f7791e827ac3e6").unwrap(),
        interpret_string(
            "bech32:klv1qqqqqqqqqqqqqpgqghmn00u47ejq8jk37a7484sn7au3aqn6c0nqqtwsgw",
            &context
        )
    );
}
