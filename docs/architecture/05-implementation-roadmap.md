# Implementation Roadmap - RUSTALK MVP

**Total Duration:** ~12 weeks (474 hours)
**Approach:** 9-phase incremental delivery with testable deliverables
**Methodology:** Incremental, human-in-the-loop development
**Platform Focus:** macOS first, then Windows

## Overview

```
Phase 0: Dev Container            ███░░░░░░░░░░░░░  0.5 weeks (20h)
Phase 1: Hello World              ███░░░░░░░░░░░░░  0.5 weeks (20h) COMPLETED
Phase 2: UI with Mocks            ██████░░░░░░░░░░  1.5 weeks (60h) COMPLETED
Phase 3: Core Infrastructure      ████████░░░░░░░░  2.4 weeks (94h) COMPLETED
Phase 4: Registration + Audio     ████░░░░░░░░░░░░  1.7 weeks (66h)
Phase 5: Call Flows               ████████░░░░░░░░  3.3 weeks (130h)
Phase 6: Call Controls            ██░░░░░░░░░░░░░░  0.6 weeks (24h)
Phase 7: Windows Platform         ████░░░░░░░░░░░░  1.3 weeks (32h)
Phase 8: Production Polish        █████░░░░░░░░░░░  1.3 weeks (28h)
                                  ═══════════════════
                                  Total: ~12 weeks (474h)
```

## Phase 0: Development Environment Setup (20 hours, ~0.5 weeks)

**Goal:** Create reproducible development environment using Docker dev container

### Tasks

#### Dev Container Configuration (20 hours)

- **DEV-0.1** (8h): Configure Docker dev container with Rust, Node.js, Tauri prerequisites

  - Files: `.devcontainer/devcontainer.json`, `Dockerfile`
  - Setup: Rust toolchain (latest stable), Node.js 18+, npm/yarn, Tauri CLI
  - Dependencies: System libraries for Tauri (required for macOS/Windows builds)
  - Tests: Verify container builds and all tools are available

- **DEV-0.2** (6h): Setup VS Code/Cursor extensions and settings

  - Files: `.devcontainer/devcontainer.json` (extensions), `.vscode/settings.json`
  - Extensions: Rust Analyzer, ESLint, Prettier, Svelte extension
  - Settings: Format on save, default formatters, Rust settings
  - Tests: Verify extensions load correctly in container

- **DEV-0.3** (6h): Create reproducible development environment
  - Files: `.devcontainer/`, `docker-compose.yml` (if needed)
  - Documentation: `docs/setup.md` with setup instructions
  - Verify: Multiple developers can clone and start development immediately
  - Tests: Test container startup, verify all tools work

### Deliverables

- ✅ Fully functional dev container with all prerequisites
- ✅ VS Code/Cursor configured with recommended extensions
- ✅ Documentation for setting up development environment
- ✅ Verified cross-platform container (at least macOS)

### Milestone

**Demo:** Clone repo, open in VS Code/Cursor, dev container starts, all tools verified

### Completion Criteria

- [x] Dev container builds successfully
- [x] Rust, Node.js, Tauri CLI all available and working
- [x] VS Code/Cursor extensions load correctly
- [x] Documentation complete for new developers

---

## Phase 1: Project Scaffolding - "Hello World" (20 hours, ~0.5 weeks)

**Goal:** Initialize Tauri + SvelteKit project structure and verify basic frontend-backend communication

### Tasks

#### Project Initialization (20 hours)

- **INIT-1.1** (8h): Initialize Tauri + SvelteKit project structure

  - Files: `src-tauri/`, `src/`, `package.json`, `Cargo.toml`
  - Setup: Run `npm create tauri-app@latest` with SvelteKit template
  - Structure: Follow Tauri v2.x best practices
  - Tests: Verify project builds successfully

- **INIT-1.2** (6h): Setup basic build and test infrastructure

  - Files: `.github/workflows/` (basic CI), `package.json` scripts
  - Build: Setup npm scripts for dev/build/test
  - CI: Basic workflow to verify build (macOS only for now)
  - Tests: Verify build pipeline works

- **INIT-1.3** (4h): Create hello world UI with Tauri command

  - Files: `src/routes/+page.svelte`, `src-tauri/src/commands/greetings.rs`
  - Frontend: Simple SvelteKit page with button
  - Backend: Tauri command that returns greeting message
  - Communication: Verify Tauri `invoke` works from frontend
  - Tests: Manual test - click button, verify message displayed

- **INIT-1.4** (2h): Verify cross-platform build (macOS)
  - Files: Build output verification
  - Test: Run `npm run tauri build` and verify macOS app builds
  - Verify: App bundle created, can launch successfully

### Deliverables

- ✅ Working "Hello World" app with frontend-backend communication
- ✅ Basic build infrastructure (dev, build, test scripts)
- ✅ Tauri IPC communication verified
- ✅ macOS build verified

### Milestone

**Demo:** Launch app, click button, backend responds with greeting message via Tauri IPC

### Completion Criteria

- [x] Tauri + SvelteKit project initialized
- [x] Hello World UI displays and calls Tauri command
- [x] Tauri command returns message successfully
- [x] macOS app builds and runs
- [x] Basic CI workflow configured

---

## Phase 2: UI Foundation with Mock Actions (60 hours, ~1.5 weeks)

**Goal:** Implement complete UI foundation with design system and mock actions (no real SIP/audio yet)

### Tasks

#### Design System Implementation (20 hours)

- **UI-2.1** (12h): Implement design system (CSS variables, components)

  - Files: `src/lib/styles/design-system.css`, `src/lib/components/`
  - Reference: [07-design-system.md](07-design-system.md) for styling guidelines
  - Setup: CSS variables for colors, typography, spacing, shadows
  - Components: Button variants, Input components, Card components
  - Tests: Visual regression (manual), component structure tests

- **UI-2.2** (8h): Create reusable Svelte components from design system
  - Files: `src/lib/components/Button.svelte`, `Input.svelte`, `Card.svelte`, etc.
  - Components: Primary/Secondary buttons, Text input, Phone input, Select dropdown
  - Styling: Use CSS variables from design system
  - Tests: Component unit tests with Vitest

#### Screen Layouts (25 hours)

- **UI-2.3** (8h): Create main screen layouts (dialer, settings, active call)

  - Files: `src/routes/`, `src/lib/components/Dialer.svelte`, `ActiveCall.svelte`, `Settings.svelte`
  - Reference: [08-ui-design.md](08-ui-design.md) for screen layouts
  - Layouts: Main dialer screen, settings screen, active call screen, incoming call screen
  - Routing: Setup SvelteKit file-based routing
  - Tests: Screen layout tests, routing tests

- **UI-2.4** (10h): Implement mock actions (dialer buttons, call controls)

  - Files: `src/lib/components/Dialer.svelte`, `CallControls.svelte`
  - Actions: Dialer pad buttons, Call button, Mute/Hold/End call buttons
  - Mock: All buttons trigger state changes but no real SIP/audio calls
  - State: Mock call state transitions (Idle → Ringing → Active → Ended)
  - Tests: Component interaction tests

- **UI-2.5** (7h): Setup routing and navigation
  - Files: `src/routes/`, `src/lib/components/Navigation.svelte`
  - Routes: `/` (dialer), `/settings`, `/contacts`, `/history`
  - Navigation: Sidebar navigation component
  - Tests: Routing tests, navigation tests

#### State Management (15 hours)

- **UI-2.6** (8h): Implement state management with mock data

  - Files: `src/lib/stores/callStore.ts`, `authStore.ts`, `audioStore.ts`
  - Stores: Svelte stores for call state, auth state, audio device state
  - Mock Data: Mock contacts, mock call history, mock audio devices
  - Integration: Connect stores to UI components
  - Tests: Store unit tests

- **UI-2.7** (7h): Connect UI components to state stores
  - Files: Update all UI components to use stores
  - Integration: Components read from and update stores
  - Mock Actions: Button clicks update store state (mock behavior)
  - Tests: Integration tests for store + component interactions

### Deliverables

- ✅ Fully functional UI with mock actions (no real SIP/audio)
- ✅ Design system implemented and used throughout
- ✅ All main screens implemented (dialer, settings, active call, incoming call)
- ✅ State management working with mock data
- ✅ Navigation and routing complete

### Milestone

**Demo:** Navigate between screens, interact with dialer, trigger mock call flow, view mock call history

### Completion Criteria

- [x] Design system CSS variables implemented
- [x] All main UI components created and styled
- [x] Main screens implemented per [08-ui-design.md](08-ui-design.md)
- [x] Mock actions trigger state changes
- [x] Navigation works between all screens
- [x] State management stores working with mock data
- [x] UI looks polished and matches design system

---

## Phase 3: Core Infrastructure (94 hours, ~2.4 weeks)

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

- [x] `cargo test` passes with 85%+ coverage (134 tests passing; coverage verification via `cargo llvm-cov`)
- [x] Manual verification: Credentials persist in macOS Keychain (see `scripts/verify_phase3.sh`)
- [x] TLS connection succeeds to test SIP server (integration test added: `tests/sip_transport_integration_test.rs`)
- [x] Audio devices appear in enumeration list (see `scripts/verify_phase3.sh`)

---

## Phase 4: SIP Registration + Audio Selection (66 hours, ~1.7 weeks)

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

## Phase 5: Call Flows - Outbound + Inbound (130 hours, ~3.3 weeks)

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

## Phase 6: Call Controls (24 hours, ~0.6 weeks)

**Goal:** Add interactive call controls (hangup, mute) for active calls

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

### Deliverables

- ✅ Hangup control working (end active calls)
- ✅ Mute/unmute control working during calls
- ✅ UI buttons functional with real SIP commands
- ✅ Call state properly updated on hangup

### Milestone

**Demo:** Make call → Mute/Unmute → Hangup call → Verify call ends cleanly

### Completion Criteria

- [ ] Hangup command sends BYE message correctly
- [ ] Mute/unmute toggles audio transmission
- [ ] UI buttons trigger correct backend commands
- [ ] Call state transitions correctly on hangup
- [ ] Integration tests pass for all controls

---

## Phase 7: Windows Platform Support (32 hours, ~1.3 weeks)

**Goal:** Add full Windows platform support with feature parity to macOS

### Tasks

#### Windows Platform Support (32 hours)

- **AUD-5.7** (20h): Windows WASAPI integration via `cpal`

  - Files: `/src-tauri/src/infrastructure/audio/windows.rs`
  - Tests: Integration test - Windows audio I/O
  - Platform: Windows 10+
  - Features: Audio device enumeration, audio streaming, device switching

- **SEC-6.3** (12h): Windows Credential Manager integration
  - Files: `/src-tauri/src/infrastructure/storage/credential_mgr.rs`
  - Tests: Integration test - Windows credential storage
  - Platform: Windows 10+
  - Features: Secure credential storage using Windows Credential Manager API

### Deliverables

- ✅ Windows build working with full feature parity
- ✅ Windows audio devices enumerate and work
- ✅ Credentials stored securely in Windows Credential Manager
- ✅ All core features tested on Windows

### Milestone

**Demo:** Full feature demo on Windows (registration, calls, controls, audio) matching macOS functionality

### Completion Criteria

- [ ] Windows app builds successfully
- [ ] Audio input/output working on Windows
- [ ] Credentials persist in Windows Credential Manager
- [ ] All E2E tests pass on Windows
- [ ] Windows-specific issues resolved

---

## Phase 8: Production Polish (28 hours, ~1.3 weeks)

**Goal:** Final polish, testing, accessibility, and production readiness

### Tasks

#### Production Polish (28 hours)

- **SEC-6.4** (6h): Tauri credential commands (`save_credentials`, `load_credentials`)

  - Files: `/src-tauri/src/commands/credentials.rs`
  - Tests: Integration tests
  - Purpose: Complete credential management API

- **POL-7.1** (16h): Cross-platform testing and bug fixes

  - Tasks: Run full test suite on macOS + Windows
  - Tasks: Fix platform-specific issues
  - Tasks: Performance tuning (audio latency < 150ms)
  - Tasks: Memory leak testing and fixes
  - Tasks: Stress testing (long calls, multiple calls)

- **POL-7.2** (6h): UI/UX polish and accessibility
  - Tasks: Keyboard navigation (full keyboard support)
  - Tasks: ARIA labels for screen readers
  - Tasks: Color contrast (WCAG AA compliance)
  - Tasks: Focus management
  - Tests: Accessibility audit with tools (Lighthouse, axe)

### Deliverables

- ✅ Production-ready UI with accessibility compliance
- ✅ Cross-platform testing complete (macOS + Windows)
- ✅ Performance optimized (audio latency < 150ms)
- ✅ Complete E2E test suite passing on both platforms
- ✅ Zero critical bugs

### Milestone

**Demo:** Full production-ready demo on macOS + Windows with all polish, accessibility, and performance optimizations

### Completion Criteria

- [ ] All MVP features working perfectly on macOS + Windows
- [ ] E2E tests pass on both platforms
- [ ] No critical bugs in issue tracker
- [ ] Code coverage ≥85% (Rust), ≥80% (SvelteKit)
- [ ] Audio latency < 150ms end-to-end
- [ ] UI passes accessibility audit (WCAG AA)
- [ ] Keyboard navigation complete
- [ ] Performance metrics meet targets

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

### Phase 3 (3 developers)

- **Dev 1:** Security + Storage (SEC-6.x)
- **Dev 2:** Audio system (AUD-5.1, AUD-5.2)
- **Dev 3:** SIP transport (SIP-1.1, SIP-1.2, SEC-6.6)

### Phase 4 (2 developers)

- **Dev 1:** SIP registration (SIP-1.x)
- **Dev 2:** Audio selection (AUD-5.x)

### Phase 5 (Sequential, some overlap)

- **Step 1:** Outbound flow (OUT-2.x) - 74h
- **Step 2:** Inbound flow (IN-3.x) - 56h
  - Note: RTP implementation (OUT-2.4) can overlap with SIP messages

### Phase 6 (1 developer)

- **Dev 1:** Call controls (CTL-4.x) - Focused phase, can overlap with Phase 5 testing

### Phase 7 (1 developer)

- **Dev 1:** Windows platform (AUD-5.7, SEC-6.3) - Can work in parallel with Phase 6 if separate developer

### Phase 8 (2-3 developers)

- **Dev 1:** Cross-platform testing and bug fixes (POL-7.1)
- **Dev 2:** UI polish and accessibility (POL-7.2)
- **Dev 3:** Credential commands (SEC-6.4) - if needed

---

## Risk Management

### High-Risk Tasks

| Task    | Risk                          | Impact         | Mitigation                                     |
| ------- | ----------------------------- | -------------- | ---------------------------------------------- |
| SIP-1.2 | Custom async layer complexity | Schedule delay | SPARC TDD, monitor rvoip, allocate buffer      |
| OUT-2.4 | RTP audio quality issues      | UX degradation | Jitter buffer, low latency buffers, test early |
| AUD-5.2 | macOS audio latency           | UX degradation | Small buffer sizes, CoreAudio low-level API    |

### Medium-Risk Tasks

| Task    | Risk                       | Impact           | Mitigation                                  |
| ------- | -------------------------- | ---------------- | ------------------------------------------- |
| OUT-2.2 | SDP negotiation edge cases | Call failures    | Thorough unit tests, real server testing    |
| AUD-5.7 | Windows audio differences  | Platform issues  | Test early on Windows, platform abstraction |
| IN-3.1  | Concurrent incoming calls  | State corruption | Thread-safe state, mutex/RwLock             |

---

## Timeline Visualization

### Gantt Chart (Weeks)

```
Week 0.5   [███ Phase 0: Dev Container ███]
Week 1     [███ Phase 1: Hello World ███]
Week 2-3   [██████ Phase 2: UI with Mocks ██████]
Week 4-5   [█████████ Phase 3: Core Infrastructure ██████████]
Week 6-7   [██████ Phase 4: Registration + Audio ███████]
Week 8-10  [████████████ Phase 5: Call Flows ██████████████]
Week 10-11 [██ Phase 6: Call Controls ██] [███ Phase 7: Windows ███]
Week 11-12 [█████ Phase 8: Production Polish █████]
```

### Milestone Timeline

| Week | Milestone        | Demo                             |
| ---- | ---------------- | -------------------------------- |
| 0.5  | Phase 0 Complete | Dev container working            |
| 1    | Phase 1 Complete | Hello World app running          |
| 3    | Phase 2 Complete | Full UI with mock actions        |
| 5    | Phase 3 Complete | Keychain + Audio devices + TLS   |
| 7    | Phase 4 Complete | Register account + Select audio  |
| 10   | Phase 5 Complete | Outbound + Inbound calls working |
| 10.5 | Phase 6 Complete | Call controls (hangup, mute)     |
| 11.5 | Phase 7 Complete | Windows platform working         |
| 12   | **MVP Complete** | **Full production-ready demo**   |

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

## Development Approach

### Incremental Deliverables

Each phase produces a testable, demonstrable deliverable:

- **Phase 0:** Working dev container
- **Phase 1:** Hello World app with frontend-backend communication
- **Phase 2:** Complete UI with mock actions (no real SIP/audio)
- **Phase 3:** Core infrastructure (security, audio enumeration, SIP transport)
- **Phase 4:** SIP registration and audio selection UI
- **Phase 5:** Full call flows with real SIP and audio
- **Phase 6:** Call controls (hangup, mute) for active calls
- **Phase 7:** Windows platform support with feature parity
- **Phase 8:** Production-ready MVP with polish and accessibility

### Human-in-the-Loop Validation

Clear checkpoints for validation before proceeding:

1. **Phase 0-2:** Foundation and UI - Validate design and user experience
2. **Phase 3:** Core infrastructure - Validate architecture decisions
3. **Phase 4:** Registration - Validate SIP connectivity
4. **Phase 5:** Call flows - Validate end-to-end calling
5. **Phase 6:** Call controls - Validate interactive call management
6. **Phase 7:** Windows - Validate cross-platform support
7. **Phase 8:** Polish - Final validation before production MVP

### Test-Driven Development (TDD)

From Phase 3 onward, use TDD approach:

- Write tests before implementation
- Ensure 85%+ Rust coverage, 80%+ frontend coverage
- Integration tests for each major feature

---

**Next Step:** Begin Phase 1 implementation - initialize Tauri + SvelteKit project structure.
