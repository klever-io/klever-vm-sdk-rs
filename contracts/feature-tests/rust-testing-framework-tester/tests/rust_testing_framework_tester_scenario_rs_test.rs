use klever_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "kleversc:output/rust-testing-framework-tester.kleversc.json",
        rust_testing_framework_tester::ContractBuilder,
    );
    blockchain
}

#[test]
fn test_rs() {
    world().run("scenarios/test.scen.json");
}

#[test]
fn test_kda_generation_rs() {
    world().run("scenarios/test_kda_generation.scen.json");
}

#[test]
fn test_multiple_sc_rs() {
    world().run("scenarios/test_multiple_sc.scen.json");
}
