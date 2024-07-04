#![no_std]

use klever_sc::imports::*;

#[klever_sc::contract]
pub trait SendTxRepeat {
    #[init]
    fn init(&self) {}

    #[payable("KLV")]
    #[endpoint]
    fn repeat(&self, to: ManagedAddress, amount: BigUint, times: usize) {
        for _ in 0..times {
            self.tx().to(&to).klv(&amount).transfer();
        }
    }
}
