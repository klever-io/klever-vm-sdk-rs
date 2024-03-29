use num_bigint::BigUint;
use num_traits::Zero;

use super::KdaInstanceMetadata;

/// Holds the data for a Klever standard digital token transaction
#[derive(Clone, Default, Debug)]
pub struct KdaInstance {
    pub nonce: u64,
    pub balance: BigUint,
    pub metadata: KdaInstanceMetadata,
}

impl KdaInstance {
    pub fn default(nonce: u64) -> Self {
        KdaInstance {
            nonce,
            balance: BigUint::zero(),
            metadata: KdaInstanceMetadata::default(),
        }
    }

    pub fn fungible(balance: BigUint) -> Self {
        KdaInstance {
            nonce: 0,
            balance,
            metadata: KdaInstanceMetadata::default(),
        }
    }

    pub fn is_empty_kda(&self) -> bool {
        self.balance.is_zero()
    }
}
