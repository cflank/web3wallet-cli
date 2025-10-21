use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

const VALID_MNEMONIC_12: &str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
const VALID_MNEMONIC_24: &str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";
const VALID_PRIVATE_KEY: &str = "0x4c0883a69102937d6231471b5dbb6204fe512961708279c1e3ae83da5e56df1a";
const EXPECTED_ADDRESS: &str = "0x9858EfFD232B4033E47d90003D41EC34EcaEda94";
const EXPECTED_PRIVATE_KEY_ADDRESS: &str = "0xc85117289fec250ddbab37f2a597af5bf950e3b0";

#[test]
fn test_import_command_mnemonic_12(){
    let mut cmd = Command::cargo_bin("web3wallet").unwrap();
    cmd.args(&["import", "--mnemonic", VALID_MNEMONIC_12]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Address:"))
        .stdout(predicate::str::contains(&EXPECTED_ADDRESS.to_lowercase()));
}

#[test]
fn test_import_command_mnemonic_24(){
    let mut cmd = Command::cargo_bin("web3wallet").unwrap();
    cmd.args(&["import", "--mnemonic", VALID_MNEMONIC_24]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Address:"));
}

#[test]
fn test_import_command_private_key(){
    let mut cmd = Command::cargo_bin("web3wallet").unwrap();
    cmd.args(&["import", "--private-key", VALID_PRIVATE_KEY]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Address:"))
        .stdout(predicate::str::contains(EXPECTED_PRIVATE_KEY_ADDRESS));
}

#[test]
fn test_import_command_invalid_mnemonic() {
    let mut cmd = Command::cargo_bin("web3wallet").unwrap();
    cmd.args(&["import", "--mnemonic", "invalid mnemonic phrase"]);

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("CRYPTO_002"));
}


#[test]
fn test_import_command_invalid_private_key() {
    let mut cmd = Command::cargo_bin("web3wallet").unwrap();
    cmd.args(&["import", "--private-key", "invalid_key"]);

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("VALIDATION_001"));
}

#[test]
fn test_import_command_conflicting_options() {
    let mut cmd = Command::cargo_bin("web3wallet").unwrap();
    cmd.args(&[
        "import",
        "--mnemonic", VALID_MNEMONIC_12,
        "--private-key", VALID_PRIVATE_KEY,
    ]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("cannot be used with"));
}

#[test]
fn test_import_command_with_save() {
    let temp_dir = TempDir::new().unwrap();
    let wallet_path = temp_dir.path().join("imported-wallet.json");

    let mut cmd = Command::cargo_bin("web3wallet").unwrap();
    cmd.env("TEST_WALLET_PASSWORD", "TestPassword123!")
        .args(&[
            "import",
            "--mnemonic", VALID_MNEMONIC_12,
            "--save", wallet_path.to_str().unwrap(),
        ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Wallet saved to:"));

    // Verify file was created
    assert!(wallet_path.exists());
}

#[test]
fn test_import_command_json_output() {
    let mut cmd = Command::cargo_bin("web3wallet").unwrap();
    cmd.args(&[
        "import",
        "--mnemonic", VALID_MNEMONIC_12,
        "--output", "json",
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(r#""success": true"#))
        .stdout(predicate::str::contains(r#""address":"#))
        .stdout(predicate::str::contains(&EXPECTED_ADDRESS.to_lowercase()));
}

#[test]
fn test_import_command_metamask_compatibility() {
    let mut cmd = Command::cargo_bin("web3wallet").unwrap();
    cmd.args(&[
        "import",
        "--mnemonic", VALID_MNEMONIC_12,
        "--output", "json",
    ]);

    let output = cmd.assert().success().get_output().stdout.clone();
    let output_str = String::from_utf8(output).unwrap();

    // Should generate the same address as MetaMask for this mnemonic
    assert!(output_str.contains(&EXPECTED_ADDRESS.to_lowercase()));
}

#[test]
fn test_import_command_custom_network() {
    let mut cmd = Command::cargo_bin("web3wallet").unwrap();
    cmd.args(&[
        "import",
        "--mnemonic", VALID_MNEMONIC_12,
        "--network", "sepolia",
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Network:  sepolia"));
}

#[test]
fn test_import_command_performance() {
    use std::time::Instant;

    let start = Instant::now();
    let mut cmd = Command::cargo_bin("web3wallet").unwrap();
    cmd.args(&["import", "--mnemonic", VALID_MNEMONIC_12]);

    cmd.assert().success();

    let duration = start.elapsed();
    assert!(duration.as_secs() < 1, "Command took {:?}, should be <1s", duration);
}


#[test]
fn test_import_command_help() {
    let mut cmd = Command::cargo_bin("web3wallet").unwrap();
    cmd.args(&["import", "--help"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Import an existing wallet"))
        .stdout(predicate::str::contains("--mnemonic"))
        .stdout(predicate::str::contains("--private-key"))
        .stdout(predicate::str::contains("--save"));
}


#[test]
fn test_import_command_missing_source() {
    let mut cmd = Command::cargo_bin("web3wallet").unwrap();
    cmd.arg("import");

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("INPUT_003")); // Missing required parameter
}