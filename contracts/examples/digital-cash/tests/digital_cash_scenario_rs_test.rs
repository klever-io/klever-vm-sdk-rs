use klever_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "kleversc:output/digital-cash.kleversc.json",
        digital_cash::ContractBuilder,
    );
    blockchain
}

#[test]
fn claim_klv_rs() {
    world().run("scenarios/claim-klv.scen.json");
}

#[test]
fn claim_kda_rs() {
    world().run("scenarios/claim-kda.scen.json");
}

#[test]
fn claim_fees_rs() {
    world().run("scenarios/claim-fees.scen.json");
}

#[test]
fn claim_multi_kda_rs() {
    world().run("scenarios/claim-multi-kda.scen.json");
}

#[test]
fn forward_rs() {
    world().run("scenarios/forward.scen.json");
}

#[test]
fn fund_klv_and_kda_rs() {
    world().run("scenarios/fund-klv-and-kda.scen.json");
}

#[test]
fn set_accounts_rs() {
    world().run("scenarios/set-accounts.scen.json");
}

#[test]
fn whitelist_blacklist_fee_token_rs() {
    world().run("scenarios/whitelist-blacklist-fee-tokens.scen.json");
}

#[test]
fn pay_fee_and_fund_kda_rs() {
    world().run("scenarios/pay-fee-and-fund-kda.scen.json");
}

#[test]
fn pay_fee_and_fund_klv_rs() {
    world().run("scenarios/pay-fee-and-fund-klv.scen.json");
}

#[test]
fn withdraw_klv_rs() {
    world().run("scenarios/withdraw-klv.scen.json");
}

#[test]
fn withdraw_kda_rs() {
    world().run("scenarios/withdraw-kda.scen.json");
}

#[test]
fn withdraw_multi_kda_rs() {
    world().run("scenarios/withdraw-multi-kda.scen.json");
}
