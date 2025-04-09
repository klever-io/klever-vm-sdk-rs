use core::ops::Div;
use core::ops::Mul;
use core::ops::Sub;

use klever_sc::{
    api::ManagedTypeApi,
    types::{BigUint, ManagedBuffer},
};

pub fn name_to_id<M: ManagedTypeApi>(buffer: &ManagedBuffer<M>) -> ManagedBuffer<M> {
    let mut ret = ManagedBuffer::new();
    buffer.for_each_batch::<1, _>(|batch| {
        let byte_to_append = match batch[0] {
            b' ' => b'-',
            b'A'..=b'Z' => batch[0] + (b'a' - b'A'),
            b'0'..=b'9' | b'a'..=b'z' | b'-' => batch[0],
            _ => return,
        };
        ret.append_bytes(&[byte_to_append]);
    });

    ret
}

pub fn discount_percentage_fee<M: ManagedTypeApi>(
    amount: &BigUint<M>,
    percentage: u32,
) -> BigUint<M> {
    let fee = amount.clone().div(&BigUint::from(100u32));
    let final_fee = fee.clone().mul(&BigUint::from(percentage));
    amount.clone().sub(final_fee.clone())
}
