use klever_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn claim_fees_go() {
    world().run("scenarios/claim-fees.scen.json");
}

#[test]
fn claim_kda_go() {
    world().run("scenarios/claim-kda.scen.json");
}

#[test]
fn claim_klv_go() {
    world().run("scenarios/claim-klv.scen.json");
}

#[test]
fn claim_multi_kda_go() {
    world().run("scenarios/claim-multi-kda.scen.json");
}

#[test]
fn forward_go() {
    world().run("scenarios/forward.scen.json");
}

#[test]
fn fund_klv_and_kda_go() {
    world().run("scenarios/fund-klv-and-kda.scen.json");
}

#[test]
fn set_accounts_go() {
    world().run("scenarios/set-accounts.scen.json");
}

#[test]
fn withdraw_kda_go() {
    world().run("scenarios/withdraw-kda.scen.json");
}

#[test]
fn withdraw_klv_go() {
    world().run("scenarios/withdraw-klv.scen.json");
}

#[test]
fn withdraw_multi_kda_go() {
    world().run("scenarios/withdraw-multi-kda.scen.json");
}
