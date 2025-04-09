use klever_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn deploy_erc_20_and_crowdfunding_go() {
    world().run("scenarios/deploy_erc20_and_crowdfunding.scen.json");
}

#[test]
#[ignore = "Halts execution during execute_on_dest_context; Go SDK does not propagate error, triggering fail instead"]
fn fund_with_insufficient_allowance_go() {
    world().run("scenarios/fund_with_insufficient_allowance.scen.json");
}

#[test]
fn fund_with_sufficient_allowance_go() {
    world().run("scenarios/fund_with_sufficient_allowance.scen.json");
}

#[test]
#[ignore = "Halts execution during execute_on_dest_context; Go SDK does not propagate error, triggering fail instead"]
fn fund_without_allowance_go() {
    world().run("scenarios/fund_without_allowance.scen.json");
}
