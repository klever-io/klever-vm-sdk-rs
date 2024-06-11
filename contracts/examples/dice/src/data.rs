use klever_sc::derive_imports::*;

// OVER
pub const MIN_PREDICTION_OVER_VALUE: u32 = 5;
pub const MAX_PREDICTION_OVER_VALUE: u32 = 98;

// UNDER
pub const MIN_PREDICTION_UNDER_VALUE: u32 = 1;
pub const MAX_PREDICTION_UNDER_VALUE: u32 = 94;

pub const MAX_ROLL_VALUE: u32 = 99;

#[derive(TopEncode, TopDecode, TypeAbi, PartialEq, Eq, Clone, Copy)]
pub enum BetType {
    UNDER,
    OVER,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Bet {
    pub bet_type: u32,
    pub bet_value: u32,
    pub dice_value: u32,
    pub multiplier: u32,
    pub is_winner: bool,
}
