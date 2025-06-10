use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use sha2::Sha256;
use sha3::{Digest, Keccak256};

pub const SHA256_RESULT_LEN: usize = 32;
pub const KECCAK256_RESULT_LEN: usize = 32;

pub fn sha256(data: &[u8]) -> [u8; SHA256_RESULT_LEN] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub fn keccak256(data: &[u8]) -> [u8; KECCAK256_RESULT_LEN] {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub fn verify_ed25519(key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    let Ok(key_bytes) = <[u8; 32]>::try_from(key) else {
        return false;
    };
    let Ok(public) = VerifyingKey::from_bytes(&key_bytes) else {
        return false;
    };

    let Ok(sig_bytes) = <[u8; 64]>::try_from(signature) else {
        return false;
    };
    let sig = Signature::from_bytes(&sig_bytes);

    public.verify(message, &sig).is_ok()
}
