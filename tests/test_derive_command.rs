use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use std::fs;

/// Test wallet derive with valid path
#[test]
fn test_derive_command_valid_path() {
    // First create a valid wallet file using the create command
    let wallet_name = "test_derive_wallet";
    let password = "Test123!";

    // Calculate expected wallet path (default config location)
    let home_dir = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    let wallet_path = home_dir.join(".web3wallet").join("wallets").join(format!("{}.json", wallet_name));

    // Clean up any existing test wallet file
    let _ = std::fs::remove_file(&wallet_path);

    // Create a wallet first
    let mut create_cmd = Command::cargo_bin("web3wallet").unwrap();
    create_cmd.env("TEST_WALLET_PASSWORD", password);
    create_cmd.args(&["create", "--save", wallet_name]);
    create_cmd.assert().success();
    assert!(wallet_path.exists());

    // Now test derive command
    let mut cmd = Command::cargo_bin("web3wallet").unwrap();
    cmd.env("TEST_WALLET_PASSWORD", password);
    cmd.args(&[
        "derive",
        "--path", "m/44'/60'/0'/0/5",
        "--from-file", &format!("{}.json", wallet_name),
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("ADDRESS"));

    // Clean up test wallet file
    let _ = std::fs::remove_file(&wallet_path);
}

/// Test wallet derive with invalid path
#[test]
fn test_derive_command_invalid_path() {
    let mut cmd = Command::cargo_bin("web3wallet").unwrap();
    // Set environment variable to avoid password prompts
    cmd.env("TEST_WALLET_PASSWORD", "Test123!");
    cmd.args(&["derive", "--path", "invalid/path", "--from-file", "nonexistent.json"]);

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("CRYPTO_012").or(predicate::str::contains("Command failed")));
}

/// Test wallet derive with invalid derivation path format
#[test]
fn test_derive_command_invalid_derivation_path() {
    // First create a valid wallet file
    let wallet_name = "test_derive_invalid_path";
    let password = "Test123!";

    // Calculate expected wallet path (default config location)
    let home_dir = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    let wallet_path = home_dir.join(".web3wallet").join("wallets").join(format!("{}.json", wallet_name));

    // Clean up any existing test wallet file
    let _ = std::fs::remove_file(&wallet_path);

    // Create a wallet first
    let mut create_cmd = Command::cargo_bin("web3wallet").unwrap();
    create_cmd.env("TEST_WALLET_PASSWORD", password);
    create_cmd.args(&["create", "--save", wallet_name]);
    create_cmd.assert().success();
    assert!(wallet_path.exists());

    // Now test derive command with extremely large index that might cause path issues
    let mut cmd = Command::cargo_bin("web3wallet").unwrap();
    cmd.env("TEST_WALLET_PASSWORD", password);
    cmd.args(&[
        "derive",
        "--path", "4294967295", // 使用 u32::MAX，可能会导致路径问题
        "--from-file", &format!("{}.json", wallet_name),
    ]);

    // 这个测试可能会成功（如果大索引是合法的）或失败
    // 我们先运行看看会发生什么
    let output = cmd.output().unwrap();
    println!("Exit code: {}", output.status.code().unwrap_or(-1));
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    // Clean up test wallet file
    let _ = std::fs::remove_file(&wallet_path);
}

/// Test derivation path validation (ValidationError::InvalidAddressFormat)
#[test]
fn test_derivation_path_validation() {
    // 这个测试直接测试路径验证逻辑，验证正确的错误类型
    use web3wallet_cli::utils::validate_derivation_path;
    use web3wallet_cli::errors::ValidationError;

    // 测试无效的派生路径格式
    let invalid_paths = vec![
        "invalid_path",           // 完全无效的格式
        "/44'/60'/0'/0",         // 缺少 m 前缀
        "44'/60'/0'/0",          // 缺少 m/ 前缀
        "m/44'/60'/0'/0/",       // 末尾有斜杠
        "m//44'/60'/0'/0",       // 空组件
        "m/44'/60'//0'/0",       // 空组件
    ];

    for invalid_path in invalid_paths {
        let result = validate_derivation_path(invalid_path);
        match result {
            Err(web3wallet_cli::WalletError::Validation(ValidationError::InvalidAddressFormat { .. })) => {
                println!("Path validation correctly triggered for: {}", invalid_path);
            }
            Ok(_) => {
                panic!(" Expected validation error for invalid path: {}, but validation passed", invalid_path);
            }
            Err(other_error) => {
                panic!("Expected ValidationError::InvalidAddressFormat for path: {}, but got different error: {:?}", invalid_path, other_error);
            }
        }
    }

    // 测试有效路径应该通过
    let valid_paths = vec![
        "m/44'/60'/0'/0",
        "m/44'/60'/0'/0/0",
        "m/44'/60'/0'/0/123",
        "m/44'/60'/0'/0/4294967295", // 极大索引也应该通过基本格式验证
    ];

    for valid_path in valid_paths {
        let result = validate_derivation_path(valid_path);
        assert!(result.is_ok(), "Valid path should pass validation: {}", valid_path);
        println!("✅ Valid path correctly validated: {}", valid_path);
    }
}