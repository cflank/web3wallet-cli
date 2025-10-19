use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_create_command_default(){
    let mut cmd = Command::cargo_bin("web3wallet").unwrap();

    cmd.args(&["create"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Address:"))
        .stdout(predicate::str::contains("Mnemonic"))
        .stdout(predicate::str::contains("0x"));
}

#[test]
fn test_create_command_with_save(){
    let wallet_name = "test_wallet_temp"; // Unique name to avoid conflicts
    let password = "Test123!";

    // Calculate expected wallet path (default config location)
    let home_dir = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    let wallet_path = home_dir.join(".web3wallet").join("wallets").join(format!("{}.json", wallet_name));

    // Clean up any existing test wallet file
    let _ = std::fs::remove_file(&wallet_path);

    let mut cmd = Command::cargo_bin("web3wallet").unwrap();

    // Set environment variable for test mode
    cmd.env("TEST_WALLET_PASSWORD", password);
    cmd.args(&["create", "--save", wallet_name]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Wallet saved to"));

    // Check that wallet was created
    assert!(wallet_path.exists(), "Wallet file should exist at: {}", wallet_path.display());

    // Clean up test wallet file
    let _ = std::fs::remove_file(&wallet_path);
}

#[test]
fn test_create_command_invalid_word_count(){
    let mut cmd = Command::cargo_bin("web3wallet").unwrap();

    cmd.args(&["create", "--words", "16"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Word count must be 12 or 24"));
}

#[test]
fn test_load_command_with_password(){
    let wallet_name = "test_load_wallet_temp";
    let password = "Test123!";

    // Calculate expected wallet path (default config location)
    let home_dir = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    let wallet_path = home_dir.join(".web3wallet").join("wallets").join(format!("{}.json", wallet_name));

    // Clean up any existing test wallet file
    let _ = std::fs::remove_file(&wallet_path);

    // First create a wallet
    let mut create_cmd = Command::cargo_bin("web3wallet").unwrap();
    create_cmd.env("TEST_WALLET_PASSWORD", password);
    create_cmd.args(&["create", "--save", wallet_name]);
    create_cmd.assert().success();
    assert!(wallet_path.exists());

    // Then load the wallet
    let mut load_cmd = Command::cargo_bin("web3wallet").unwrap();
    load_cmd.env("TEST_WALLET_PASSWORD", password);
    load_cmd.args(&["load", &format!("{}.json", wallet_name)]); // Load command expects full filename

    load_cmd.assert()
            .success()
            .stdout(predicate::str::contains("Loading wallet from"))
            .stdout(predicate::str::contains("Address:"));

    // Clean up test wallet file
    let _ = std::fs::remove_file(&wallet_path);
}

#[test]
fn test_create_command_12_words() {
    let mut cmd = Command::cargo_bin("web3wallet").unwrap();
    cmd.args(&["create", "--words", "12"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Address:"));
}

/// Test wallet create with 24-word mnemonic
#[test]
fn test_create_command_24_words() {
    let mut cmd = Command::cargo_bin("web3wallet").unwrap();
    cmd.args(&["create", "--words", "24"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Address:"));
}

#[test]
fn test_create_command_json_output() {
    let mut cmd = Command::cargo_bin("web3wallet").unwrap();
    cmd.args(&["create", "--output", "json"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(r#""success": true"#))
        .stdout(predicate::str::contains(r#""address":"#))
        .stdout(predicate::str::contains(r#""mnemonic":"#));
}

/// Test wallet create with custom network
#[test]
fn test_create_command_custom_network() {
    let mut cmd = Command::cargo_bin("web3wallet").unwrap();
    cmd.args(&["create", "--network", "sepolia"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("sepolia network"))
        .stdout(predicate::str::contains("Network: sepolia"));
}

/// Test wallet create performance requirement (<1s)
#[test]
fn test_create_command_performance() {
    use std::time::Instant;

    let start = Instant::now();
    let mut cmd = Command::cargo_bin("web3wallet").unwrap();
    cmd.arg("create");

    cmd.assert().success();

    let duration = start.elapsed();
    assert!(duration.as_secs() < 1, "Command took {:?}, should be <1s", duration);
}

/// Test wallet create help text
#[test]
fn test_create_command_help() {
    let mut cmd = Command::cargo_bin("web3wallet").unwrap();
    cmd.args(&["create", "--help"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Create a new wallet"))
        .stdout(predicate::str::contains("--words"))
        .stdout(predicate::str::contains("--save"))
        .stdout(predicate::str::contains("--network"));
}

/// Test that created wallets have proper MetaMask-compatible addresses
#[test]
fn test_create_command_metamask_compatibility() {
    let mut cmd = Command::cargo_bin("web3wallet").unwrap();
    cmd.args(&["create", "--output", "json"]);

    let output = cmd.assert().success().get_output().stdout.clone();
    let output_str = String::from_utf8(output).unwrap();

    // Should contain a valid Ethereum address format
    assert!(output_str.contains("0x"));

    // Extract address and verify it's 42 characters (0x + 40 hex chars)
    if let Some(start) = output_str.find(r#""address":"0x"#) {
        let addr_start = start + r#""address":""#.len();
        let addr_end = addr_start + 42; // 0x + 40 hex chars
        if addr_end <= output_str.len() {
            let address = &output_str[addr_start..addr_end];
            assert!(address.starts_with("0x"));
            assert_eq!(address.len(), 42);
            assert!(address[2..].chars().all(|c| c.is_ascii_hexdigit()));
        }
    }
}