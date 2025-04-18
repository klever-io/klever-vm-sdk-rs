/// New hooks added in Q2 2022. This is the EI version of VM 1.4.
///
/// This is the version currently on mainnet.
///
/// Added:
/// - more managed type conversions
/// - more managed crypto hooks
/// - big floats
/// - some managed KDA properties.
///
/// This list of hooks is reconstructed from history.
pub const EI_1_2_NAMES: &[&str] = &[
    "getGasLeft",
    "getSCAddress",
    "getOwnerAddress",
    "isSmartContract",
    "signalError",
    "getExternalBalance",
    "getBlockHash",
    "getKDABalance",
    "getKDANameLength",
    "getKDAURILength",
    "getKDATokenData",
    "validateTokenIdentifier",
    "upgradeContract",
    "upgradeFromSourceContract",
    "deleteContract",
    "getArgumentLength",
    "getArgument",
    "getFunction",
    "getNumArguments",
    "storageStore",
    "storageLoadLength",
    "storageLoadFromAddress",
    "storageLoad",
    "setStorageLock",
    "getStorageLock",
    "isStorageLocked",
    "clearStorageLock",
    "getCaller",
    "checkNoPayment",
    "getCallValue",
    "getKDAValue",
    "getKDAValueByIndex",
    "getKDATokenName",
    "getKDATokenNameByIndex",
    "getKDATokenNonce",
    "getKDATokenNonceByIndex",
    "getKDATokenType",
    "getKDATokenTypeByIndex",
    "getNumKDATransfers",
    "getCallValueByTokenName",
    "getCallValueTokenName",
    "getCallValueTokenNameByIndex",
    "writeLog",
    "writeEventLog",
    "getBlockTimestamp",
    "getBlockNonce",
    "getBlockRound",
    "getBlockEpoch",
    "getBlockRandomSeed",
    "getStateRootHash",
    "getPrevBlockTimestamp",
    "getPrevBlockNonce",
    "getPrevBlockRound",
    "getPrevBlockEpoch",
    "getPrevBlockRandomSeed",
    "finish",
    "executeOnSameContext",
    "executeOnDestContext",
    "executeReadOnly",
    "createContract",
    "deployFromSourceContract",
    "getNumReturnData",
    "getReturnDataSize",
    "getReturnData",
    "cleanReturnData",
    "deleteFromReturnData",
    "getOriginalTxHash",
    "getCurrentTxHash",
    "managedSCAddress",
    "managedOwnerAddress",
    "managedCaller",
    "managedSignalError",
    "managedWriteLog",
    "managedGetOriginalTxHash",
    "managedGetStateRootHash",
    "managedGetBlockRandomSeed",
    "managedGetPrevBlockRandomSeed",
    "managedGetReturnData",
    "managedGetKDACallValue",
    "managedGetMultiKDACallValue",
    "managedGetKDABalance",
    "managedGetKDATokenData",
    "managedUpgradeFromSourceContract",
    "managedUpgradeContract",
    "managedDeleteContract",
    "managedDeployFromSourceContract",
    "managedCreateContract",
    "managedExecuteReadOnly",
    "managedExecuteOnSameContext",
    "managedExecuteOnDestContext",
    "managedMultiTransferKDANFTExecute",
    "managedBufferToHex",
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
    "bigIntGetUnsignedArgument",
    "bigIntGetSignedArgument",
    "bigIntStorageStoreUnsigned",
    "bigIntStorageLoadUnsigned",
    "bigIntGetCallValue",
    "bigIntGetKDACallValue",
    "bigIntGetKDACallValueByIndex",
    "bigIntGetExternalBalance",
    "bigIntGetKDAExternalBalance",
    "bigIntNew",
    "bigIntUnsignedByteLength",
    "bigIntSignedByteLength",
    "bigIntGetUnsignedBytes",
    "bigIntGetSignedBytes",
    "bigIntSetUnsignedBytes",
    "bigIntSetSignedBytes",
    "bigIntIsInt64",
    "bigIntGetInt64",
    "bigIntSetInt64",
    "bigIntAdd",
    "bigIntSub",
    "bigIntMul",
    "bigIntTDiv",
    "bigIntTMod",
    "bigIntEDiv",
    "bigIntEMod",
    "bigIntSqrt",
    "bigIntPow",
    "bigIntLog2",
    "bigIntAbs",
    "bigIntNeg",
    "bigIntSign",
    "bigIntCmp",
    "bigIntNot",
    "bigIntAnd",
    "bigIntOr",
    "bigIntXor",
    "bigIntShr",
    "bigIntShl",
    "bigIntFinishUnsigned",
    "bigIntFinishSigned",
    "bigIntToString",
    "mBufferNew",
    "mBufferNewFromBytes",
    "mBufferGetLength",
    "mBufferGetBytes",
    "mBufferGetByteSlice",
    "mBufferCopyByteSlice",
    "mBufferEq",
    "mBufferSetBytes",
    "mBufferSetByteSlice",
    "mBufferAppend",
    "mBufferAppendBytes",
    "mBufferToBigIntUnsigned",
    "mBufferToBigIntSigned",
    "mBufferFromBigIntUnsigned",
    "mBufferFromBigIntSigned",
    "mBufferToBigFloat",
    "mBufferFromBigFloat",
    "mBufferStorageStore",
    "mBufferStorageLoad",
    "mBufferStorageLoadFromAddress",
    "mBufferGetArgument",
    "mBufferFinish",
    "mBufferSetRandom",
    "managedMapNew",
    "managedMapPut",
    "managedMapGet",
    "managedMapRemove",
    "managedMapContains",
    "smallIntGetUnsignedArgument",
    "smallIntGetSignedArgument",
    "smallIntFinishUnsigned",
    "smallIntFinishSigned",
    "smallIntStorageStoreUnsigned",
    "smallIntStorageStoreSigned",
    "smallIntStorageLoadUnsigned",
    "smallIntStorageLoadSigned",
    "int64getArgument",
    "int64finish",
    "int64storageStore",
    "int64storageLoad",
    "sha256",
    "managedSha256",
    "keccak256",
    "managedKeccak256",
    "ripemd160",
    "managedRipemd160",
    "verifyBLS",
    "managedVerifyBLS",
    "verifyEd25519",
    "managedVerifyEd25519",
    "verifyCustomSecp256k1",
    "managedVerifyCustomSecp256k1",
    "verifySecp256k1",
    "managedVerifySecp256k1",
    "encodeSecp256k1DerSignature",
    "managedEncodeSecp256k1DerSignature",
    "addEC",
    "doubleEC",
    "isOnCurveEC",
    "scalarBaseMultEC",
    "managedScalarBaseMultEC",
    "scalarMultEC",
    "managedScalarMultEC",
    "marshalEC",
    "managedMarshalEC",
    "marshalCompressedEC",
    "managedMarshalCompressedEC",
    "unmarshalEC",
    "managedUnmarshalEC",
    "unmarshalCompressedEC",
    "managedUnmarshalCompressedEC",
    "generateKeyEC",
    "managedGenerateKeyEC",
    "createEC",
    "managedCreateEC",
    "getCurveLengthEC",
    "getPrivKeyByteLengthEC",
    "ellipticCurveGetValues",
];
