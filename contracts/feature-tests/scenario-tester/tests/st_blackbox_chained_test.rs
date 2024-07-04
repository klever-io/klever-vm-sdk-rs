use klever_sc_scenario::imports::*;

const ADDER_PATH_EXPR: &str = "kleversc:output/scenario-tester.kleversc.json";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/scenario-tester");

    blockchain.register_contract(ADDER_PATH_EXPR, scenario_tester::ContractBuilder);
    blockchain
}
