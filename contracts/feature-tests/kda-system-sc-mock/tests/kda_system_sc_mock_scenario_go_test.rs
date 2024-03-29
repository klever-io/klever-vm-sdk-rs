use klever_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
#[ignore = "builtin SC not implemented"]
fn kda_system_sc_go() {
    world().run("scenarios/kda_system_sc.scen.json");
}
