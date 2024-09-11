use klever_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "kleversc:output/nft-minter.kleversc.json",
        nft_minter::ContractBuilder,
    );
    blockchain
}

#[test]
fn buy_nft_rs() {
    world().run("scenarios/buy_nft.scen.json");
}

#[test]
fn create_nft_rs() {
    world().run("scenarios/create_nft.scen.json");
}

#[test]
fn init_rs() {
    world().run("scenarios/init.scen.json");
}
