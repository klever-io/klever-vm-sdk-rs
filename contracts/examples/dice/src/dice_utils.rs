use rand::*;
use crate::dice_data::{BetType,MAX_ROLL_VALUE};

use core::ops::Div;
use core::ops::Mul;

use klever_sc::{
    api::ManagedTypeApi, types::BigUint
};

pub fn roll(randomizer: Rand) -> u32 {
    let mut randomizer = randomizer;

    let result: u32 = randomizer.next_u32() % MAX_ROLL_VALUE;

    result + 1 
}

pub fn calculate_payout<M: ManagedTypeApi>(payout: u32, payment: BigUint<M>) -> (BigUint<M>, u32) {
    let multiplier = 10000 / payout;
    
    return (payment.mul(multiplier).div(BigUint::from(100 as u32)), multiplier);
}

pub fn check_bet(bet_value: u32, bet_type: BetType, dice_value: u32) -> bool {
    let mut is_winner = false;

    if bet_type == BetType::UNDER {
        if dice_value <= bet_value {
            is_winner = true;
        }
    } else {
        if dice_value >= bet_value {
            is_winner = true 
        }
    }

    return is_winner;
}
