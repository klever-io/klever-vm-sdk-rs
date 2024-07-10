use klever_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "kleversc:output/kda-transfer-with-fee.kleversc.json",
        kda_transfer_with_fee::ContractBuilder,
    );
    blockchain
}

// #[test]
// fn claim_rs() {
//     world().run("scenarios/claim.scen.json");
// }

#[test]
fn deploy_rs() {
    world().run("scenarios/deploy.scen.json");
}

// #[test]
// fn setup_fees_and_transfer_rs() {
//     world().run("scenarios/setup_fees_and_transfer.scen.json");
// }
