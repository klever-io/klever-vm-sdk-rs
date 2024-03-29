use klever_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/digital-cash");

    blockchain.register_contract(
        "file:output/digital-cash.wasm",
        digital_cash::ContractBuilder,
    );
    blockchain
}

#[test]
fn claim_fees_rs() {
    world().run("scenarios/claim-fees.scen.json");
}

#[test]
fn claim_kda_rs() {
    world().run("scenarios/claim-kda.scen.json");
}

#[test]
fn claim_klv_rs() {
    world().run("scenarios/claim-klv.scen.json");
}

#[test]
fn claim_multi_kda_rs() {
    world().run("scenarios/claim-multi-kda.scen.json");
}

#[test]
fn forward_rs() {
    world().run("scenarios/forward.scen.json");
}

#[test]
fn fund_klv_and_kda_rs() {
    world().run("scenarios/fund-klv-and-kda.scen.json");
}

#[test]
fn set_accounts_rs() {
    world().run("scenarios/set-accounts.scen.json");
}

#[test]
fn withdraw_kda_rs() {
    world().run("scenarios/withdraw-kda.scen.json");
}

#[test]
fn withdraw_klv_rs() {
    world().run("scenarios/withdraw-klv.scen.json");
}

#[test]
fn withdraw_multi_kda_rs() {
    world().run("scenarios/withdraw-multi-kda.scen.json");
}
