use klever_sc_meta::ei;

use std::collections::HashSet;

/// Added around November-December 2021.
pub const EI_1_1_ADDED_NAMES: &[&str] = &[
    "mBufferSetByteSlice",
    "managedSha256",
    "managedKeccak256",
    "mBufferStorageLoadFromAddress",
    "validateTokenIdentifier",
    "cleanReturnData",
    "deleteFromReturnData",
];

/// Added around May 2022.
pub const EI_1_2_ADDED_NAMES: &[&str] = &[
    // debugging/display utilities
    "bigIntToString",
    "managedBufferToHex",
    // more managed crypto functions
    "managedRipemd160",
    "managedVerifyBLS",
    "managedVerifyEd25519",
    "managedVerifySecp256k1",
    "managedVerifyCustomSecp256k1",
    "managedEncodeSecp256k1DerSignature",
    "managedScalarBaseMultEC",
    "managedScalarMultEC",
    "managedMarshalEC",
    "managedUnmarshalEC",
    "managedMarshalCompressedEC",
    "managedUnmarshalCompressedEC",
    "managedGenerateKeyEC",
    "managedCreateEC",
    // big floats
    "mBufferToBigFloat",
    "mBufferFromBigFloat",
    "bigFloatNewFromParts",
    "bigFloatNewFromFrac",
    "bigFloatNewFromSci",
    "bigFloatAdd",
    "bigFloatSub",
    "bigFloatMul",
    "bigFloatDiv",
    "bigFloatNeg",
    "bigFloatClone",
    "bigFloatCmp",
    "bigFloatAbs",
    "bigFloatSign",
    "bigFloatSqrt",
    "bigFloatPow",
    "bigFloatFloor",
    "bigFloatCeil",
    "bigFloatTruncate",
    "bigFloatSetInt64",
    "bigFloatIsInt",
    "bigFloatSetBigInt",
    "bigFloatGetConstPi",
    "bigFloatGetConstE",
];

/// Planned to be released with VM 1.5.
pub const EI_1_3_ADDED_NAMES: &[&str] = &[
    "managedGetBackTransfers",
    "managedAccHasPerm",
    "managedGetMultiKDAWithoutKLVCallValue",
    "managedGetCodeMetadata",
    "managedGetSftMetadata",
    "managedGetUserKDA",
    "managedGetKDARoles",
    "managedIsBuiltinFunction",
];

fn list_to_set<'a>(list: &[&'a str]) -> HashSet<&'a str> {
    let mut set = HashSet::new();
    for &item in list {
        assert!(!set.contains(item), "duplicate item: {item}");
        set.insert(item);
    }
    set
}

fn test_added_names(base: &[&str], added: &[&str], expected_result: &[&str]) {
    let mut check = list_to_set(base);
    for &added_name in added {
        assert!(
            !check.contains(added_name),
            "added name already present: {added_name}"
        );
        check.insert(added_name);
    }
    let mut actual_sorted: Vec<_> = check.into_iter().collect();
    let mut expected_sorted: Vec<_> = list_to_set(expected_result).into_iter().collect();

    actual_sorted.sort();
    expected_sorted.sort();

    assert_eq!(actual_sorted, expected_sorted);
}

#[test]
fn test_added_names_ei_1_1() {
    test_added_names(ei::EI_1_0_NAMES, EI_1_1_ADDED_NAMES, ei::EI_1_1_NAMES);
}

#[test]
fn test_added_names_ei_1_2() {
    test_added_names(ei::EI_1_1_NAMES, EI_1_2_ADDED_NAMES, ei::EI_1_2_NAMES);
}

#[test]
fn test_added_names_ei_1_3() {
    test_added_names(ei::EI_1_2_NAMES, EI_1_3_ADDED_NAMES, ei::EI_1_3_NAMES);
}
