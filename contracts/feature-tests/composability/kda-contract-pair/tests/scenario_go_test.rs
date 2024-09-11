use klever_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn init_go() {
    world().run("scenarios/init.scen.json");
}

#[test]
#[ignore = "Halts execution during execute_on_dest_context; Go SDK does not propagate error, triggering fail instead"]
fn reject_transfer_go() {
    world().run("scenarios/reject_transfer.scen.json");
}

#[test]
fn simple_transfer_full_go() {
    world().run("scenarios/simple_transfer_full.scen.json");
}

#[test]
fn simple_transfer_full_wrong_token_go() {
    world().run("scenarios/simple_transfer_full_wrong_token.scen.json");
}

#[test]
fn simple_transfer_half_go() {
    world().run("scenarios/simple_transfer_half.scen.json");
}
