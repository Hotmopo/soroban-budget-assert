#[path = "support/mock_env.rs"]
mod mock_env;

use budget_macros::budget_ledger_entries_lt;
// Keep `ContractCostType` imported (via the mock's `soroban_sdk` shim) so the
// failure is the missing `env` binding, not the cost-type path.
#[allow(unused_imports)]
use mock_env::{soroban_sdk, ContractCostType};

// Same as missing_env but exercises budget_ledger_entries_lt to confirm
// it produces sensible errors when `env` is absent.
#[budget_ledger_entries_lt(10)]
fn test_ledger_without_env() {
    // No `env` variable — should fail to compile.
}

fn main() {}
