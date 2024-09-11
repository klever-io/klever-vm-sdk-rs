use klever_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "kleversc:output/empty.kleversc.json",
        empty::ContractBuilder,
    );
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/empty.scen.json");
}
