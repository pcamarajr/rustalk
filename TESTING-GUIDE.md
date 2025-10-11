# Testing Guide: Phase 1 Security & Storage

## Quick Start

### Prerequisites

```bash
# Ensure you have Rust installed
rustup --version

# Navigate to the Rust backend
cd src-tauri
```

### Run All Tests

```bash
# Run all security module tests
cargo test --lib security::tests

# Run with output visible
cargo test --lib security::tests -- --nocapture

# Run specific test
cargo test --lib security::tests::test_store_credentials_successfully
```

## Test Categories

### 1. Credential Storage Tests (7 tests)

```bash
cargo test --lib credential_storage_tests::test_store_credentials_successfully
cargo test --lib credential_storage_tests::test_store_credentials_duplicate_account
cargo test --lib credential_storage_tests::test_store_credentials_validates_username
cargo test --lib credential_storage_tests::test_store_credentials_validates_password
cargo test --lib credential_storage_tests::test_store_credentials_validates_server_host
cargo test --lib credential_storage_tests::test_store_credentials_validates_server_port
cargo test --lib credential_storage_tests::test_store_multiple_accounts
```

### 2. Credential Retrieval Tests (5 tests)

```bash
cargo test --lib credential_storage_tests::test_get_credentials_successfully
cargo test --lib credential_storage_tests::test_get_credentials_not_found
cargo test --lib credential_storage_tests::test_get_credentials_with_special_characters
cargo test --lib credential_storage_tests::test_get_credentials_performance
cargo test --lib credential_storage_tests::test_list_accounts_excludes_passwords
```

### 3. Credential Deletion Tests (3 tests)

```bash
cargo test --lib credential_storage_tests::test_delete_account_successfully
cargo test --lib credential_storage_tests::test_delete_nonexistent_account
cargo test --lib credential_storage_tests::test_delete_account_removes_from_keychain
```

### 4. Error Handling Tests (5 tests)

```bash
cargo test --lib credential_storage_tests::test_keychain_unavailable_error
cargo test --lib credential_storage_tests::test_keychain_permission_denied_error
cargo test --lib credential_storage_tests::test_sql_injection_prevention
cargo test --lib credential_storage_tests::test_null_byte_prevention
cargo test --lib credential_storage_tests::test_control_characters_rejection
```

### 5. Memory Safety Tests (2 tests)

```bash
cargo test --lib credential_storage_tests::test_credentials_zeroized_on_drop
cargo test --lib credential_storage_tests::test_credentials_not_logged_in_debug
```

### 6. Platform-Specific Tests

**macOS:**
```bash
cargo test --lib credential_storage_tests::test_macos_keychain_integration
```

**Windows:**
```bash
cargo test --lib credential_storage_tests::test_windows_credential_manager_integration
```

**Unsupported Platforms:**
```bash
cargo test --lib credential_storage_tests::test_unsupported_platform_error
```

### 7. Concurrent Access Tests

```bash
cargo test --lib credential_storage_tests::test_concurrent_credential_access
```

## Test Coverage

### Generate Coverage Report

```bash
# Install cargo-tarpaulin (first time only)
cargo install cargo-tarpaulin

# Generate HTML coverage report
cargo tarpaulin --lib --exclude-files='src/main.rs' --out Html

# Open report in browser
open tarpaulin-report.html  # macOS
xdg-open tarpaulin-report.html  # Linux
```

### Coverage Targets

- **Rust Backend:** 85%+ coverage
- **Current Status:** Run `cargo tarpaulin` to check

## Performance Benchmarks

### Credential Retrieval Performance

```bash
# Run performance test
cargo test --lib credential_storage_tests::test_get_credentials_performance -- --nocapture
```

**Expected:** <50ms p95 latency

## Manual Testing

### 1. Build the Project

```bash
# Clean build
cargo clean
cargo build

# Release build
cargo build --release
```

### 2. Run Development Server

```bash
# Start Tauri development server
cargo tauri dev
```

**Note:** UI components not yet implemented. Manual testing requires implementing Tauri commands first.

## Continuous Integration

### GitHub Actions

Tests will run automatically on:
- Pull request creation
- Push to `main` branch
- Manual workflow dispatch

```yaml
# .github/workflows/rust-tests.yml
name: Rust Tests
on: [push, pull_request]
jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [macos-latest, windows-latest, ubuntu-latest]
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cd src-tauri && cargo test --lib security::tests
```

## Troubleshooting

### Test Failures

#### Keychain Permission Errors (macOS)

```bash
# Symptom: "Permission denied" errors
# Solution: Grant keychain access in System Preferences
# Security & Privacy > Privacy > Keychain Access > Terminal (enable)
```

#### Credential Manager Errors (Windows)

```bash
# Symptom: "Access denied" errors
# Solution: Run as administrator or check Windows Credential Manager permissions
```

#### Build Errors

```bash
# Clean and rebuild
cargo clean
cargo build

# Update dependencies
cargo update
```

### Debug Mode

```bash
# Run tests with debug output
RUST_LOG=debug cargo test --lib security::tests -- --nocapture
```

## Test Results Interpretation

### Success

```
test credential_storage_tests::test_store_credentials_successfully ... ok
test credential_storage_tests::test_get_credentials_successfully ... ok
test credential_storage_tests::test_delete_account_successfully ... ok

test result: ok. 40 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Failure

```
test credential_storage_tests::test_store_credentials_successfully ... FAILED

failures:

---- credential_storage_tests::test_store_credentials_successfully stdout ----
thread 'credential_storage_tests::test_store_credentials_successfully' panicked at 'assertion failed'
```

**Action:** Check error message and review implementation.

## Next Steps

After all tests pass:

1. **Code Review** - Request review from team
2. **Tauri Commands** - Implement IPC layer
3. **Frontend Components** - Build SvelteKit UI
4. **E2E Tests** - Add Playwright tests
5. **Security Audit** - Third-party review

## Resources

- [Rust Testing Documentation](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Cargo Test Documentation](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
- [Tarpaulin Documentation](https://github.com/xd009642/tarpaulin)
- [Keyring Crate Tests](https://github.com/hwchen/keyring-rs/tree/main/tests)

---

**Questions?** Check the [Security Specification](./docs/features/security-storage.md) or [Architecture Design](./docs/architecture/security-storage-design.md).
