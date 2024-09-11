use klever_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract(
        "kleversc:first-contract/output/first-contract.kleversc.json",
        first_contract::ContractBuilder,
    );

    blockchain.register_contract(
        "kleversc:second-contract/output/second-contract.kleversc.json",
        second_contract::ContractBuilder,
    );
    blockchain
}

#[test]
fn init_rs() {
    world().run("scenarios/init.scen.json");
}

#[test]
fn reject_transfer_rs() {
    world().run("scenarios/reject_transfer.scen.json");
}

#[test]
fn simple_transfer_full_rs() {
    world().run("scenarios/simple_transfer_full.scen.json");
}

#[test]
fn simple_transfer_full_wrong_token_rs() {
    world().run("scenarios/simple_transfer_full_wrong_token.scen.json");
}

#[test]
fn simple_transfer_half_rs() {
    world().run("scenarios/simple_transfer_half.scen.json");
}
