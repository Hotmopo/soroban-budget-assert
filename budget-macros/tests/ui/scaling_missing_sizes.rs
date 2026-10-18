//! Scaling configuration without `sizes` must name the missing field.

use budget_macros::budget_scaling;

// Keep the attribute on the documented line used by the compile-fail snapshot.
#[budget_scaling(model = linear, tolerance = 0.3)]
fn test_missing_sizes() {}

fn main() {}
