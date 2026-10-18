# Adding a New Cost Metric

Adding a metric is a coordinated change across four real extension points.
Keep the metric name and units consistent at every point, then add focused
tests for the implementation and report output.

## Step 1 — Add the public metric contract

**Files:** `src/traits.rs` and `src/impls.rs`

Add the metric to `CostMeasurer::value_for` in `src/traits.rs` when it can be
measured through the public core abstraction. Add a dedicated cost trait or
extend the relevant trait in `src/impls.rs` when the value needs a typed
implementation. Keep local-only estimates separate from network-only values:
the CLI obtains ledger resource values from simulation rather than local
execution.

Update the examples and unit tests in these files so the new metric has a
documented name, unit, and expected value.

## Step 2 — Add the assertion macro

**File:** `budget-macros/src/lib.rs`

Add a `budget_<metric>_lt` attribute beside the existing metric attributes.
Parse the shared `StandaloneSpec`, validate environment-backed limits, select
the budget method (or another real local measurement), and pass the result to
`generate_metric_assert`. Use `instrument_exit_paths` so early returns and
trailing expressions are checked consistently with the existing macros.

Add or update the macro's UI fixture and test the static, environment-backed,
and baseline forms that the metric supports.

## Step 3 — Register the report metric

**File:** `cargo-budget-report/src/main.rs`

Register the metric in the CLI's report registry: map its JSON name when
loading reports, add its field to the measured-resource/baseline conversion,
include it in the simulation report-row loop, and add its limit to
`limit_for_metric` and failed-simulation entries when it is enforceable.
Follow the existing `CPU Instructions`, `Read Bytes`, and `Write Bytes`
entries rather than duplicating output logic.

Add coverage for text, JSON, CSV, baseline comparison, and check-mode output
where the new metric is exposed.

## Step 4 — Document and verify

Update the relevant reference or measurements page with the metric's source,
unit, local/network distinction, and any conversion or proxy caveat. Add the
metric to the appropriate fixture and run:

```bash
cargo fmt --all -- --check
cargo test --workspace
```

Before opening a pull request, confirm that every path named in this guide
exists and that the metric appears in both local assertion and CLI report
tests. Property-test notes belong with the real tests in
`budget-macros/src/parser_props.rs`; there is no `budget-core` crate in this
workspace.
