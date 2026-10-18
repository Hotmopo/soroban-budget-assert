use budget_macros::budget_cpu_lt;

#[budget_cpu_lt(env = "UI_TEST_INVALID_BUDGET")]
fn test_with_invalid_env_var() {
    let env = ();
}

fn main() {}
