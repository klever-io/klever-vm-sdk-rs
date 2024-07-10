use klever_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("kleversc:output/adder.kleversc.json", adder::ContractBuilder);
    blockchain
}

#[test]
fn adder_rs() {
    world().run("scenarios/adder.scen.json");
}

#[test]
fn interactor_trace_rs() {
    world().run("scenarios/interactor_trace.scen.json");
}
