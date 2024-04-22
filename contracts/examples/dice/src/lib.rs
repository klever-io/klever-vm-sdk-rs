#![no_std]

klever_sc::imports!();
klever_sc::derive_imports!();

use rand::*;
use crate::utils::{calculate_payout, check_bet, roll};
use crate::data::{BetType, Bet, MAX_PREDICTION_OVER_VALUE, MIN_PREDICTION_OVER_VALUE, MAX_PREDICTION_UNDER_VALUE, MIN_PREDICTION_UNDER_VALUE};

pub mod data;
pub mod utils;
pub mod rand;

#[klever_sc::contract]
pub trait Dice :
 {

    #[init]
    fn init(&self) {}

    #[view(getLastResult)]
    #[storage_mapper("lastResult")]
    fn last_result(&self, address: &ManagedAddress) -> SingleValueMapper<Bet>;
    

    #[payable("KLV")]
    #[endpoint(bet)]
    fn bet(&self, bet_type: BetType, bet_value: u32) {
        let caller = self.blockchain().get_caller();

        require!(
            !self.blockchain().is_smart_contract(&caller),
            "Can't call this contract by other contract"
        );

        require!(
            bet_type == BetType::OVER || bet_type == BetType::UNDER,
            "Bet type needs to be Under(0) or Over(1)",
        );

        let mut payout: u32 = bet_value;

        if bet_type == BetType::UNDER{
            require!(
                bet_value >= MIN_PREDICTION_UNDER_VALUE || bet_value <= MAX_PREDICTION_UNDER_VALUE,
                "The prediction must be between 1 and 94",
            );
        } else {
            require!(
               bet_value >= MIN_PREDICTION_OVER_VALUE || bet_value <= MAX_PREDICTION_OVER_VALUE,
                "The prediction must be between 5 and 98",
            );

            payout = 99 - bet_value;
        }

        let (token_identifier, payment) = self.call_value().klv_or_single_fungible_kda();
        require!(
            token_identifier == TokenIdentifier::from("KLV"),
            "payment token must be KLV"
        );

        require!(payment > 0, "bet amount can't be zero");

        let dice_value: u32 = roll(
            Rand::new(
                self.blockchain().get_block_random_seed(),
                self.blockchain().get_tx_hash(),
            )
        );

        let is_winner: bool = check_bet(bet_value, bet_type, dice_value);

        let (payment_total,multiplier) = calculate_payout(payout, payment);

        if is_winner {
            self.send().direct_klv(
                &self.blockchain().get_caller(),
                &BigUint::from(payment_total),
            );
        }

        self.last_result(&self.blockchain().get_caller()).set(Bet {
            bet_type: bet_type as u32,
            bet_value: bet_value,
            dice_value: dice_value,
            multiplier: multiplier,
            is_winner: is_winner,
        });
    }

}
