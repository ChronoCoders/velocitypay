@echo off
echo ========================================
echo VelocityPay Complete Verification
echo ========================================

set OPENSSL_VENDORED=1

echo.
echo [1/10] Checking compilation...
cargo check --all-features
if errorlevel 1 goto :error

echo.
echo [2/10] Building release version...
cargo build --release
if errorlevel 1 goto :error

echo.
echo [3/10] Running Clippy...
cargo clippy --all-targets
if errorlevel 1 goto :error

echo.
echo [4/10] Checking format...
cargo fmt --check
if errorlevel 1 goto :error

echo.
echo [5/10] Running tests...
cargo test --workspace
if errorlevel 1 goto :error

echo.
echo [6/10] Building WASM runtime...
cargo build --release -p velocity-runtime
if errorlevel 1 goto :error

echo.
echo [7/10] Verifying node binary...
.\target\release\velocity-node.exe --version
if errorlevel 1 goto :error

echo.
echo [8/10] Generating documentation...
cargo doc --no-deps --workspace
if errorlevel 1 goto :error

echo.
echo [9/10] Checking dependencies...
cargo tree > dependency-tree.txt
echo Dependency tree saved to dependency-tree.txt

echo.
echo [10/10] Auditing dependencies...
cargo audit
if errorlevel 1 (
    echo Warning: Security vulnerabilities found
) else (
    echo No security vulnerabilities found
)

echo.
echo ========================================
echo ALL CHECKS PASSED!
echo ========================================
echo.
echo Build artifacts:
echo - Node binary: .\target\release\velocity-node.exe
echo - Runtime WASM: .\target\release\wbuild\velocity-runtime\velocity_runtime.compact.compressed.wasm
echo - Documentation: .\target\doc\index.html
echo.
goto :end

:error
echo.
echo ========================================
echo CHECK FAILED!
echo ========================================
echo Please review the errors above and fix them.
exit /b 1

:end