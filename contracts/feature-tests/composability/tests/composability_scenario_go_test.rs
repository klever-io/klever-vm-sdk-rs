use klever_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn proxy_test_init_go() {
    world().run("scenarios/proxy_test_init.scen.json");
}

#[test]
fn proxy_test_message_same_shard_go() {
    world().run("scenarios/proxy_test_message.scen.json");
}

#[test]
fn proxy_test_payment_same_shard_go() {
    world().run("scenarios/proxy_test_payment.scen.json");
}

#[test]
fn proxy_test_upgrade_go() {
    world().run("scenarios/proxy_test_upgrade.scen.json");
}
