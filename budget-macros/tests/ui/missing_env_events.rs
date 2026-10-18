use budget_macros::budget_events_lt;

// Same as missing_env but exercises budget_events_lt to confirm
// it produces sensible errors when `env` is absent.
#[budget_events_lt(10)]
fn test_events_without_env() {
    // No `env` variable — should fail to compile.
}

fn main() {}
