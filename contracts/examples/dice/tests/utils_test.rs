 use dice::utils::check_bet;
 use dice::data::BetType;

#[test]
fn test_check_bet(){
    // Bet Under 50 - Roll 30 - Win
    
    let mut bet_value: u32 = 50;
    let mut lucky_number: u32 = 30;

    let mut is_winner: bool = check_bet(bet_value, BetType::UNDER, lucky_number);

    assert_eq!(true, is_winner);

    // Bet Under 50 - Roll 50 - Win

    bet_value = 50;
    lucky_number = 50;
 
    is_winner = check_bet(bet_value, BetType::UNDER, lucky_number);
 
    assert_eq!(true, is_winner);

    // Bet Under 30 - Roll 50 - Lose

    bet_value = 30;
    lucky_number = 50;

    is_winner = check_bet(bet_value, BetType::UNDER, lucky_number);

    assert_eq!(false, is_winner);


    // Bet Over 50 - Roll 80 - Win

    bet_value = 50;
    lucky_number = 80;

    is_winner = check_bet(bet_value, BetType::OVER, lucky_number);

    assert_eq!(true, is_winner);

    // Bet Over 80 - Roll 80 - Win

    bet_value = 80;
    lucky_number = 80;
   
    is_winner = check_bet(bet_value, BetType::OVER, lucky_number);
   
    assert_eq!(true, is_winner);

    // Bet Over 80 - Roll 60 - Lose

    bet_value = 80;
    lucky_number = 50;
    
    is_winner = check_bet(bet_value, BetType::OVER, lucky_number);
    
    assert_eq!(false, is_winner);

}