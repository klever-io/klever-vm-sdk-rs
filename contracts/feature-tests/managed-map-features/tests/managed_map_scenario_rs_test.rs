use klever_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract(
        "kleversc:output/managed-map-features.kleversc.json",
        managed_map_features::ContractBuilder,
    );
    blockchain
}

#[test]
fn mmap_get_rs() {
    world().run("scenarios/mmap_get.scen.json");
}

#[test]
fn mmap_remove_rs() {
    world().run("scenarios/mmap_remove.scen.json");
}
