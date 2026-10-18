//! Issue #842: `--check-baseline --csv` must emit parseable CSV, not the text table.

use assert_cmd::Command;
use std::fs;
use std::path::{Path, PathBuf};

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn fake_bin_dir() -> PathBuf {
    manifest_dir().join("tests/fixtures/fake_bin")
}

fn copy_dir_all(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("failed to create destination directory");
    for entry in fs::read_dir(src).expect("failed to read source directory") {
        let entry = entry.expect("failed to read directory entry");
        let file_type = entry.file_type().expect("failed to read file type");
        let dst_path = dst.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_all(&entry.path(), &dst_path);
        } else {
            fs::copy(entry.path(), &dst_path).expect("failed to copy fixture file");
        }
    }
}

fn setup_mock_workspace() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().expect("failed to create tempdir");
    copy_dir_all(
        &manifest_dir().join("tests/fixtures/mock_workspace"),
        tmp.path(),
    );
    tmp
}

fn mocked_path() -> String {
    let real_path = std::env::var("PATH").unwrap_or_default();
    format!("{}:{}", fake_bin_dir().display(), real_path)
}

fn budget_report_cmd(dir: &Path) -> Command {
    let mut cmd = Command::cargo_bin("cargo-budget-report").expect("binary should be built");
    cmd.current_dir(dir).env("PATH", mocked_path());
    cmd
}

fn write_baseline(dir: &Path, ping_cpu: u32) {
    fs::write(
        dir.join("budget-baseline.toml"),
        format!(
            "[\"mock-contract-a::ping\"]\n\
             cpu_instructions = {ping_cpu}\n\
             read_bytes = 2048\n\
             write_bytes = 4096\n\
             \n\
             [\"mock-contract-b::pong\"]\n\
             cpu_instructions = 1000000\n\
             read_bytes = 2048\n\
             write_bytes = 4096\n"
        ),
    )
    .expect("failed to write baseline");
}

fn run_check_baseline_csv(dir: &Path) -> (String, Option<i32>) {
    let assert = budget_report_cmd(dir)
        .args([
            "budget-report",
            "--network",
            "local",
            "--source",
            "alice",
            "--check-baseline",
            "budget-baseline.toml",
            "--csv",
        ])
        .assert();
    let output = assert.get_output();
    (
        String::from_utf8_lossy(&output.stdout).into_owned(),
        output.status.code(),
    )
}

#[test]
fn check_baseline_csv_emits_verdict_columns_on_regression() {
    let workspace = setup_mock_workspace();
    // ping's recorded cpu (100000) is well under the ~1,000,000 the fake RPC
    // reports, so its cpu row must carry a `regression` verdict and the run
    // exits 5; the output must still be CSV, not the text table.
    write_baseline(workspace.path(), 100000);

    let (stdout, code) = run_check_baseline_csv(workspace.path());
    assert_eq!(code, Some(5), "regressing baseline check exits 5: {stdout}");

    let mut lines = stdout.lines();
    assert_eq!(
        lines.next().unwrap_or(""),
        "package,function,metric,baseline,current,tolerance,verdict",
        "CSV header row and column order: {stdout}"
    );
    assert!(
        stdout.contains("mock-contract-a,ping,cpu_instructions,100000,1000000,"),
        "regressed row carries recorded/current/tolerance fields: {stdout}"
    );
    assert!(
        stdout.contains(",regression"),
        "regressed row carries a regression verdict: {stdout}"
    );
    assert!(
        !stdout.contains("=== BASELINE CHECK REPORT ==="),
        "must not fall back to the text table: {stdout}"
    );
}

#[test]
fn check_baseline_csv_clean_case_exits_zero_with_pass_verdicts() {
    let workspace = setup_mock_workspace();
    // Baseline matches the fake RPC exactly, so every row passes.
    write_baseline(workspace.path(), 1000000);

    let (stdout, code) = run_check_baseline_csv(workspace.path());
    assert_eq!(code, Some(0), "clean baseline check exits 0: {stdout}");

    let mut lines = stdout.lines();
    assert_eq!(
        lines.next().unwrap_or(""),
        "package,function,metric,baseline,current,tolerance,verdict",
        "CSV header row and column order: {stdout}"
    );
    assert!(
        stdout.contains(",pass"),
        "clean rows carry a pass verdict: {stdout}"
    );
    assert!(
        !stdout.contains(",regression"),
        "no regression verdicts on a clean run: {stdout}"
    );
}
