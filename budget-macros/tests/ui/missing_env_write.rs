use budget_macros::budget_write_bytes_lt;

// Same as missing_env but exercises budget_write_bytes_lt to confirm
// it produces sensible errors when `env` is absent.
#[budget_write_bytes_lt(4096)]
fn test_write_without_env() {
    // No `env` variable — should fail to compile.
}

fn main() {}
