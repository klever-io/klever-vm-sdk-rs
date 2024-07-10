use klever_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "kleversc:output/crowdfunding-erc20.kleversc.json",
        crowdfunding_erc20::ContractBuilder,
    );

    blockchain.register_contract("kleversc:../erc20/output/erc20.kleversc.json", erc20::ContractBuilder);

    blockchain
}

#[test]
fn deploy_erc_20_and_crowdfunding_rs() {
    world().run("scenarios/deploy_erc20_and_crowdfunding.scen.json");
}

#[test]
fn fund_with_insufficient_allowance_rs() {
    world().run("scenarios/fund_with_insufficient_allowance.scen.json");
}

#[test]
fn fund_with_sufficient_allowance_rs() {
    world().run("scenarios/fund_with_sufficient_allowance.scen.json");
}

#[test]
fn fund_without_allowance_rs() {
    world().run("scenarios/fund_without_allowance.scen.json");
}
