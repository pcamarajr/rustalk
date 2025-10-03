# Implementation Roadmap - RUSTALK MVP

**Total Duration:** 9.4 weeks (374 hours)
**Approach:** 4-phase incremental delivery
**Methodology:** SPARC TDD (Test-Driven Development)
**Platform Focus:** macOS first, then Windows

## Overview

```
Phase 1: Core Infrastructure      ████████░░░░░░░░░░░░  2.4 weeks
Phase 2: Registration + Audio     ████░░░░░░░░░░░░░░░░  1.7 weeks
Phase 3: Call Flows               ████████░░░░░░░░░░░░  3.3 weeks
Phase 4: Polish + Windows         █████░░░░░░░░░░░░░░░  2.1 weeks
                                  ═══════════════════════
                                  Total: 9.4 weeks
```

## Phase 1: Core Infrastructure (94 hours, ~2.4 weeks)

**Goal:** Establish foundation with platform abstractions, security, and basic SIP transport

### Tasks

#### Security & Storage (28 hours)
- **SEC-6.1** (8h): Design `CredentialStore` trait abstraction
  - Files: `/src-tauri/src/domain/traits/credential_store.rs`
  - Tests: Unit tests for trait contract

- **SEC-6.2** (12h): Implement macOS Keychain integration
  - Files: `/src-tauri/src/infrastructure/storage/keychain.rs`
  - Tests: Integration test - store/retrieve credentials
  - Dependencies: `keyring` crate, `security-framework`

- **SEC-6.5** (8h): Input validation for all Tauri commands
  - Files: `/src-tauri/src/commands/validation.rs`
  - Tests: Unit tests for validation rules

#### Audio System (36 hours)
- **AUD-5.1** (16h): Design `AudioEngine` trait + platform abstraction
  - Files: `/src-tauri/src/domain/traits/audio_engine.rs`
  - Files: `/src-tauri/src/infrastructure/audio/mod.rs`
  - Tests: Mock audio engine for testing

- **AUD-5.2** (20h): macOS CoreAudio integration via `cpal`
  - Files: `/src-tauri/src/infrastructure/audio/macos.rs`
  - Files: `/src-tauri/src/infrastructure/audio/device_manager.rs`
  - Tests: Integration test - enumerate devices, start stream
  - Dependencies: `cpal`

#### SIP Transport (30 hours)
- **SIP-1.1** (16h): Integrate `rsip` library + basic parsing
  - Files: `/src-tauri/src/infrastructure/sip/parser.rs`
  - Files: `/src-tauri/src/infrastructure/sip/message_builder.rs`
  - Tests: Unit tests for parsing REGISTER, INVITE, BYE
  - Dependencies: `rsip`

- **SIP-1.2** (20h): Async SIP transport layer (UDP/TCP/TLS) with Tokio
  - Files: `/src-tauri/src/infrastructure/sip/transport.rs`
  - Files: `/src-tauri/src/infrastructure/sip/client.rs`
  - Tests: Integration test - connect to test SIP server
  - Dependencies: `tokio`, `rustls`

- **SEC-6.6** (10h): TLS certificate validation for SIPS
  - Files: `/src-tauri/src/infrastructure/sip/tls.rs`
  - Tests: Unit tests for cert validation

### Deliverables
- ✅ Secure credential storage working on macOS (Keychain)
- ✅ Audio device enumeration working on macOS (CoreAudio)
- ✅ SIP transport layer with SIPS (TLS) support
- ✅ All tests passing with 85%+ coverage

### Milestone
**Demo:** Show credentials stored in Keychain + list audio devices + TLS handshake to SIP server

### Completion Criteria
- [ ] `cargo test` passes with 85%+ coverage
- [ ] Manual verification: Credentials persist in macOS Keychain
- [ ] TLS connection succeeds to test SIP server
- [ ] Audio devices appear in enumeration list

---

## Phase 2: SIP Registration + Audio Selection (66 hours, ~1.7 weeks)

**Goal:** Complete SIP registration flow and audio device management UI

### Tasks

#### SIP Registration (36 hours)
- **SIP-1.3** (12h): REGISTER message handling (with 401 challenge)
  - Files: `/src-tauri/src/infrastructure/sip/registration.rs`
  - Tests: Unit tests for 401 challenge flow

- **SIP-1.4** (10h): Registration state machine
  - Files: `/src-tauri/src/services/auth_service.rs`
  - Files: `/src-tauri/src/domain/entities/registration.rs`
  - Tests: Unit tests for state transitions

- **SIP-1.5** (6h): Tauri `register_account` command
  - Files: `/src-tauri/src/commands/auth.rs`
  - Tests: Integration test - full registration flow

- **SIP-1.6** (8h): Frontend registration UI (login page)
  - Files: `/src/routes/login/+page.svelte`
  - Files: `/src/lib/stores/authStore.ts`
  - Tests: Component tests for login form

#### Audio Device Selection (30 hours)
- **AUD-5.3** (10h): Audio device enumeration API
  - Files: `/src-tauri/src/services/audio_service.rs`
  - Tests: Unit tests with mock audio engine

- **AUD-5.4** (12h): Device selection and switching logic
  - Files: `/src-tauri/src/services/audio_service.rs`
  - Tests: Integration test - switch devices

- **AUD-5.5** (6h): Tauri audio commands (`list_devices`, `set_device`)
  - Files: `/src-tauri/src/commands/audio.rs`
  - Tests: Integration test - IPC layer

- **AUD-5.6** (8h): Frontend audio settings UI
  - Files: `/src/routes/settings/+page.svelte`
  - Files: `/src/lib/components/AudioDeviceSelector.svelte`
  - Tests: Component tests

### Deliverables
- ✅ Working SIP registration with real SIP server
- ✅ Audio device selection in settings UI
- ✅ Persistent credential storage
- ✅ E2E test for registration flow

### Milestone
**Demo:** Register SIP account via UI + select audio devices in settings

### Completion Criteria
- [ ] Successful registration with test SIP server
- [ ] Audio devices appear in UI dropdowns
- [ ] Credentials persist across app restarts
- [ ] E2E test passes: Login → Register → Settings → Select audio

---

## Phase 3: Call Flows - Outbound + Inbound (130 hours, ~3.3 weeks)

**Goal:** Implement full call lifecycle with bidirectional RTP audio

### Tasks

#### Outbound Calls (74 hours)
- **OUT-2.1** (10h): INVITE message construction
  - Files: `/src-tauri/src/infrastructure/sip/invite.rs`
  - Tests: Unit tests for INVITE generation

- **OUT-2.2** (16h): SDP offer/answer negotiation
  - Files: `/src-tauri/src/infrastructure/sip/sdp.rs`
  - Tests: Unit tests for SDP parsing and generation
  - Dependencies: `webrtc-sdp`

- **OUT-2.3** (12h): Call state machine for outbound calls
  - Files: `/src-tauri/src/domain/entities/call.rs`
  - Files: `/src-tauri/src/services/call_service.rs`
  - Tests: Unit tests for state transitions (Idle → Ringing → Active)

- **OUT-2.4** (20h): RTP session setup and audio streaming
  - Files: `/src-tauri/src/infrastructure/rtp/session.rs`
  - Files: `/src-tauri/src/infrastructure/rtp/codec.rs` (G.711)
  - Tests: Integration test - RTP session with mock peer
  - Dependencies: `webrtc-rtp`

- **OUT-2.5** (6h): Tauri `initiate_call` command
  - Files: `/src-tauri/src/commands/call.rs`
  - Tests: Integration test - initiate call via IPC

- **OUT-2.6** (10h): Frontend dialer UI + active call view
  - Files: `/src/lib/components/Dialer.svelte`
  - Files: `/src/lib/components/ActiveCall.svelte`
  - Files: `/src/lib/stores/callStore.ts`
  - Tests: Component tests

#### Inbound Calls (56 hours)
- **IN-3.1** (10h): INVITE listener for incoming calls
  - Files: `/src-tauri/src/infrastructure/sip/listener.rs`
  - Tests: Unit tests for INVITE handling

- **IN-3.2** (12h): Inbound SDP processing
  - Files: `/src-tauri/src/infrastructure/sip/sdp.rs` (extend)
  - Tests: Unit tests for SDP answer generation

- **IN-3.3** (10h): Call state machine for inbound calls
  - Files: `/src-tauri/src/services/call_service.rs` (extend)
  - Tests: Unit tests for inbound state flow

- **IN-3.4** (6h): Tauri `incoming_call` event emission
  - Files: `/src-tauri/src/commands/events.rs`
  - Tests: Integration test - event emission

- **IN-3.5** (8h): Tauri `answer_call` command
  - Files: `/src-tauri/src/commands/call.rs` (extend)
  - Tests: Integration test - answer flow

- **IN-3.6** (10h): Frontend incoming call notification UI
  - Files: `/src/lib/components/IncomingCall.svelte`
  - Tests: Component tests

### Deliverables
- ✅ Make outbound calls with audio
- ✅ Receive inbound calls with audio
- ✅ Working RTP bidirectional audio
- ✅ E2E tests for both call flows

### Milestone
**Demo:** Make outbound call to test number + receive inbound call + two-way audio working

### Completion Criteria
- [ ] Two-way audio working in test calls (manual verification)
- [ ] Call state transitions correctly (Idle → Ringing → Active → Ended)
- [ ] UI updates reflect call status in real-time
- [ ] E2E tests pass: Outbound call + Inbound call + Audio quality check

---

## Phase 4: Call Controls + Polish + Windows (84 hours, ~2.1 weeks)

**Goal:** Add call controls, Windows platform support, and production polish

### Tasks

#### Call Controls (24 hours)
- **CTL-4.1** (6h): BYE message handling (hangup)
  - Files: `/src-tauri/src/infrastructure/sip/bye.rs`
  - Tests: Unit tests for BYE message

- **CTL-4.2** (6h): Audio mute/unmute logic
  - Files: `/src-tauri/src/services/call_service.rs` (extend)
  - Tests: Unit tests for mute state

- **CTL-4.3** (4h): Tauri `hangup_call`, `mute_call` commands
  - Files: `/src-tauri/src/commands/call.rs` (extend)
  - Tests: Integration tests

- **CTL-4.4** (8h): Frontend call controls UI (hangup, mute buttons)
  - Files: `/src/lib/components/CallControls.svelte`
  - Tests: Component tests

#### Windows Platform Support (32 hours)
- **AUD-5.7** (20h): Windows WASAPI integration via `cpal`
  - Files: `/src-tauri/src/infrastructure/audio/windows.rs`
  - Tests: Integration test - Windows audio I/O
  - Platform: Windows 10+

- **SEC-6.3** (12h): Windows Credential Manager integration
  - Files: `/src-tauri/src/infrastructure/storage/credential_mgr.rs`
  - Tests: Integration test - Windows credential storage
  - Platform: Windows 10+

#### Production Polish (28 hours)
- **SEC-6.4** (6h): Tauri credential commands (`save_credentials`, `load_credentials`)
  - Files: `/src-tauri/src/commands/credentials.rs`
  - Tests: Integration tests

- **POL-7.1** (16h): Cross-platform testing and bug fixes
  - Tasks: Run full test suite on macOS + Windows
  - Tasks: Fix platform-specific issues
  - Tasks: Performance tuning (audio latency < 150ms)

- **POL-7.2** (6h): UI/UX polish and accessibility
  - Tasks: Keyboard navigation
  - Tasks: ARIA labels
  - Tasks: Color contrast
  - Tests: Accessibility audit with tools

### Deliverables
- ✅ Hangup, mute controls working
- ✅ Windows build with full feature parity
- ✅ Production-ready UI
- ✅ Complete E2E test suite

### Milestone
**Demo:** Full feature demo on macOS + Windows (registration, calls, controls, audio)

### Completion Criteria
- [ ] All MVP features working on macOS + Windows
- [ ] E2E tests pass on both platforms
- [ ] No critical bugs in issue tracker
- [ ] Code coverage ≥85% (Rust), ≥80% (SvelteKit)
- [ ] Audio latency < 150ms
- [ ] UI passes accessibility audit

---

## Dependencies Between Features

```
Secure Storage ──────┐
                     ├─→ SIP Registration ──→ Outbound Calls ──┐
                     │                                          ├─→ Call Controls
                     └─→ Audio Selection ───→ Inbound Calls ───┘
```

### Critical Path
1. **Secure Storage** (blocks SIP registration)
2. **SIP Registration** (blocks all calling features)
3. **Audio Selection** (blocks call flows)
4. **Call State Machine** (shared by outbound/inbound/controls)

---

## Parallel Work Opportunities

### Phase 1 (3 developers)
- **Dev 1:** Security + Storage (SEC-6.x)
- **Dev 2:** Audio system (AUD-5.1, AUD-5.2)
- **Dev 3:** SIP transport (SIP-1.1, SIP-1.2, SEC-6.6)

### Phase 2 (2 developers)
- **Dev 1:** SIP registration (SIP-1.x)
- **Dev 2:** Audio selection (AUD-5.x)

### Phase 3 (Sequential, some overlap)
- **Step 1:** Outbound flow (OUT-2.x) - 74h
- **Step 2:** Inbound flow (IN-3.x) - 56h
  - Note: RTP implementation (OUT-2.4) can overlap with SIP messages

### Phase 4 (3 developers)
- **Dev 1:** Call controls (CTL-4.x)
- **Dev 2:** Windows platform (AUD-5.7, SEC-6.3)
- **Dev 3:** UI polish (POL-7.x)

---

## Risk Management

### High-Risk Tasks
| Task | Risk | Impact | Mitigation |
|------|------|--------|------------|
| SIP-1.2 | Custom async layer complexity | Schedule delay | SPARC TDD, monitor rvoip, allocate buffer |
| OUT-2.4 | RTP audio quality issues | UX degradation | Jitter buffer, low latency buffers, test early |
| AUD-5.2 | macOS audio latency | UX degradation | Small buffer sizes, CoreAudio low-level API |

### Medium-Risk Tasks
| Task | Risk | Impact | Mitigation |
|------|------|--------|------------|
| OUT-2.2 | SDP negotiation edge cases | Call failures | Thorough unit tests, real server testing |
| AUD-5.7 | Windows audio differences | Platform issues | Test early on Windows, platform abstraction |
| IN-3.1 | Concurrent incoming calls | State corruption | Thread-safe state, mutex/RwLock |

---

## Timeline Visualization

### Gantt Chart (Weeks)
```
Week 1-2   [█████████ Phase 1: Core Infrastructure ██████████]
Week 3-4   [██████ Phase 2: Registration + Audio ███████]
Week 5-7   [████████████ Phase 3: Call Flows ██████████████]
Week 8-9   [████████ Phase 4: Polish + Windows ████████]
```

### Milestone Timeline
| Week | Milestone | Demo |
|------|-----------|------|
| 2 | Phase 1 Complete | Keychain + Audio devices + TLS |
| 4 | Phase 2 Complete | Register account + Select audio |
| 7 | Phase 3 Complete | Outbound + Inbound calls working |
| 9 | **MVP Complete** | **Full demo on macOS + Windows** |

---

## Success Metrics

### Technical Metrics
- [ ] Code coverage: Rust ≥85%, SvelteKit ≥80%
- [ ] Audio latency: <150ms end-to-end
- [ ] SIP registration time: <3 seconds
- [ ] Call setup time: <2 seconds
- [ ] Zero memory leaks in 24h stress test

### Quality Metrics
- [ ] Zero critical bugs
- [ ] All E2E tests passing on macOS + Windows
- [ ] Accessibility score ≥90 (Lighthouse)
- [ ] No security vulnerabilities (`cargo audit`)

### Feature Completeness
- [ ] All 6 MVP features implemented
- [ ] macOS build working
- [ ] Windows build working
- [ ] Documentation complete

---

## Post-MVP Backlog (Future Phases)

### Phase 5 (Future)
- Contact list management
- Call history with local storage
- DTMF tone support
- Call transfer (attended/blind)

### Phase 6 (Future)
- Linux support (ALSA/PulseAudio)
- Additional audio codecs (Opus, G.722)
- Conference calling
- Call recording

---

## Coordination Notes

### For Agents
- **Coder:** Follow this roadmap sequentially, TDD approach
- **Tester:** Create test infrastructure in Phase 1 (before implementation)
- **Reviewer:** Review code at end of each phase
- **SIP Specialist:** Focus on SIP-1.x tasks in Phases 1-3
- **Audio Engineer:** Focus on AUD-5.x tasks in Phases 1-2, 4
- **Tauri Engineer:** Focus on command handlers in Phases 2-4

### Memory Coordination
All roadmap data stored in swarm memory:
- `rustalk-mvp/implementation/feature_breakdown` - 39 detailed tasks
- `rustalk-mvp/implementation/dependencies` - Dependency graph
- `rustalk-mvp/implementation/phases` - This document
- `rustalk-mvp/implementation/risks` - Risk assessment

---

**Next Step:** Begin Phase 1 implementation using SPARC TDD methodology.
