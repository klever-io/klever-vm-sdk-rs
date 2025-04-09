use klever_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "kleversc:output/factorial.kleversc.json",
        factorial::ContractBuilder,
    );
    blockchain
}

#[test]
fn factorial_rs() {
    world().run("scenarios/factorial.scen.json");
}
