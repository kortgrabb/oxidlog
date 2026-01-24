use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn test_xlog_init() {
    let temp_dir = tempdir().unwrap();
    let mut cmd = Command::cargo_bin("xlog").unwrap();
    cmd.arg("init")
        .env("XLOG_HOME", temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Journal initialized successfully"));
}

#[test]
fn test_xlog_add() {
    let temp_dir = tempdir().unwrap();
    let mut cmd = Command::cargo_bin("xlog").unwrap();
    cmd.arg("init")
        .env("XLOG_HOME", temp_dir.path())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("xlog").unwrap();
    cmd.arg("add")
        .arg("Test entry #test")
        .env("XLOG_HOME", temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Entry 1 added"));
}

#[test]
fn test_xlog_view() {
    let temp_dir = tempdir().unwrap();
    let mut cmd = Command::cargo_bin("xlog").unwrap();
    cmd.arg("init")
        .env("XLOG_HOME", temp_dir.path())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("xlog").unwrap();
    cmd.arg("add")
        .arg("Test entry #test")
        .env("XLOG_HOME", temp_dir.path())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("xlog").unwrap();
    cmd.arg("view")
        .env("XLOG_HOME", temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Test entry"));
}

#[test]
fn test_xlog_remove() {
    let temp_dir = tempdir().unwrap();
    let mut cmd = Command::cargo_bin("xlog").unwrap();
    cmd.arg("init")
        .env("XLOG_HOME", temp_dir.path())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("xlog").unwrap();
    cmd.arg("add")
        .arg("Test entry #test")
        .env("XLOG_HOME", temp_dir.path())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("xlog").unwrap();
    cmd.arg("remove")
        .arg("1")
        .env("XLOG_HOME", temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Entry 1 removed"));
}

#[test]
fn test_xlog_edit() {
    let temp_dir = tempdir().unwrap();
    let mut cmd = Command::cargo_bin("xlog").unwrap();
    cmd.arg("init")
        .env("XLOG_HOME", temp_dir.path())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("xlog").unwrap();
    cmd.arg("add")
        .arg("Test entry #test")
        .env("XLOG_HOME", temp_dir.path())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("xlog").unwrap();
    cmd.arg("edit")
        .arg("1")
        .arg("Updated entry")
        .env("XLOG_HOME", temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Entry updated"));
}

#[test]
fn test_xlog_search() {
    let temp_dir = tempdir().unwrap();
    let mut cmd = Command::cargo_bin("xlog").unwrap();
    cmd.arg("init")
        .env("XLOG_HOME", temp_dir.path())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("xlog").unwrap();
    cmd.arg("add")
        .arg("Test entry #test")
        .env("XLOG_HOME", temp_dir.path())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("xlog").unwrap();
    cmd.arg("search")
        .arg("Test")
        .env("XLOG_HOME", temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Test entry"));
}

#[test]
fn test_xlog_export() {
    let temp_dir = tempdir().unwrap();
    let mut cmd = Command::cargo_bin("xlog").unwrap();
    cmd.arg("init")
        .env("XLOG_HOME", temp_dir.path())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("xlog").unwrap();
    cmd.arg("add")
        .arg("Test entry #test")
        .env("XLOG_HOME", temp_dir.path())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("xlog").unwrap();
    cmd.arg("export")
        .arg("--format")
        .arg("json")
        .env("XLOG_HOME", temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Journal exported successfully"));
}

#[test]
fn test_xlog_backup() {
    let temp_dir = tempdir().unwrap();
    let mut cmd = Command::cargo_bin("xlog").unwrap();
    cmd.arg("init")
        .env("XLOG_HOME", temp_dir.path())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("xlog").unwrap();
    cmd.arg("add")
        .arg("Test entry #test")
        .env("XLOG_HOME", temp_dir.path())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("xlog").unwrap();
    cmd.arg("backup")
        .arg("--action")
        .arg("create")
        .env("XLOG_HOME", temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Backup created"));
}
