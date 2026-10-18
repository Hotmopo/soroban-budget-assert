//! A `return` that belongs to a closure written inside macro invocation tokens
//! is still rejected: `find_return_token` cannot see macro semantics, so the
//! conservative guidance error is kept rather than silently skipping the
//! budget assertion.

#[path = "support/mock_env.rs"]
mod mock_env;

use budget_macros::budget_cpu_lt;
use mock_env::Env;

#[budget_cpu_lt(1_000)]
fn closure_return_inside_macro() {
    let _env = Env::new(999, 0);
    assert!((|| {
        return true;
    })());
}

fn main() {
    closure_return_inside_macro();
}
