# Web3 Wallet CLI Release Build Script
# 构建并发布 Web3 钱包 CLI 工具

param(
    [string]$Version = "",
    [string]$Tag = "",
    [switch]$SkipTests = $false,
    [switch]$CreateRelease = $true
)

# 配置
$ProjectName = "web3wallet-cli"
$BinaryName = "web3wallet"
$Owner = "cflank"
$Repo = "web3wallet-cli"

# 颜色输出函数
function Write-Info { param($Message) Write-Host "ℹ️  $Message" -ForegroundColor Blue }
function Write-Success { param($Message) Write-Host "✅ $Message" -ForegroundColor Green }
function Write-Error { param($Message) Write-Host "❌ $Message" -ForegroundColor Red }
function Write-Warning { param($Message) Write-Host "⚠️  $Message" -ForegroundColor Yellow }

# 检查前置条件
function Test-Prerequisites {
    Write-Info "检查前置条件..."

    # 检查 Rust
    try {
        $rustVersion = cargo --version
        Write-Success "Rust: $rustVersion"
    }
    catch {
        Write-Error "未找到 Rust。请安装 Rust: https://rustup.rs/"
        return $false
    }

    # 检查 Git
    try {
        $gitVersion = git --version
        Write-Success "Git: $gitVersion"
    }
    catch {
        Write-Error "未找到 Git。请安装 Git。"
        return $false
    }

    # 检查 GitHub CLI
    try {
        $ghVersion = gh --version
        Write-Success "GitHub CLI: $($ghVersion[0])"
    }
    catch {
        Write-Error "未找到 GitHub CLI。请运行: winget install --id GitHub.cli"
        return $false
    }

    # 检查 GitHub 登录状态
    try {
        $authStatus = gh auth status 2>&1
        if ($authStatus -match "Logged in") {
            Write-Success "GitHub 已登录"
            return $true
        }
        else {
            Write-Error "未登录 GitHub。请运行: gh auth login"
            return $false
        }
    }
    catch {
        Write-Error "无法检查 GitHub 登录状态"
        return $false
    }
}

# 获取版本信息
function Get-VersionInfo {
    if ($Version -ne "") {
        return $Version
    }

    # 从 Cargo.toml 读取版本
    $cargoToml = Get-Content "Cargo.toml" -Raw
    if ($cargoToml -match 'version\s*=\s*"([^"]+)"') {
        return $matches[1]
    }

    Write-Error "无法从 Cargo.toml 获取版本信息"
    return $null
}

# 清理旧的构建文件
function Clear-BuildArtifacts {
    Write-Info "清理旧的构建文件..."

    if (Test-Path "target/release") {
        Remove-Item "target/release/$BinaryName.exe" -ErrorAction SilentlyContinue
    }

    if (Test-Path "release") {
        Remove-Item "release" -Recurse -Force -ErrorAction SilentlyContinue
    }

    New-Item -ItemType Directory -Path "release" -Force | Out-Null
    Write-Success "构建目录已清理"
}

# 运行测试
function Invoke-Tests {
    if ($SkipTests) {
        Write-Warning "跳过测试"
        return $true
    }

    Write-Info "运行测试..."

    try {
        $env:TEST_WALLET_PASSWORD = "TestPassword123!"
        cargo test --release
        Write-Success "所有测试通过"
        return $true
    }
    catch {
        Write-Error "测试失败"
        return $false
    }
}

# 构建 Release 版本
function Build-Release {
    param([string]$Target = "")

    Write-Info "构建 Release 版本..."

    try {
        if ($Target -eq "") {
            cargo build --release
            $binaryPath = "target/release/$BinaryName.exe"
        } else {
            cargo build --release --target $Target
            $binaryPath = "target/$Target/release/$BinaryName.exe"
        }

        if (Test-Path $binaryPath) {
            Write-Success "构建成功: $binaryPath"
            return $binaryPath
        } else {
            Write-Error "构建失败: 找不到二进制文件"
            return $null
        }
    }
    catch {
        Write-Error "构建过程出现错误: $($_.Exception.Message)"
        return $null
    }
}

# 创建发布包
function New-ReleasePackage {
    param(
        [string]$BinaryPath,
        [string]$Version,
        [string]$Platform = "windows-x64"
    )

    Write-Info "创建发布包..."

    $packageName = "$ProjectName-$Version-$Platform"
    $packageDir = "release/$packageName"

    # 创建包目录
    New-Item -ItemType Directory -Path $packageDir -Force | Out-Null

    # 复制二进制文件
    Copy-Item $BinaryPath "$packageDir/$BinaryName.exe"

    # 复制文档
    Copy-Item "README.md" $packageDir -ErrorAction SilentlyContinue

    # 创建 LICENSE 文件（如果不存在）
    if (-not (Test-Path "LICENSE")) {
        @"
MIT License

Copyright (c) 2024 Frank

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"@ | Out-File -FilePath "LICENSE" -Encoding UTF8
    }
    Copy-Item "LICENSE" $packageDir -ErrorAction SilentlyContinue

    # 创建使用说明
    @"
# Web3 Wallet CLI v$Version

## 快速开始

1. 将 web3wallet.exe 添加到系统 PATH 或直接使用完整路径
2. 打开命令行终端
3. 运行: web3wallet --help

## 基本用法

### 创建新钱包
```
web3wallet create
web3wallet create --words 24 --save my-wallet
```

### 导入钱包
```
web3wallet import --mnemonic "your twelve word mnemonic phrase here"
web3wallet import --private-key 0x1234567890abcdef...
```

### 加载钱包
```
web3wallet load my-wallet.json
web3wallet list
```

### 派生地址
```
web3wallet derive --from-file my-wallet.json --count 10
```

## 更多信息

- GitHub: https://github.com/$Owner/$Repo
- 完整文档: 参见 README.md
"@ | Out-File -FilePath "$packageDir/USAGE.txt" -Encoding UTF8

    # 创建 ZIP 包
    $zipPath = "release/$packageName.zip"
    Compress-Archive -Path $packageDir -DestinationPath $zipPath -Force

    Write-Success "发布包已创建: $zipPath"
    return $zipPath
}

# 创建 GitHub Release
function New-GitHubRelease {
    param(
        [string]$Version,
        [string]$Tag,
        [string[]]$Assets
    )

    Write-Info "创建 GitHub Release..."

    # 生成发布说明
    $releaseNotes = @"
# Web3 Wallet CLI v$Version

## 🚀 新功能和改进

- ✅ 完整的 BIP39/BIP44 钱包支持
- ✅ MetaMask 兼容性
- ✅ 多网络支持（主网、测试网）
- ✅ 安全的 AES-256-GCM 加密
- ✅ 批量地址派生
- ✅ 命令行友好的界面

## 📦 下载

### Windows x64
- **推荐**: 下载 ``$ProjectName-$Version-windows-x64.zip``
- 解压后将 ``web3wallet.exe`` 添加到系统 PATH

## 🔧 安装方法

### 方法一：下载二进制文件（推荐）
1. 下载适合您系统的压缩包
2. 解压到任意目录
3. 将目录添加到系统 PATH 或直接使用完整路径

### 方法二：从源码编译
```bash
git clone https://github.com/$Owner/$Repo.git
cd $Repo
cargo build --release
```

## 🚀 快速开始

```bash
# 显示帮助
web3wallet --help

# 创建新钱包
web3wallet create --words 12

# 导入现有钱包
web3wallet import --mnemonic "your mnemonic phrase here"

# 列出所有钱包
web3wallet list
```

## 🔒 安全提醒

⚠️ **重要**:
- 请安全保管您的助记词和私钥
- 不要与任何人分享您的助记词
- 建议在测试网上先验证操作

## 📝 完整文档

详细使用说明请参见 [README.md](https://github.com/$Owner/$Repo/blob/main/README.md)

---

**校验和**
如需验证下载文件的完整性，请联系项目维护者获取文件哈希值。
"@

    try {
        # 创建 tag（如果不存在）
        if ($Tag -eq "") {
            $Tag = "v$Version"
        }

        # 检查 tag 是否已存在
        $existingTag = git tag -l $Tag
        if ($existingTag) {
            Write-Warning "Tag $Tag 已存在，将删除并重新创建"
            git tag -d $Tag
            git push origin --delete $Tag 2>$null
        }

        # 创建并推送 tag
        git tag $Tag
        git push origin $Tag

        # 创建 Release
        $assetArgs = @()
        foreach ($asset in $Assets) {
            $assetArgs += "--attach"
            $assetArgs += $asset
        }

        gh release create $Tag $assetArgs --title "Web3 Wallet CLI v$Version" --notes $releaseNotes

        Write-Success "GitHub Release 已创建: https://github.com/$Owner/$Repo/releases/tag/$Tag"
        return $true
    }
    catch {
        Write-Error "创建 GitHub Release 失败: $($_.Exception.Message)"
        return $false
    }
}

# 主执行流程
function Invoke-Main {
    Write-Host "=== Web3 Wallet CLI Release Builder ===" -ForegroundColor Cyan
    Write-Host ""

    # 检查前置条件
    if (-not (Test-Prerequisites)) {
        exit 1
    }

    # 获取版本信息
    $currentVersion = Get-VersionInfo
    if (-not $currentVersion) {
        exit 1
    }

    Write-Info "当前版本: $currentVersion"

    # 确认版本
    if ($Version -eq "") {
        $Version = $currentVersion
    }

    Write-Info "发布版本: $Version"

    # 清理构建文件
    Clear-BuildArtifacts

    # 运行测试
    if (-not (Invoke-Tests)) {
        exit 1
    }

    # 构建 Release
    $binaryPath = Build-Release
    if (-not $binaryPath) {
        exit 1
    }

    # 创建发布包
    $packagePath = New-ReleasePackage -BinaryPath $binaryPath -Version $Version
    if (-not $packagePath) {
        exit 1
    }

    # 创建 GitHub Release
    if ($CreateRelease) {
        $success = New-GitHubRelease -Version $Version -Tag $Tag -Assets @($packagePath)
        if ($success) {
            Write-Success "🎉 Release 创建完成！"
            Write-Info "📦 下载地址: https://github.com/$Owner/$Repo/releases/latest"
        } else {
            Write-Error "Release 创建失败"
            exit 1
        }
    } else {
        Write-Success "🎉 构建完成！发布包位于: $packagePath"
    }
}

# 显示使用说明
if ($args -contains "-Help" -or $args -contains "--help" -or $args -contains "-h") {
    Write-Host "Web3 Wallet CLI Release Builder" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "用法:" -ForegroundColor Yellow
    Write-Host "  .\build_release.ps1                    # 使用默认版本构建并发布"
    Write-Host "  .\build_release.ps1 -Version 1.0.1     # 指定版本号"
    Write-Host "  .\build_release.ps1 -SkipTests         # 跳过测试"
    Write-Host "  .\build_release.ps1 -CreateRelease:`$false # 只构建，不发布到 GitHub"
    Write-Host ""
    Write-Host "选项:" -ForegroundColor Yellow
    Write-Host "  -Version <string>    指定版本号（默认从 Cargo.toml 读取）"
    Write-Host "  -Tag <string>        指定 Git tag（默认为 v{Version}）"
    Write-Host "  -SkipTests           跳过测试阶段"
    Write-Host "  -CreateRelease       是否创建 GitHub Release（默认: true）"
    exit 0
}

# 运行主流程
Invoke-Main