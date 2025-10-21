# Web3 Wallet CLI

一个安全、专业级的 Web3 钱包命令行工具，支持 BIP39/BIP44 标准和 MetaMask 兼容性。

[English](#english) | [中文](#中文)

## 中文

### 🚀 功能特性

- **🔐 高级安全性**: 使用 AES-256-GCM 加密和 Argon2id 密钥派生
- **📝 BIP39/BIP44 兼容**: 完全支持行业标准助记词和分层确定性钱包
- **🦊 MetaMask 兼容**: 生成与 MetaMask 完全兼容的地址
- **🌐 多网络支持**: 支持主网、Sepolia、Goerli、Holesky 测试网
- **💾 安全存储**: 密码保护的加密钱包文件
- **⚡ 批量地址生成**: 从 HD 钱包批量派生多个地址
- **📊 多种输出格式**: 支持表格和 JSON 格式输出
- **🔍 钱包管理**: 列出、加载和管理保存的钱包

### 🛠 技术栈

| 分类 | 技术 | 版本 | 用途 |
|------|------|------|------|
| **编程语言** | Rust | 2021 Edition | 系统级性能和内存安全 |
| **CLI 框架** | Clap | 4.0 | 命令行参数解析和帮助 |
| **加密技术** | AES-GCM, Argon2, PBKDF2 | - | AES-256-GCM 加密和安全密钥派生 |
| **区块链** | Ethers, BIP39 | 2.0 | 以太坊地址生成和 BIP39 助记词 |
| **异步运行时** | Tokio | 1.0 | 异步文件 I/O 操作 |
| **序列化** | Serde | - | JSON 序列化和密钥库格式 |
| **安全性** | Zeroize | - | 敏感数据内存安全清除 |

### 📁 项目架构

```
web3wallet-cli/
├── src/
│   ├── main.rs              # CLI 入口点和命令处理器
│   ├── lib.rs               # 库导出和 WalletConfig
│   ├── config.rs            # 配置常量和设置
│   ├── errors.rs            # 全面的错误类型定义
│   ├── utils.rs             # 验证工具函数
│   ├── models/              # 数据模型
│   │   ├── wallet.rs        # 钱包结构和操作
│   │   ├── address.rs       # 地址模型
│   │   ├── keystore.rs      # 加密钱包存储格式
│   │   └── command.rs       # 命令结构
│   └── services/            # 业务逻辑层
│       ├── walletmanager.rs # 高级钱包操作
│       ├── crypto.rs        # 加密/解密操作
│       └── mnemonic.rs      # BIP39 助记词生成和验证
├── tests/                   # 集成测试
│   ├── test_create_command.rs
│   ├── test_import_command.rs
│   ├── test_derive_command.rs
│   └── test_list_command.rs
└── Cargo.toml              # 项目配置
```

### 🔧 安装

#### 前置要求

- Rust 1.70+ 和 Cargo
- Git (可选，用于源码安装)

#### 从源码编译

```bash
# 克隆仓库
git clone <repository-url>
cd web3wallet-cli

# 编译项目
cargo build --release

# 安装到系统路径
cargo install --path .
```

#### 验证安装

```bash
web3wallet --help
```

### 📖 使用方法

#### 全局选项

```bash
web3wallet [OPTIONS] <COMMAND>

选项:
  -v, --verbose              启用详细日志记录
  -o, --output <FORMAT>      输出格式 [table, json]
  -c, --config <PATH>        自定义配置文件路径
  -h, --help                 显示帮助信息
  -V, --version              显示版本信息
```

#### 1. 创建新钱包

生成一个新的 BIP39/BIP44 兼容钱包：

```bash
# 创建 12 词助记词钱包（默认）
web3wallet create

# 创建 24 词助记词钱包
web3wallet create --words 24

# 创建钱包并保存到文件
web3wallet create --words 12 --save my-wallet --network mainnet

# JSON 格式输出
web3wallet create --output json
```

**示例输出:**
```
🎉 钱包创建成功！

助记词:  abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about
地址:    0x9858EfFD232B4033E47d90003D41EC34EcaEda94
网络:    mainnet
类型:    HD 钱包 (BIP44)
路径:    m/44'/60'/0'/0/0

⚠️  请安全保存您的助记词！它是恢复钱包的唯一方式。
```

#### 2. 导入现有钱包

从助记词或私钥导入钱包：

```bash
# 从助记词导入
web3wallet import --mnemonic "your twelve word mnemonic phrase here..."

# 从私钥导入
web3wallet import --private-key 0x1234567890abcdef...

# 导入并保存
web3wallet import --mnemonic "..." --save imported-wallet --network sepolia
```

#### 3. 加载保存的钱包

解密并显示之前保存的钱包：

```bash
# 加载钱包（需要密码）
web3wallet load my-wallet.json

# 只显示地址（无需密码）
web3wallet load my-wallet.json --address-only

# 从 HD 钱包派生特定地址
web3wallet load my-wallet.json --derive 5
```

#### 4. 列出所有钱包

显示钱包目录中的所有保存的钱包：

```bash
# 列出所有钱包
web3wallet list

# JSON 格式输出
web3wallet list --output json

# 指定自定义钱包目录
web3wallet list --path /custom/wallet/path
```

#### 5. 批量地址派生

从 HD 钱包生成多个地址：

```bash
# 从保存的钱包文件派生地址
web3wallet derive --from-file my-wallet.json --count 10

# 从助记词直接派生
web3wallet derive --mnemonic "your mnemonic..." --count 5 --start-index 0

# 使用自定义派生路径
web3wallet derive --from-file wallet.json --path "m/44'/60'/0'/0" --count 3
```

### ⚙️ 配置

#### 默认配置

- **钱包目录**: `~/.web3wallet/wallets/`
- **默认网络**: `mainnet`
- **派生路径**: `m/44'/60'/0'/0` (以太坊 BIP44 标准)
- **加密算法**: AES-256-GCM
- **密钥派生**: Argon2id (内存: 47,104 KB, 迭代: 1)

#### 支持的网络

| 网络 | 描述 | Chain ID |
|------|------|----------|
| `mainnet` | 以太坊主网 | 1 |
| `sepolia` | Sepolia 测试网 | 11155111 |
| `goerli` | Goerli 测试网 | 5 |
| `holesky` | Holesky 测试网 | 17000 |

#### 密码要求

保存钱包时的密码必须满足：
- 长度 8-1024 字符
- 包含小写字母
- 包含大写字母
- 包含数字
- 包含特殊字符

### 🔒 安全特性

#### 加密规格

- **对称加密**: AES-256-GCM
- **密钥派生**: Argon2id (推荐) 或 PBKDF2 (兼容模式)
- **MAC 验证**: HMAC-SHA256
- **随机性**: 加密安全的随机数生成器
- **内存安全**: 使用 `zeroize` 清除敏感数据

#### 密钥库格式

钱包以 JSON 格式加密存储：

```json
{
  "version": "1.0.0",
  "metadata": {
    "alias": "my-wallet",
    "address": "0x...",
    "created_at": "2024-01-01T00:00:00Z",
    "network": "mainnet",
    "wallet_type": "HDWallet"
  },
  "crypto": {
    "cipher": "aes-256-gcm",
    "ciphertext": "...",
    "iv": "...",
    "kdf": {
      "type": "argon2id",
      "params": {...}
    },
    "mac": "..."
  }
}
```

### 🧪 测试

运行完整的测试套件：

```bash
# 运行所有测试
cargo test

# 运行特定命令测试
cargo test test_create_command
cargo test test_import_command
cargo test test_derive_command
cargo test test_list_command

# 运行测试并显示输出
cargo test -- --nocapture

# 使用测试密码环境变量
TEST_WALLET_PASSWORD=test cargo test
```

**测试覆盖率**: 27+ 集成测试覆盖所有命令和边界情况

### 📝 示例用例

#### 开发者工作流

```bash
# 1. 为开发创建新钱包
web3wallet create --words 12 --save dev-wallet --network sepolia

# 2. 派生多个测试地址
web3wallet derive --from-file dev-wallet.json --count 10

# 3. 导入现有的 MetaMask 钱包
web3wallet import --mnemonic "your metamask mnemonic" --save metamask-backup

# 4. 列出所有管理的钱包
web3wallet list --output json
```

#### 安全备份流程

```bash
# 1. 创建主钱包
web3wallet create --words 24 --save master-wallet

# 2. 验证钱包可以正确加载
web3wallet load master-wallet.json --address-only

# 3. 导出多个地址用于监控
web3wallet derive --from-file master-wallet.json --count 20 --output json > addresses.json
```

### 🚨 安全注意事项

1. **助记词安全**:
   - 永远不要与他人分享您的助记词
   - 将助记词存储在安全的离线位置
   - 考虑使用硬件钱包进行长期存储

2. **密码安全**:
   - 使用强密码保护保存的钱包
   - 不要重复使用密码
   - 考虑使用密码管理器

3. **文件安全**:
   - 定期备份钱包文件
   - 确保钱包目录权限正确
   - 在删除钱包文件前三思

4. **网络安全**:
   - 在测试网上验证操作后再在主网使用
   - 验证网络配置的正确性

### 🤝 贡献

欢迎贡献！请遵循以下步骤：

1. Fork 此仓库
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 打开 Pull Request

### 📄 许可证

此项目基于 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

### 🐛 问题报告

如果您发现错误或有功能请求，请在 [GitHub Issues](https://github.com/your-repo/web3wallet-cli/issues) 中提交。

### 📞 支持

- 文档: 查看此 README 和内置帮助 (`web3wallet --help`)
- 问题: GitHub Issues
- 讨论: GitHub Discussions

---

## English

### 🚀 Features

- **🔐 Advanced Security**: AES-256-GCM encryption with Argon2id key derivation
- **📝 BIP39/BIP44 Compliant**: Full support for industry-standard mnemonics and HD wallets
- **🦊 MetaMask Compatible**: Generates addresses fully compatible with MetaMask
- **🌐 Multi-Network Support**: Mainnet, Sepolia, Goerli, Holesky testnets
- **💾 Secure Storage**: Password-protected encrypted wallet files
- **⚡ Batch Address Generation**: Derive multiple addresses from HD wallets
- **📊 Multiple Output Formats**: Table and JSON output support
- **🔍 Wallet Management**: List, load, and manage saved wallets

### 🛠 Technology Stack

| Category | Technology | Version | Purpose |
|----------|------------|---------|---------|
| **Language** | Rust | 2021 Edition | Systems-level performance and memory safety |
| **CLI Framework** | Clap | 4.0 | Command-line argument parsing and help |
| **Cryptography** | AES-GCM, Argon2, PBKDF2 | - | AES-256-GCM encryption with secure key derivation |
| **Blockchain** | Ethers, BIP39 | 2.0 | Ethereum address generation and BIP39 mnemonics |
| **Async Runtime** | Tokio | 1.0 | Async file I/O operations |
| **Serialization** | Serde | - | JSON serialization and keystore format |
| **Security** | Zeroize | - | Secure memory clearing for sensitive data |

### 📁 Project Architecture

```
web3wallet-cli/
├── src/
│   ├── main.rs              # CLI entry point and command handlers
│   ├── lib.rs               # Library exports and WalletConfig
│   ├── config.rs            # Configuration constants and settings
│   ├── errors.rs            # Comprehensive error type definitions
│   ├── utils.rs             # Validation utility functions
│   ├── models/              # Data models
│   │   ├── wallet.rs        # Wallet structure and operations
│   │   ├── address.rs       # Address model
│   │   ├── keystore.rs      # Encrypted wallet storage format
│   │   └── command.rs       # Command structures
│   └── services/            # Business logic layer
│       ├── walletmanager.rs # High-level wallet operations
│       ├── crypto.rs        # Encryption/decryption operations
│       └── mnemonic.rs      # BIP39 mnemonic generation and validation
├── tests/                   # Integration tests
│   ├── test_create_command.rs
│   ├── test_import_command.rs
│   ├── test_derive_command.rs
│   └── test_list_command.rs
└── Cargo.toml              # Project configuration
```

### 🔧 Installation

#### Prerequisites

- Rust 1.70+ and Cargo
- Git (optional, for source installation)

#### Build from Source

```bash
# Clone the repository
git clone <repository-url>
cd web3wallet-cli

# Build the project
cargo build --release

# Install to system path
cargo install --path .
```

#### Verify Installation

```bash
web3wallet --help
```

### 📖 Usage

#### Global Options

```bash
web3wallet [OPTIONS] <COMMAND>

Options:
  -v, --verbose              Enable verbose logging
  -o, --output <FORMAT>      Output format [table, json]
  -c, --config <PATH>        Custom configuration file path
  -h, --help                 Show help information
  -V, --version              Show version information
```

#### 1. Create New Wallet

Generate a new BIP39/BIP44 compliant wallet:

```bash
# Create 12-word mnemonic wallet (default)
web3wallet create

# Create 24-word mnemonic wallet
web3wallet create --words 24

# Create wallet and save to file
web3wallet create --words 12 --save my-wallet --network mainnet

# JSON output format
web3wallet create --output json
```

**Example Output:**
```
🎉 Wallet created successfully!

Mnemonic:  abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about
Address:   0x9858EfFD232B4033E47d90003D41EC34EcaEda94
Network:   mainnet
Type:      HD Wallet (BIP44)
Path:      m/44'/60'/0'/0/0

⚠️  Please save your mnemonic phrase securely! It's the only way to recover your wallet.
```

#### 2. Import Existing Wallet

Import wallet from mnemonic or private key:

```bash
# Import from mnemonic
web3wallet import --mnemonic "your twelve word mnemonic phrase here..."

# Import from private key
web3wallet import --private-key 0x1234567890abcdef...

# Import and save
web3wallet import --mnemonic "..." --save imported-wallet --network sepolia
```

#### 3. Load Saved Wallet

Decrypt and display previously saved wallet:

```bash
# Load wallet (requires password)
web3wallet load my-wallet.json

# Address-only mode (no password required)
web3wallet load my-wallet.json --address-only

# Derive specific address from HD wallet
web3wallet load my-wallet.json --derive 5
```

#### 4. List All Wallets

Display all saved wallets in the wallet directory:

```bash
# List all wallets
web3wallet list

# JSON output format
web3wallet list --output json

# Specify custom wallet directory
web3wallet list --path /custom/wallet/path
```

#### 5. Batch Address Derivation

Generate multiple addresses from HD wallet:

```bash
# Derive addresses from saved wallet file
web3wallet derive --from-file my-wallet.json --count 10

# Derive directly from mnemonic
web3wallet derive --mnemonic "your mnemonic..." --count 5 --start-index 0

# Use custom derivation path
web3wallet derive --from-file wallet.json --path "m/44'/60'/0'/0" --count 3
```

### ⚙️ Configuration

#### Default Settings

- **Wallet Directory**: `~/.web3wallet/wallets/`
- **Default Network**: `mainnet`
- **Derivation Path**: `m/44'/60'/0'/0` (Ethereum BIP44 standard)
- **Encryption**: AES-256-GCM
- **Key Derivation**: Argon2id (Memory: 47,104 KB, Iterations: 1)

#### Supported Networks

| Network | Description | Chain ID |
|---------|-------------|----------|
| `mainnet` | Ethereum Mainnet | 1 |
| `sepolia` | Sepolia Testnet | 11155111 |
| `goerli` | Goerli Testnet | 5 |
| `holesky` | Holesky Testnet | 17000 |

#### Password Requirements

Passwords for saving wallets must have:
- Length: 8-1024 characters
- Lowercase letter
- Uppercase letter
- Digit
- Special character

### 🔒 Security Features

#### Encryption Specifications

- **Symmetric Encryption**: AES-256-GCM
- **Key Derivation**: Argon2id (recommended) or PBKDF2 (legacy compatibility)
- **MAC Verification**: HMAC-SHA256
- **Randomness**: Cryptographically secure random number generation
- **Memory Safety**: Uses `zeroize` to clear sensitive data

#### Keystore Format

Wallets are stored encrypted in JSON format:

```json
{
  "version": "1.0.0",
  "metadata": {
    "alias": "my-wallet",
    "address": "0x...",
    "created_at": "2024-01-01T00:00:00Z",
    "network": "mainnet",
    "wallet_type": "HDWallet"
  },
  "crypto": {
    "cipher": "aes-256-gcm",
    "ciphertext": "...",
    "iv": "...",
    "kdf": {
      "type": "argon2id",
      "params": {...}
    },
    "mac": "..."
  }
}
```

### 🧪 Testing

Run the complete test suite:

```bash
# Run all tests
cargo test

# Run specific command tests
cargo test test_create_command
cargo test test_import_command
cargo test test_derive_command
cargo test test_list_command

# Run tests with output
cargo test -- --nocapture

# Use test password environment variable
TEST_WALLET_PASSWORD=test cargo test
```

**Test Coverage**: 27+ integration tests covering all commands and edge cases

### 📝 Example Use Cases

#### Developer Workflow

```bash
# 1. Create new wallet for development
web3wallet create --words 12 --save dev-wallet --network sepolia

# 2. Derive multiple test addresses
web3wallet derive --from-file dev-wallet.json --count 10

# 3. Import existing MetaMask wallet
web3wallet import --mnemonic "your metamask mnemonic" --save metamask-backup

# 4. List all managed wallets
web3wallet list --output json
```

#### Secure Backup Process

```bash
# 1. Create master wallet
web3wallet create --words 24 --save master-wallet

# 2. Verify wallet can be loaded correctly
web3wallet load master-wallet.json --address-only

# 3. Export multiple addresses for monitoring
web3wallet derive --from-file master-wallet.json --count 20 --output json > addresses.json
```

### 🚨 Security Considerations

1. **Mnemonic Security**:
   - Never share your mnemonic phrase with anyone
   - Store mnemonic phrases in secure, offline locations
   - Consider using hardware wallets for long-term storage

2. **Password Security**:
   - Use strong passwords to protect saved wallets
   - Don't reuse passwords
   - Consider using a password manager

3. **File Security**:
   - Regularly backup wallet files
   - Ensure proper permissions on wallet directory
   - Think twice before deleting wallet files

4. **Network Security**:
   - Test operations on testnets before mainnet use
   - Verify network configuration correctness

### 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

### 🐛 Bug Reports

If you find a bug or have a feature request, please submit an issue at [GitHub Issues](https://github.com/your-repo/web3wallet-cli/issues).

### 📞 Support

- Documentation: See this README and built-in help (`web3wallet --help`)
- Issues: GitHub Issues
- Discussions: GitHub Discussions

---

**Made with ❤️ by Frank** | **Version**: 0.1.0 | **License**: MIT