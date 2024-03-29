use klever_sc_scenario::*;

fn world() -> ScenarioWorld {
    todo!()
}

#[test]
#[ignore = "builtin SC not implemented"]
fn kda_system_sc_rs() {
    world().run("scenarios/kda_system_sc.scen.json");
}
