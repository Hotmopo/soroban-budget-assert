use budget_macros::budget_lt;

// Same as missing_env but exercises budget_lt to confirm
// it produces sensible errors when `env` is absent.
#[budget_lt(cpu = 1000)]
fn test_budget_lt_without_env() {
    // No `env` variable — should fail to compile.
}

fn main() {}
