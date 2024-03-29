use klever_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn auction_batch_go() {
    world().run("scenarios/auction_batch.scen.json");
}

#[test]
fn auction_single_token_klv_go() {
    world().run("scenarios/auction_single_token_klv.scen.json");
}

#[test]
fn bid_first_klv_go() {
    world().run("scenarios/bid_first_klv.scen.json");
}

#[test]
fn bid_second_klv_go() {
    world().run("scenarios/bid_second_klv.scen.json");
}

#[test]
fn bid_third_klv_go() {
    world().run("scenarios/bid_third_klv.scen.json");
}

#[test]
fn end_auction_go() {
    world().run("scenarios/end_auction.scen.json");
}
