//! Issue #844: a run with no successful simulations must still honour the
//! requested output format instead of leaving `--json`/`--csv` consumers
//! with an empty document.

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

/// The `no_exports_workspace` fixture contains no simulatable functions, so
/// every run through it hits the `measurements.is_empty()` early return.
fn setup_empty_workspace() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().expect("failed to create tempdir");
    copy_dir_all(
        &manifest_dir().join("tests/fixtures/no_exports_workspace"),
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

#[test]
fn empty_run_with_json_writes_a_valid_envelope() {
    let workspace = setup_empty_workspace();

    let assert = budget_report_cmd(workspace.path())
        .args([
            "budget-report",
            "--network",
            "local",
            "--source",
            "alice",
            "--json",
        ])
        .assert();
    // Exit code is preserved (the fixture has nothing simulatable, so the
    // run still fails); what matters here is that stdout is parseable.
    let output = assert.failure().get_output().clone();
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    assert!(
        !stdout.trim().is_empty(),
        "--json on an empty run must not produce a zero-byte document"
    );
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("--json on empty run must be valid JSON");
    assert_eq!(json["schema_version"], 1);
    assert_eq!(
        json["snapshots"].as_array().map(Vec::len),
        Some(0),
        "empty run snapshots: {stdout}"
    );
}

#[test]
fn empty_run_with_csv_writes_the_header_row() {
    let workspace = setup_empty_workspace();

    let assert = budget_report_cmd(workspace.path())
        .args([
            "budget-report",
            "--network",
            "local",
            "--source",
            "alice",
            "--csv",
        ])
        .assert();
    let output = assert.failure().get_output().clone();
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let mut lines = stdout.lines();
    assert_eq!(
        lines.next().unwrap_or(""),
        "package,function,metric,value",
        "empty --csv must still emit the header row: {stdout}"
    );
}

#[test]
fn empty_run_with_markdown_renders_header_and_footer() {
    let workspace = setup_empty_workspace();

    let assert = budget_report_cmd(workspace.path())
        .args([
            "budget-report",
            "--network",
            "local",
            "--source",
            "alice",
            "--markdown",
        ])
        .assert();
    let output = assert.failure().get_output().clone();
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    assert!(
        stdout.contains("| Package | Function | Metric | Value |"),
        "empty --markdown keeps the table shell: {stdout}"
    );
}

#[test]
fn empty_run_with_html_still_writes_the_file() {
    let workspace = setup_empty_workspace();

    let assert = budget_report_cmd(workspace.path())
        .args([
            "budget-report",
            "--network",
            "local",
            "--source",
            "alice",
            "--html",
            "report.html",
        ])
        .assert();
    let output = assert.failure().get_output().clone();
    let html = fs::read_to_string(workspace.path().join("report.html"))
        .expect("HTML report must still be written on an empty run");
    assert!(
        html.contains("<html"),
        "HTML-on-empty behaviour is unchanged: {html:?} / stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
