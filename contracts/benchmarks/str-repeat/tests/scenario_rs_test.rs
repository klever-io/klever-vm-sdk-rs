use klever_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract("kleversc:output/str-repeat.kleversc.json", str_repeat::ContractBuilder);
    blockchain
}

#[test]
fn str_repeat_rs() {
    world().run("scenarios/str_repeat.scen.json");
}
