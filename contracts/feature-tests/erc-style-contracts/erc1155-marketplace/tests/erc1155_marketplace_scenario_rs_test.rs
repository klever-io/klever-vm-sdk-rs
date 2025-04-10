use klever_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract(
        "kleversc:output/erc1155-marketplace.kleversc.json",
        erc1155_marketplace::ContractBuilder,
    );
    blockchain.register_contract(
        "kleversc:../erc1155/output/erc1155.kleversc.json",
        erc1155::ContractBuilder,
    );

    blockchain
}

#[test]
fn auction_batch_rs() {
    world().run("scenarios/auction_batch.scen.json");
}

#[test]
fn auction_single_token_klv_rs() {
    world().run("scenarios/auction_single_token_klv.scen.json");
}

#[test]
fn bid_first_klv_rs() {
    world().run("scenarios/bid_first_klv.scen.json");
}

#[test]
fn bid_second_klv_rs() {
    world().run("scenarios/bid_second_klv.scen.json");
}

#[test]
fn bid_third_klv_rs() {
    world().run("scenarios/bid_third_klv.scen.json");
}

#[test]
fn end_auction_rs() {
    world().run("scenarios/end_auction.scen.json");
}
