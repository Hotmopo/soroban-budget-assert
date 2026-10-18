//! Scaling configuration without `tolerance` must name the missing field.

use budget_macros::budget_scaling;

// Keep the attribute on the documented line used by the compile-fail snapshot.
#[budget_scaling(sizes = [10, 100], model = linear)]
fn test_missing_tolerance() {}

fn main() {}
