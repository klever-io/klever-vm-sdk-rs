use klever_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn ambiguous_split_rejected_go() {
    world().run("scenarios/ambiguous-split-rejected.scen.json");
}

#[test]
fn claim_go() {
    world().run("scenarios/claim.scen.json");
}

#[test]
fn commit_and_reveal_on_time_go() {
    world().run("scenarios/commit-and-reveal-on-time.scen.json");
}

#[test]
fn commitment_not_reusable_go() {
    world().run("scenarios/commitment-not-reusable.scen.json");
}

#[test]
fn commit_reveal_init_go() {
    world().run("scenarios/commit-reveal-init.scen.json");
}

#[test]
fn forfeit_remainder_go() {
    world().run("scenarios/forfeit-remainder.scen.json");
}

#[test]
fn missing_reveal_forfeits_go() {
    world().run("scenarios/missing-reveal-forfeits.scen.json");
}

#[test]
fn money_guards_go() {
    world().run("scenarios/money-guards.scen.json");
}

#[test]
fn no_one_reveals_go() {
    world().run("scenarios/no-one-reveals.scen.json");
}

// Validation and deadline boundaries
#[test]
fn deadlines_go() {
    world().run("scenarios/deadlines.scen.json");
}

#[test]
fn payment_guards_go() {
    world().run("scenarios/payment-guards.scen.json");
}

#[test]
fn round_full_go() {
    world().run("scenarios/round-full.scen.json");
}

#[test]
fn reveal_guards_go() {
    world().run("scenarios/reveal-guards.scen.json");
}

#[test]
fn create_round_guards_go() {
    world().run("scenarios/create-round-guards.scen.json");
}

#[test]
fn finalize_guards_go() {
    world().run("scenarios/finalize-guards.scen.json");
}
