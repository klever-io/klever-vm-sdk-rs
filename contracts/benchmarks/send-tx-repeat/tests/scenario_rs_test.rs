use klever_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract(
        "kleversc:output/send-tx-repeat.kleversc.json",
        send_tx_repeat::ContractBuilder,
    );
    blockchain
}

#[test]
fn send_tx_repeat_rs() {
    world().run("scenarios/send_tx_repeat.scen.json");
}
