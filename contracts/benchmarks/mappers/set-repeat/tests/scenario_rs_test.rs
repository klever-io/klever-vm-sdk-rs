use klever_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "kleversc:output/set-repeat.kleversc.json",
        set_repeat::ContractBuilder,
    );
    blockchain
}

#[test]
fn set_repeat_rs() {
    world().run("scenarios/set_repeat.scen.json");
}

#[test]
fn set_repeat_struct_rs() {
    world().run("scenarios/set_repeat_struct.scen.json");
}
