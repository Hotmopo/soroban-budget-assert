use budget_macros::budget_cpu_lt;

#[budget_cpu_lt(env = "UI_TEST_UNSET_BUDGET")]
fn test_with_unset_env_var() {
    let env = ();
}

fn main() {}
