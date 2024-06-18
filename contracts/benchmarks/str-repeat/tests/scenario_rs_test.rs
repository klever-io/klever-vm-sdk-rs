use klever_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract(
        "kleversc:output/str-repeat.kleversc.json", 
        str_repeat::ContractBuilder
    );
    blockchain.register_contract(
        "kleversc:output/str-repeat-mb-builder-basic.kleversc.json",
        str_repeat::ContractBuilder,
    );
    blockchain.register_contract(
        "kleversc:output/str-repeat-mb-builder-cached.kleversc.json",
        str_repeat::ContractBuilder,
    );
    blockchain
}

#[test]
fn mb_builder_basic_rs() {
    world().run("scenarios/mb_builder_basic.scen.json");
}

#[test]
fn mb_builder_cached_rs() {
    world().run("scenarios/mb_builder_cached.scen.json");
}

#[test]
fn str_repeat_rs() {
    world().run("scenarios/str_repeat.scen.json");
}
