# Windows Build Troubleshooting Guide

## Common Build Issues on Windows

### Issue 1: Network Timeout (crates.io download failures)

**Symptoms**:
```
error: failed to download from `https://static.crates.io/crates/cxxbridge-cmd/1.0.188/download`
Caused by: [28] Timeout was reached
```

**Solutions** (try in order):

#### A. Retry the Build
Sometimes it's just a temporary network issue:
```powershell
cargo clean
cargo build --release
```

#### B. Use the Cargo Config (Already Set Up)
The `.cargo/config.toml` file now has:
- 120 second timeout (instead of 30)
- 3 retry attempts
- Git fetch with CLI enabled

Just rebuild:
```powershell
cargo build --release
```

#### C. Pre-download Dependencies
Download all dependencies first without building:
```powershell
cargo fetch
cargo build --release
```

#### D. Use a Cargo Mirror (China/Asia users)
If crates.io is slow in your region, add to `~/.cargo/config.toml`:

```toml
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"
```

Or use other mirrors:
- **TUNA** (China): `https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git`
- **SJTU** (China): `https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index/`

#### E. Check Firewall/Antivirus
- Temporarily disable Windows Defender or antivirus
- Check Windows Firewall isn't blocking cargo
- If behind corporate proxy, configure cargo proxy:

```powershell
$env:HTTP_PROXY = "http://proxy.company.com:8080"
$env:HTTPS_PROXY = "http://proxy.company.com:8080"
```

#### F. Clear Cargo Cache and Retry
```powershell
# Clear cargo registry cache
Remove-Item -Recurse -Force $env:USERPROFILE\.cargo\registry
Remove-Item -Recurse -Force $env:USERPROFILE\.cargo\git

# Retry build
cargo build --release
```

### Issue 2: OpenSSL Build Failures

**Symptoms**:
```
error: failed to run custom build command for `openssl-sys`
```

**Solution**:
Set the environment variable (already in scripts, but ensure it's set):
```powershell
$env:OPENSSL_VENDORED = "1"
cargo build --release
```

Or set it permanently:
```powershell
[System.Environment]::SetEnvironmentVariable('OPENSSL_VENDORED', '1', 'User')
```

### Issue 3: Out of Memory

**Symptoms**:
```
error: linking with `link.exe` failed: exit code: 1120
LINK : fatal error LNK1102: out of memory
```

**Solutions**:

#### A. Reduce Parallel Jobs
```powershell
cargo build --release -j 2
```

#### B. Close Other Programs
- Close browser, IDE, and other heavy applications
- At least 8GB free RAM recommended

#### C. Increase Virtual Memory
1. System Properties → Advanced → Performance Settings
2. Advanced → Virtual Memory → Change
3. Set custom size: Initial = 16GB, Maximum = 32GB

### Issue 4: Long Path Names

**Symptoms**:
```
error: could not create file ... (path too long)
```

**Solution**:
Enable long paths in Windows:
```powershell
# Run as Administrator
New-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem" -Name "LongPathsEnabled" -Value 1 -PropertyType DWORD -Force
```

Or move the project to a shorter path like `C:\vp\`

### Issue 5: Missing Build Tools

**Symptoms**:
```
error: linker `link.exe` not found
```

**Solution**:
Install Visual Studio Build Tools with C++ workload:
1. Download from: https://visualstudio.microsoft.com/downloads/
2. Select "Desktop development with C++"
3. Install and restart

### Issue 6: WASM Target Missing

**Symptoms**:
```
error: the `wasm32-unknown-unknown` target is not installed
```

**Solution**:
```powershell
rustup target add wasm32-unknown-unknown
rustup component add rust-src
```

## Recommended Build Process for Windows

### First Time Build

```powershell
# 1. Set environment
$env:OPENSSL_VENDORED = "1"

# 2. Ensure WASM target is installed
rustup target add wasm32-unknown-unknown
rustup component add rust-src

# 3. Pre-fetch all dependencies (optional but recommended)
cargo fetch

# 4. Build with moderate parallelism
cargo build --release -j 4
```

**Estimated time**: 20-45 minutes depending on hardware

### Subsequent Builds

```powershell
$env:OPENSSL_VENDORED = "1"
cargo build --release
```

**Estimated time**: 2-5 minutes (incremental)

## Hardware Recommendations

**Minimum**:
- CPU: 4 cores
- RAM: 8GB
- Disk: 20GB free space
- Internet: Stable connection

**Recommended**:
- CPU: 8+ cores
- RAM: 16GB+
- Disk: 50GB free space (SSD preferred)
- Internet: 10+ Mbps

## Network Requirements

The build will download approximately:
- **Dependencies**: ~2-3 GB
- **Compilation artifacts**: ~5-10 GB total

Ensure you have:
- Stable internet connection
- No download limits/quotas
- crates.io is accessible (not blocked by firewall)

## Using Build Scripts

We've provided scripts that handle environment setup:

### Development Build
```powershell
.\check-all.bat
```
Runs all verification steps automatically.

### Quick Build
```powershell
# Just build without running
$env:OPENSSL_VENDORED = "1"
cargo build --release
```

### Run Development Node
```powershell
.\run-dev.bat
```
Starts the chain in development mode.

## Troubleshooting Checklist

If build fails, check:
- [ ] Internet connection is stable
- [ ] Antivirus/firewall not blocking cargo
- [ ] At least 10GB free disk space
- [ ] Visual Studio Build Tools installed
- [ ] OPENSSL_VENDORED=1 is set
- [ ] wasm32-unknown-unknown target installed
- [ ] rust-src component installed
- [ ] No VPN/proxy issues
- [ ] Try with `cargo clean` first
- [ ] Check `.cargo/config.toml` has increased timeout

## Still Having Issues?

### Get Verbose Output
```powershell
cargo build --release -vv 2>&1 | Out-File build-log.txt
```
Then check `build-log.txt` for detailed error messages.

### Check Cargo Cache
```powershell
cargo cache --info  # If cargo-cache is installed
```

### Test Network Connection
```powershell
Test-NetConnection static.crates.io -Port 443
```

### Verify Rust Installation
```powershell
rustc --version
cargo --version
rustup show
```

## Alternative: Use WSL2 (Windows Subsystem for Linux)

If Windows builds continue to fail, consider using WSL2:

```powershell
# Install WSL2
wsl --install -d Ubuntu

# Inside WSL2
cd /mnt/c/velocitypay/velocity-chain
# Follow Linux build instructions instead
```

WSL2 often has fewer build issues than native Windows.

## Success Indicators

Build is successful when you see:
```
Compiling velocity-node v1.0.0
Finished `release` profile [optimized] target(s) in X.XXm
```

And the binary exists:
```powershell
Test-Path target\release\velocity-node.exe
# Should return: True
```

## After Successful Build

Run the health check:
```powershell
.\check-node.bat
```

This verifies:
- Binary executes correctly
- Node starts successfully
- RPC endpoints respond
- Chain is producing blocks

---

**Pro Tip**: Once you have a successful build, the `target/` directory contains all compiled artifacts. Subsequent builds are much faster (2-5 minutes) thanks to incremental compilation.
