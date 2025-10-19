use assert_cmd::Command;
use predicates::prelude::*;

/// Test wallet list with no wallets
#[test]
fn test_list_command_empty() {
    let mut cmd = Command::cargo_bin("web3wallet").unwrap();
    cmd.arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No wallet found."));
}

/// Test wallet list JSON output
#[test]
fn test_list_command_json() {
    let mut cmd = Command::cargo_bin("web3wallet").unwrap();
    cmd.args(&["list", "--output", "json"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(r#""success": true"#));
}