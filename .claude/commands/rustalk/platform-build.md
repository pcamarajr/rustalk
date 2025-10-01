---
name: platform-build
description: Build RUSTALK for macOS or Windows with platform-specific configuration
usage: npx claude-flow@alpha command rustalk/platform-build --platform "<macos|windows>"
params:
  - name: platform
    required: true
    description: Target platform (macos or windows)
  - name: sign
    required: false
    default: false
    description: Enable code signing (requires credentials)
---

# Platform-Specific Build Command

Builds RUSTALK desktop application for macOS or Windows with proper configuration, signing, and packaging.

## Workflow

### macOS Build

```bash
# Prerequisites check
- Xcode Command Line Tools installed
- Rust toolchains: x86_64-apple-darwin, aarch64-apple-darwin
- Apple Developer ID (for signing)

# Build steps
1. Install dependencies: pnpm install
2. Build frontend: pnpm build
3. Build Tauri app: cargo tauri build --target universal-apple-darwin
4. Sign app (if --sign=true):
   - Code sign with Developer ID
   - Notarize with Apple
5. Create DMG installer
6. Output: target/universal-apple-darwin/release/bundle/macos/RUSTALK.app
```

### Windows Build

```bash
# Prerequisites check
- Visual Studio Build Tools 2022
- Rust toolchain: x86_64-pc-windows-msvc
- WiX Toolset (for MSI, optional)

# Build steps
1. Install dependencies: pnpm install
2. Build frontend: pnpm build
3. Build Tauri app: cargo tauri build --target x86_64-pc-windows-msvc
4. Sign installer (if --sign=true and certificate available)
5. Create NSIS installer
6. Output: target/release/bundle/nsis/rustalk_<version>_x64-setup.exe
```

## Agent Coordination

```javascript
// Platform-specific build workflow
Task("Platform Specialist", "Configure build for <platform>", "cicd-engineer")
Task("Build Validator", "Verify build artifacts and test", "tester")

// Store build configuration in memory
mcp__claude-flow__memory_usage {
  action: "store",
  key: "rustalk/build/<platform>",
  namespace: "rustalk",
  value: JSON.stringify({
    platform: "macos",
    target: "universal-apple-darwin",
    signed: true,
    output: "target/.../RUSTALK.app"
  })
}
```

## macOS Detailed Steps

### 1. Environment Setup

```bash
# Check prerequisites
xcode-select --install
rustup target add x86_64-apple-darwin aarch64-apple-darwin

# Verify Node.js and pnpm
node --version  # Should be v18+
pnpm --version
```

### 2. Build Configuration

```toml
# tauri.conf.json - macOS specific
{
  "tauri": {
    "bundle": {
      "identifier": "com.rustalk.app",
      "macOS": {
        "minimumSystemVersion": "11.0",
        "frameworks": [],
        "entitlements": "entitlements.plist"
      },
      "targets": ["dmg", "app"]
    }
  }
}
```

### 3. Code Signing (Optional)

```bash
# Sign the app
codesign --deep --force --verify --verbose \
  --sign "Developer ID Application: <TEAM>" \
  --options runtime \
  --entitlements entitlements.plist \
  target/universal-apple-darwin/release/bundle/macos/RUSTALK.app

# Verify signature
codesign --verify --deep --strict target/.../RUSTALK.app
spctl --assess --type execute target/.../RUSTALK.app
```

### 4. Notarization (Optional)

```bash
# Create DMG
hdiutil create -volname "RUSTALK" -srcfolder target/.../RUSTALK.app -ov -format UDZO rustalk.dmg

# Submit for notarization
xcrun notarytool submit rustalk.dmg \
  --apple-id <APPLE_ID> \
  --password <APP_SPECIFIC_PASSWORD> \
  --team-id <TEAM_ID> \
  --wait

# Staple notarization ticket
xcrun stapler staple rustalk.dmg
```

## Windows Detailed Steps

### 1. Environment Setup

```powershell
# Check prerequisites
rustup target add x86_64-pc-windows-msvc

# Install Visual Studio Build Tools
winget install Microsoft.VisualStudio.2022.BuildTools

# Verify Node.js and pnpm
node --version
pnpm --version
```

### 2. Build Configuration

```json
// tauri.conf.json - Windows specific
{
  "tauri": {
    "bundle": {
      "identifier": "com.rustalk.app",
      "windows": {
        "certificateThumbprint": null,
        "digestAlgorithm": "sha256",
        "timestampUrl": "",
        "wix": {
          "language": "en-US"
        }
      },
      "targets": ["nsis", "msi"]
    }
  }
}
```

### 3. Code Signing (Optional)

```powershell
# Sign the installer (if certificate available)
signtool sign /tr http://timestamp.digicert.com /td sha256 /fd sha256 \
  /a target/release/bundle/nsis/rustalk_setup.exe
```

## Testing Builds

### Automated Tests

```bash
# After build completes, run validation tests
Task("Build Tester", "Validate <platform> build", "tester")

# Tests include:
1. App launches successfully
2. SIP registration works
3. Audio devices enumerate
4. No crashes on basic operations
```

### Manual Testing Checklist

**macOS**:
- [ ] App opens without security warnings
- [ ] Microphone permission prompt appears
- [ ] SIP registration succeeds
- [ ] Audio playback works
- [ ] App Icon displays correctly

**Windows**:
- [ ] Installer runs without errors
- [ ] App installs to correct location
- [ ] Firewall prompts for network access
- [ ] SIP registration succeeds
- [ ] Audio works on both input/output

## CI/CD Integration

```yaml
# .github/workflows/build.yml
name: Platform Build

on:
  push:
    tags:
      - 'v*'

jobs:
  build-macos:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v3
      - name: Build for macOS
        run: npx claude-flow@alpha command rustalk/platform-build --platform macos --sign true
        env:
          APPLE_ID: ${{ secrets.APPLE_ID }}
          APPLE_PASSWORD: ${{ secrets.APPLE_PASSWORD }}
          APPLE_TEAM_ID: ${{ secrets.APPLE_TEAM_ID }}

  build-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3
      - name: Build for Windows
        run: npx claude-flow@alpha command rustalk/platform-build --platform windows
```

## Example Usage

```bash
# macOS universal binary (unsigned)
npx claude-flow@alpha command rustalk/platform-build --platform macos

# macOS with code signing
npx claude-flow@alpha command rustalk/platform-build --platform macos --sign true

# Windows installer
npx claude-flow@alpha command rustalk/platform-build --platform windows

# Windows with code signing (requires certificate)
npx claude-flow@alpha command rustalk/platform-build --platform windows --sign true
```

## Troubleshooting

**macOS**:
- "Developer cannot be verified": Enable code signing
- "Microphone permission denied": Add entitlements.plist
- Universal binary too large: Check for duplicate dependencies

**Windows**:
- NSIS errors: Update WiX Toolset
- DLL not found: Check Visual Studio runtime dependencies
- Installer not signed: Add code signing certificate

## Output Artifacts

**macOS**:
- `RUSTALK.app` - Application bundle
- `rustalk.dmg` - Disk image installer
- `rustalk-universal.tar.gz` - Archive for distribution

**Windows**:
- `rustalk_<version>_x64-setup.exe` - NSIS installer
- `rustalk_<version>_x64.msi` - MSI installer (if WiX configured)
