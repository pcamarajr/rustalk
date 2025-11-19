# RUSTALK

[![CI](https://img.shields.io/badge/CI-Phase%202%20Complete-green)](.github/workflows/ci.yml)

Open-source white-label VoIP desktop application built with Rust and Tauri.

## Project Status

**Phase 4 Complete** ✅ - Registration + Audio + Credential Persistence  
**Phase 5 Next** 🔄 - Call Flows (Outbound + Inbound)

### Completed Features

**Phase 1:**

- ✅ Tauri v2.x + SvelteKit project structure
- ✅ Working frontend-backend IPC communication
- ✅ Production macOS build (.app + .dmg)
- ✅ GitHub Actions CI workflow
- ✅ Basic build infrastructure

**Phase 2:**

- ✅ Complete design system with CSS variables and reusable components
- ✅ All main screens implemented (dialer, settings, active call, incoming call, contacts, history)
- ✅ State management with Svelte stores (call, auth, audio, contacts, history)
- ✅ Navigation and routing between all screens
- ✅ Mock actions for dialer, call controls, and call state transitions
- ✅ UI components library (buttons, inputs, cards, dialogs, etc.)

**Phase 3:**

- ✅ Secure credential storage (macOS Keychain integration)
- ✅ Audio device enumeration (macOS CoreAudio via `cpal`)
- ✅ SIP transport layer (UDP/TCP/TLS) with async Tokio
- ✅ SIP message parsing and building (rsip integration)
- ✅ TLS certificate validation for SIPS
- ✅ Platform abstractions (AudioEngine, CredentialStore traits)
- ✅ Input validation for Tauri commands
- ✅ Integration tests for SIP transport (tested with Asterisk server)

**Phase 4 (Complete):**

- ✅ SIP registration backend (SIP-1.3, SIP-1.4, SIP-1.5)
- ✅ Frontend registration UI (SIP-1.6) - Login page with form validation
- ✅ Real Tauri command integration for registration
- ✅ Registration status polling
- ✅ Audio device enumeration API (AUD-5.3)
- ✅ Device selection and switching logic (AUD-5.4)
- ✅ Tauri audio commands (AUD-5.5)
- ✅ Frontend audio settings UI (AUD-5.6)
- ✅ Manual verification: Registration and audio devices working

**Phase 4.1 (Complete):**

- ✅ Credential store integrated into AppState (SEC-6.7)
- ✅ Save credentials after successful registration (SEC-6.8)
- ✅ Load credentials on app startup and auto-login (SEC-6.9)
- ✅ E2E test suite (POL-7.3)

## Quick Start

### Prerequisites

- **Node.js** 18+ (tested with v22.16.0)
- **Rust** 1.80+ (tested with v1.91.1)
- **npm** 10+

### Development

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri:dev
```

### Build for Production

```bash
# Build macOS app
npm run tauri:build

# Output: src-tauri/target/release/bundle/
```

### Available Scripts

- `npm run dev` - Start Vite dev server (frontend only)
- `npm run build` - Build frontend for production
- `npm run tauri:dev` - Run Tauri app in development mode (starts local SIP server automatically)
- `npm run tauri:dev:stop` - Stop Tauri dev server and SIP server
- `npm run tauri:build` - Build production app bundle
- `npm run docker:start` - Start local Asterisk SIP server
- `npm run docker:stop` - Stop local Asterisk SIP server
- `npm run check` - Run TypeScript and Svelte checks
- `npm run lint` - Run linters
- `npm test` - Run tests (not yet implemented)

## Local SIP Testing Environment

Rustalk includes a local Asterisk SIP server for testing SIP integration. The server starts automatically with `npm run tauri:dev`.

### Quick Start

```bash
# Start SIP server manually (if needed)
npm run docker:start

# Stop SIP server
npm run docker:stop
```

### Features

- **Automatic startup**: SIP server starts with `npm run tauri:dev`
- **Pre-configured test user**: `testuser` / `testpass` for integration testing
- **SIP support**: UDP and TCP transports (TLS disabled - requires certificate configuration)
- **Persistent configuration**: Config files persist across restarts

### Documentation

For detailed setup instructions, troubleshooting, and configuration options, see:

- **[SIP Testing Environment Guide](docs/development/sip-testing-environment.md)** - Complete guide to using the local SIP server

### Prerequisites

- **Docker Desktop** (or Docker Engine + Docker Compose)
  - Download from: https://www.docker.com/products/docker-desktop
  - Verify: `docker --version` and `docker-compose --version`

## Technology Stack

- **Frontend**: SvelteKit 2.x + TypeScript
- **Backend**: Rust (Tauri 2.x)
- **Build Tool**: Vite 6.x
- **Desktop Framework**: Tauri 2.x

## Architecture

See detailed architecture documentation in [`docs/architecture/`](docs/architecture/):

- [00-overview.md](docs/architecture/00-overview.md) - Architecture overview
- [05-implementation-roadmap.md](docs/architecture/05-implementation-roadmap.md) - Development roadmap (~12.4 weeks, 9 phases + Phase 4.1)
- [06-technology-decisions.md](docs/architecture/06-technology-decisions.md) - Technology choices and rationale
- [07-design-system.md](docs/architecture/07-design-system.md) - Design system guidelines
- [08-ui-design.md](docs/architecture/08-ui-design.md) - UI specifications

## Development Roadmap

```
Phase 0: Dev Container            ✅ (Skipped - using local environment)
Phase 1: Hello World              ✅ Complete (~0.5 weeks)
Phase 2: UI with Mocks            ✅ Complete (~1.5 weeks)
Phase 3: Core Infrastructure      ✅ Complete (~2.4 weeks)
Phase 4: Registration + Audio     ✅ Complete (~1.7 weeks)
Phase 4.1: Credentials + E2E      ✅ Complete (~0.4 weeks)
Phase 5: Call Flows               🔄 Next (~3.3 weeks)
Phase 6: Call Controls            📋 Planned (~0.6 weeks)
Phase 7: Windows Platform         📋 Planned (~1.3 weeks)
Phase 8: Production Polish        📋 Planned (~1.3 weeks)
```

**Total Estimated Duration**: ~12.4 weeks for MVP

## Current Phase: Phase 5 Next 🔄

Phase 4 and Phase 4.1 are complete. Phase 5 will implement full call lifecycle with bidirectional RTP audio for outbound and inbound calls.

### What Works

**Foundation:**

- Tauri app launches successfully
- Frontend (SvelteKit) communicates with backend (Rust) via IPC
- macOS app bundle builds and runs
- CI workflow configured for automated testing

**UI & Navigation:**

- Complete design system with CSS variables and component library
- All main screens functional (dialer, settings, active call, incoming call, contacts, history)
- Navigation between screens via sidebar
- State management with Svelte stores (call state, auth state, audio devices, contacts, history)

**Mock Functionality:**

- Dialer pad with number input
- Mock call initiation and state transitions (Idle → Ringing → Active → Ended)
- Call controls UI (mute, hold, end call buttons)
- Incoming call notification UI
- Settings screens with audio device selection UI
- Contacts and call history with mock data

**Core Infrastructure (Phase 3):**

- Secure credential storage via macOS Keychain
- Audio device enumeration and management (macOS CoreAudio)
- SIP transport layer (UDP/TCP/TLS) with async support
- SIP message parsing and building (rsip integration)
- TLS certificate validation for secure SIP connections
- Platform abstractions for cross-platform support
- Integration tests validated with real Asterisk server

**SIP Registration & Audio (Phase 4):**

- SIP registration with digest authentication (401 challenge/response)
- Registration state machine (Unregistered → Registering → Registered)
- Frontend login UI with form validation
- Real SIP account registration via Tauri commands
- Audio device enumeration and selection
- Audio device switching in settings UI
- Manual verification: Registration and audio devices working with test SIP server

### Demo the UI

1. Navigate between screens using the sidebar
2. **Register a SIP account** via the login page (use test SIP server credentials)
3. **Select audio devices** in Settings → Audio (microphone and speaker)
4. Enter a phone number in the dialer
5. Click "Call" to trigger a mock call flow
6. View call state transitions in the active call screen
7. Browse mock contacts and call history

## Next Steps: Phase 5

Phase 5 will implement full call lifecycle with bidirectional RTP audio:

**Outbound Calls:**

- **OUT-2.1**: INVITE message construction ✅
- **OUT-2.2**: SDP offer/answer negotiation ✅
- **OUT-2.3**: Call state machine for outbound calls ✅
- **OUT-2.4**: RTP session setup and audio streaming ✅
- **OUT-2.5**: Tauri `initiate_call` command ✅
- **OUT-2.6**: Frontend dialer UI + active call view ✅
- **OUT-2.6.1**: SIP message receiver loop ✅

**Inbound Calls:**

- **IN-3.1**: INVITE listener for incoming calls
- **IN-3.2**: Inbound SDP processing
- **IN-3.3**: Call state machine for inbound calls
- **IN-3.4**: Tauri `incoming_call` event emission
- **IN-3.5**: Tauri `answer_call` command
- **IN-3.6**: Frontend incoming call notification UI

## Project Goals

**Vision**: An open-source VoIP softphone that companies can rebrand and distribute to their customers.

**Target Users**:

- VoIP service providers needing a softphone for customers
- Companies wanting internal VoIP communication tools
- Developers interested in Rust/Tauri/VoIP development

## License

Apache 2.0 - See [LICENSE](LICENSE)

## Contributing

This project is under active development following a structured 12.4-week roadmap. Please check the [implementation roadmap](docs/architecture/05-implementation-roadmap.md) for current status and planned features.

## IDE Setup

**Recommended**: [VS Code](https://code.visualstudio.com/) with:

- [Svelte for VS Code](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode)
- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Resources

- [Tauri Documentation](https://tauri.app)
- [SvelteKit Documentation](https://kit.svelte.dev)
- [Rust Documentation](https://www.rust-lang.org/learn)
