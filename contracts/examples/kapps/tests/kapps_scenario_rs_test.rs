use klever_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("file:output/kapps.wasm", kapps::ContractBuilder);
    blockchain
}

// Create Asset Scenario

#[test]
fn kapps_create_asset_rs() {
    world().run("scenarios/kapps_create_asset.scen.json");
}
