use klever_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn test_go() {
    world().run("scenarios/test.scen.json");
}

#[test]
fn test_kda_generation_go() {
    world().run("scenarios/test_kda_generation.scen.json");
}

#[test]
fn test_multiple_sc_go() {
    world().run("scenarios/test_multiple_sc.scen.json");
}
