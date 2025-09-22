use crate::data::{BetType, MAX_ROLL_VALUE};
use klever_sc::proxy_imports::RandomnessSource;

use core::ops::Div;
use core::ops::Mul;

use klever_sc::{api::ManagedTypeApi, types::BigUint};

pub fn roll<M: ManagedTypeApi>() -> u32 {
    let mut rand = RandomnessSource::<M>::new();

    let result: u32 = rand.next_u32() % MAX_ROLL_VALUE;

    result + 1
}

pub fn calculate_payout<M: ManagedTypeApi>(payout: u32, payment: BigUint<M>) -> (BigUint<M>, u32) {
    let multiplier = 10000 / payout;

    (
        payment.mul(multiplier).div(BigUint::from(100u32)),
        multiplier,
    )
}

pub fn check_bet(bet_value: u32, bet_type: BetType, dice_value: u32) -> bool {
    let mut is_winner = false;

    if bet_type == BetType::UNDER {
        if dice_value <= bet_value {
            is_winner = true;
        }
    } else if dice_value >= bet_value {
        is_winner = true
    }

    is_winner
}
