//! Growth model matching is case-sensitive: `Linear` is unknown.

use budget_macros::budget_scaling;

// Keep the attribute on the documented line used by the compile-fail snapshot.
#[budget_scaling(sizes = [10, 100], model = Linear, tolerance = 0.3)]
fn test_model_case_sensitive() {}

fn main() {}
