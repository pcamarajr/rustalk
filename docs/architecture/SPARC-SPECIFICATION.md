# SPARC Specification - RUSTALK MVP Architecture

**Phase:** Specification (Complete)
**Date:** 2025-10-03
**Status:** ✅ Approved by Hive Mind Consensus

## S - Specification

### Project Overview
**RUSTALK** is an open-source, white-label VoIP desktop application built with Rust + Tauri + SvelteKit. This document serves as the comprehensive SPARC Specification for the MVP architecture.

### Requirements

#### Functional Requirements
1. **SIP Registration (FR-001)**
   - User can register SIP account with credentials (username, password, server)
   - System handles 401 authentication challenge
   - Credentials stored securely in platform keychain
   - Registration status visible in UI

2. **Outbound Calls (FR-002)**
   - User can dial phone number via UI
   - System sends SIP INVITE to target
   - System negotiates SDP for audio
   - System establishes RTP session
   - Two-way audio communication

3. **Inbound Calls (FR-003)**
   - System receives SIP INVITE from server
   - UI displays incoming call notification
   - User can answer or reject call
   - System sends SIP 200 OK on answer
   - Two-way audio communication

4. **Call Controls (FR-004)**
   - User can hangup active call
   - System sends SIP BYE message
   - User can mute/unmute microphone
   - Mute state reflected in UI

5. **Audio Device Selection (FR-005)**
   - System enumerates available audio devices (mic + speakers)
   - User can select devices via settings UI
   - Device selection persists across sessions
   - System supports device switching during call

6. **Secure Credential Storage (FR-006)**
   - Credentials stored in macOS Keychain (Phase 1-3)
   - Credentials stored in Windows Credential Manager (Phase 4)
   - Never store credentials in plaintext
   - Auto-populate credentials on login

#### Non-Functional Requirements
1. **Performance (NFR-001)**
   - Audio latency: <150ms end-to-end
   - SIP registration: <3 seconds
   - Call setup time: <2 seconds

2. **Quality (NFR-002)**
   - Code coverage: Rust ≥85%, SvelteKit ≥80%
   - Zero critical bugs at release
   - All E2E tests passing

3. **Security (NFR-003)**
   - TLS/SIPS for SIP signaling
   - No credentials in logs or error messages
   - Input validation on all Tauri commands
   - Regular security audits (cargo audit)

4. **Platform Support (NFR-004)**
   - macOS 11+ (primary target)
   - Windows 10+ x64 (secondary target)
   - Universal binary for macOS (Intel + Apple Silicon)

5. **Maintainability (NFR-005)**
   - Clean Architecture with clear layer boundaries
   - Pure Rust (no FFI where possible)
   - Comprehensive documentation
   - SPARC methodology for all features

### Success Criteria
- [ ] All 6 MVP features implemented and tested
- [ ] macOS build distributable (DMG)
- [ ] Windows build distributable (MSI)
- [ ] Code coverage targets met
- [ ] Documentation complete
- [ ] Zero critical security vulnerabilities

---

## P - Pseudocode

### High-Level System Flow

```
APPLICATION STARTUP:
1. Initialize Tauri application
2. Load settings from storage
3. Restore SIP registration if credentials exist
4. Initialize audio system
5. Display main UI

SIP REGISTRATION FLOW:
1. User enters credentials (username, password, server)
2. Validate input (not empty, server format)
3. Store credentials in keychain
4. Send SIP REGISTER to server
5. Receive 401 Unauthorized with challenge
6. Calculate authentication response
7. Send authenticated REGISTER
8. Receive 200 OK
9. Update UI to "Registered"
10. Emit registration_state_changed event

OUTBOUND CALL FLOW:
1. User enters phone number in dialer
2. Validate phone number format
3. Create Call entity (state: Idle)
4. Send SIP INVITE with SDP offer
5. Receive 180 Ringing
6. Update Call state to Ringing, update UI
7. Receive 200 OK with SDP answer
8. Parse SDP, extract RTP endpoint
9. Create RTP session
10. Start audio streams (mic → RTP → network)
11. Update Call state to Active
12. Emit call_state_changed event
13. Display active call UI

INBOUND CALL FLOW:
1. Receive SIP INVITE from server
2. Parse INVITE, extract caller ID and SDP
3. Create Call entity (state: Ringing, direction: Inbound)
4. Emit incoming_call event to UI
5. Display incoming call notification
6. User clicks "Answer" button
7. Send SIP 200 OK with SDP answer
8. Create RTP session
9. Start audio streams
10. Update Call state to Active
11. Emit call_state_changed event
12. Display active call UI

CALL HANGUP FLOW:
1. User clicks "Hangup" button
2. Send SIP BYE message
3. Stop RTP session
4. Stop audio streams
5. Update Call state to Ended
6. Emit call_state_changed event
7. Return to idle UI

MUTE FLOW:
1. User clicks "Mute" button
2. Stop microphone audio stream (keep speaker)
3. Update mute state in UI
4. User clicks "Unmute" button
5. Resume microphone audio stream
6. Update mute state in UI
```

### Key Algorithms

#### SIP Authentication (Digest)
```
FUNCTION calculate_auth_response(username, password, realm, nonce, method, uri):
    ha1 = MD5(username + ":" + realm + ":" + password)
    ha2 = MD5(method + ":" + uri)
    response = MD5(ha1 + ":" + nonce + ":" + ha2)
    RETURN response
```

#### RTP Session Setup
```
FUNCTION setup_rtp_session(local_sdp, remote_sdp):
    local_port = allocate_rtp_port()  // Even number
    rtcp_port = local_port + 1        // Odd number

    remote_ip = parse_ip_from_sdp(remote_sdp)
    remote_port = parse_port_from_sdp(remote_sdp)

    rtp_socket = bind_udp_socket(local_port)
    rtcp_socket = bind_udp_socket(rtcp_port)

    jitter_buffer = create_jitter_buffer(size: 20ms)

    RETURN RtpSession(rtp_socket, rtcp_socket, remote_ip, remote_port, jitter_buffer)
```

#### Audio Stream
```
FUNCTION audio_output_loop(rtp_session, speaker_device):
    WHILE call_active:
        packet = rtp_session.receive()
        audio_data = decode_g711(packet.payload)
        jitter_buffer.push(audio_data)

        IF jitter_buffer.ready():
            samples = jitter_buffer.pop()
            speaker_device.play(samples)

FUNCTION audio_input_loop(mic_device, rtp_session):
    WHILE call_active AND not_muted:
        samples = mic_device.record()
        payload = encode_g711(samples)
        packet = create_rtp_packet(payload)
        rtp_session.send(packet)
```

---

## A - Architecture

### Layer Architecture

#### 1. Presentation Layer (SvelteKit)
**Path:** `/src`

**Components:**
- `Dialer.svelte` - Phone number input and call button
- `ActiveCall.svelte` - Active call status and duration
- `CallControls.svelte` - Hangup, mute buttons
- `IncomingCall.svelte` - Incoming call notification
- `AudioDeviceSelector.svelte` - Device dropdown
- `LoginForm.svelte` - SIP credentials input

**Stores:**
- `callStore.ts` - Call state (active call, status, duration)
- `authStore.ts` - Registration state
- `audioStore.ts` - Audio devices and selection
- `settingsStore.ts` - User preferences

**API Layer:**
- `callApi.ts` - Wrapper for Tauri call commands
- `authApi.ts` - Wrapper for auth commands
- `audioApi.ts` - Wrapper for audio commands

#### 2. IPC Boundary Layer
**Path:** `/src-tauri/src/commands`

**Modules:**
- `auth.rs` - register_account, unregister_account, get_registration_status
- `call.rs` - initiate_call, answer_call, hangup_call, mute_call
- `audio.rs` - list_audio_devices, set_audio_device, get_audio_levels
- `credentials.rs` - save_credentials, load_credentials
- `validation.rs` - Input validation helpers

**Events Emitted:**
- `call_state_changed` - { call_id, state, remote_number }
- `incoming_call` - { call_id, caller_id }
- `registration_state_changed` - { state }
- `audio_device_changed` - { device_id }

#### 3. Application Layer
**Path:** `/src-tauri/src/services`

**Services:**
- `CallService` - Orchestrates call lifecycle
- `AuthService` - Manages SIP registration
- `AudioService` - Audio device management
- `SettingsService` - User settings persistence

**State Management:**
- `Arc<RwLock<CallState>>` - Thread-safe call state
- `Arc<RwLock<RegistrationState>>` - Thread-safe auth state

#### 4. Domain Layer
**Path:** `/src-tauri/src/domain`

**Entities:**
- `Call` - id, direction, remote_number, state, timestamps
- `Credentials` - username, password, server (value object)
- `AudioDevice` - id, name, device_type (Input/Output)
- `Contact` - name, number (future)

**Traits:**
- `SipClient` - send_invite, send_bye, register
- `AudioEngine` - enumerate_devices, start_stream, stop_stream
- `RtpManager` - create_session, send_packet, receive_packet
- `CredentialStore` - save, load, delete

#### 5. Infrastructure Layer
**Path:** `/src-tauri/src/infrastructure`

**SIP Module (`sip/`):**
- `client.rs` - SipClient trait implementation
- `transport.rs` - UDP/TCP/TLS transport with Tokio
- `parser.rs` - rsip message parsing
- `registration.rs` - REGISTER flow
- `invite.rs` - INVITE/BYE handling
- `sdp.rs` - SDP offer/answer generation

**RTP Module (`rtp/`):**
- `session.rs` - RtpManager implementation
- `codec.rs` - G.711 encoder/decoder
- `jitter_buffer.rs` - Audio jitter buffer

**Audio Module (`audio/`):**
- `engine.rs` - AudioEngine trait implementation
- `macos.rs` - CoreAudio backend (via cpal)
- `windows.rs` - WASAPI backend (via cpal)
- `device_manager.rs` - Device enumeration

**Storage Module (`storage/`):**
- `credentials.rs` - CredentialStore implementation
- `keychain.rs` - macOS Keychain (via keyring)
- `credential_mgr.rs` - Windows Credential Manager (via keyring)

### Data Flow Diagram

```
User clicks "Call" button
    ↓
callStore.initiate(number)
    ↓ invoke("initiate_call", { number })
commands/call.rs: initiate_call(number)
    ↓ validate_phone_number
    ↓ call_service.initiate_outbound_call
services/call_service.rs
    ↓ Create Call entity (Domain)
    ↓ sip_client.send_invite
infrastructure/sip/client.rs
    ↓ Build INVITE message (rsip)
    ↓ Send via transport (Tokio)
    → SIP Server
    ← 180 Ringing
    ↓ Parse response
    ↓ Update Call state to Ringing
    ↓ emit("call_state_changed", ...)
callStore receives event
    ↓
UI updates to show "Ringing..."
    ← 200 OK + SDP
    ↓ Parse SDP answer
    ↓ rtp_manager.create_session
infrastructure/rtp/session.rs
    ↓ Set up RTP socket
    ↓ audio_engine.start_stream
infrastructure/audio/macos.rs (or windows.rs)
    ↓ cpal stream start
    ↓ Audio callback loop begins
    ↓ Update Call state to Active
    ↓ emit("call_state_changed", ...)
callStore receives event
    ↓
UI updates to show "Active Call"
```

### Technology Mapping

| Layer | Technology | Files |
|-------|-----------|-------|
| Presentation | SvelteKit + TypeScript | `/src/**/*.svelte`, `/src/**/*.ts` |
| IPC Boundary | Tauri Commands | `/src-tauri/src/commands/**/*.rs` |
| Application | Rust + Tokio | `/src-tauri/src/services/**/*.rs` |
| Domain | Pure Rust | `/src-tauri/src/domain/**/*.rs` |
| Infrastructure | rsip, cpal, webrtc-rs, keyring | `/src-tauri/src/infrastructure/**/*.rs` |

---

## R - Refinement

### Optimization Strategies

#### 1. Audio Latency Reduction
- Use small audio buffer sizes (128-256 samples)
- Direct CoreAudio/WASAPI APIs via cpal
- Jitter buffer tuning (20-50ms)
- Packet loss concealment for RTP

#### 2. SIP Message Parsing
- Use rsip's lazy parsing (zero-copy where possible)
- Cache parsed headers for repeated access
- Reuse buffer allocations

#### 3. Async Performance
- Use Tokio's multi-threaded runtime
- Channel-based communication (not mutex for hot paths)
- Separate Tokio tasks for SIP, RTP, audio
- Avoid blocking in async contexts

#### 4. Memory Management
- Pool audio buffers to avoid allocations
- Reuse RTP packet buffers
- Arc for shared state (not Clone)
- Weak references to avoid cycles

### Error Handling Strategy

#### Error Types
```rust
#[derive(thiserror::Error)]
pub enum RustalkError {
    #[error("SIP error: {0}")]
    Sip(#[from] SipError),

    #[error("Audio error: {0}")]
    Audio(#[from] AudioError),

    #[error("RTP error: {0}")]
    Rtp(#[from] RtpError),

    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
}
```

#### Error Propagation
- Infrastructure: Return `Result<T, RustalkError>`
- Application: Log errors, emit events to UI
- IPC Boundary: Translate to Tauri-compatible errors
- Presentation: Display user-friendly messages

### Security Hardening

#### Input Validation
- Phone number: regex `^\+?[0-9]{1,15}$`
- SIP server: validate hostname/IP format
- Username/password: max length limits
- SDP: sanitize before parsing

#### TLS Configuration
- TLS 1.2+ only (no TLS 1.0/1.1)
- Certificate pinning for known servers (optional)
- Validate certificate chains

#### Credential Protection
- Never log credentials
- Scrub error messages
- Use SecureString/Zeroize for passwords in memory

---

## C - Completion

### Testing Strategy

#### Unit Tests (85%+ Rust, 80%+ SvelteKit)
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_initiate_outbound_call() {
        let mut mock_sip = MockSipClient::new();
        mock_sip
            .expect_send_invite()
            .times(1)
            .returning(|_| Ok(()));

        let service = CallService::new(Arc::new(mock_sip), ...);
        let result = service.initiate_outbound_call("555-1234").await;

        assert!(result.is_ok());
    }
}
```

#### Integration Tests
```rust
#[tokio::test]
async fn test_full_registration_flow() {
    // Set up test SIP server
    let server = MockSipServer::start().await;

    // Create service
    let auth_service = create_auth_service(server.url());

    // Test registration
    let result = auth_service.register(credentials).await;
    assert!(result.is_ok());

    // Verify server received REGISTER
    assert!(server.received_register());
}
```

#### E2E Tests (Playwright)
```typescript
test('user can register SIP account', async ({ page }) => {
  await page.goto('/login');
  await page.fill('[data-testid="username"]', 'testuser');
  await page.fill('[data-testid="password"]', 'testpass');
  await page.fill('[data-testid="server"]', 'sip.example.com');
  await page.click('[data-testid="register-btn"]');

  await expect(page.locator('[data-testid="status"]'))
    .toHaveText('Registered');
});
```

### Definition of Done

Each feature is complete when:
- [ ] Unit tests written and passing (85%+ coverage)
- [ ] Integration tests passing
- [ ] E2E test passing on macOS
- [ ] E2E test passing on Windows (Phase 4)
- [ ] Code reviewed by reviewer agent
- [ ] Documentation updated
- [ ] No critical bugs
- [ ] Performance targets met

### Acceptance Criteria (MVP Complete)

- [ ] All 6 MVP features implemented
- [ ] Can register SIP account on macOS
- [ ] Can register SIP account on Windows
- [ ] Can make outbound call with audio
- [ ] Can receive inbound call with audio
- [ ] Can hangup call
- [ ] Can mute/unmute during call
- [ ] Can select audio devices
- [ ] Credentials persist across restarts
- [ ] Audio latency <150ms
- [ ] Code coverage ≥85% (Rust), ≥80% (SvelteKit)
- [ ] All E2E tests passing
- [ ] macOS DMG build successful
- [ ] Windows MSI build successful
- [ ] Documentation complete
- [ ] Zero critical security vulnerabilities

### Deployment

#### macOS Build
```bash
# Universal binary (Intel + Apple Silicon)
cargo tauri build --target universal-apple-darwin

# Sign with Developer ID
codesign --deep --force --verify --verbose \
  --sign "Developer ID Application: ..." \
  target/universal-apple-darwin/release/bundle/macos/RusTalk.app

# Notarize
xcrun notarytool submit target/...dmg --wait

# Staple
xcrun stapler staple target/...dmg
```

#### Windows Build
```bash
# x64 build
cargo tauri build --target x86_64-pc-windows-msvc

# Output: MSI installer (NSIS)
```

### Release Checklist

- [ ] Version number updated (Cargo.toml, package.json, tauri.conf.json)
- [ ] CHANGELOG.md updated
- [ ] All tests passing on CI
- [ ] macOS build signed and notarized
- [ ] Windows build created
- [ ] Documentation published
- [ ] GitHub release created
- [ ] Release notes written

---

## SPARC Metadata

**Specification Phase:** ✅ Complete
**Pseudocode Phase:** ✅ Complete
**Architecture Phase:** ✅ Complete
**Refinement Phase:** ✅ Complete
**Completion Phase:** 🔄 Ready for implementation

**Next Action:** Begin Phase 1 implementation (Core Infrastructure)

---

## Appendix: Swarm Memory Keys

All SPARC data stored in swarm memory under namespace `rustalk-mvp`:

- `hive/objective` - Mission statement
- `hive/tech_stack` - Technology stack summary
- `hive/mvp_features` - 6 MVP features
- `research/sip_libraries` - SIP library evaluation
- `research/audio_libraries` - Audio library evaluation
- `research/rtp_libraries` - RTP library evaluation
- `architecture/layers` - 5-layer architecture
- `architecture/backend_modules` - Rust module structure
- `architecture/frontend_structure` - SvelteKit organization
- `testing/rust_strategy` - Rust testing plan
- `testing/frontend_strategy` - Frontend testing plan
- `testing/e2e_strategy` - E2E testing plan
- `testing/summary` - Testing overview
- `implementation/feature_breakdown` - 39 detailed tasks
- `implementation/dependencies` - Dependency graph
- `implementation/phases` - 4-phase timeline
- `implementation/risks` - Risk assessment
- `consensus/technology_stack` - Approved tech stack
- `consensus/architecture_approved` - Approved architecture
- `validation/mvp_requirements` - MVP validation
- `artifacts/documentation_complete` - Documentation status

**Total Memory Entries:** 22 keys

---

**Approved by:** Queen Coordinator (Strategic)
**Consensus Algorithm:** Majority
**Worker Agents:** Researcher, Analyst, Tester, Coder
**Swarm ID:** swarm-1759490413384-mo4ol70oc
