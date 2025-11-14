# RUSTALK

[![CI](https://img.shields.io/badge/CI-Phase%202%20Complete-green)](.github/workflows/ci.yml)

Open-source white-label VoIP desktop application built with Rust and Tauri.

## Project Status

**Phase 2 Complete** ✅ - UI Foundation with Mock Actions

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
- `npm run tauri:dev` - Run Tauri app in development mode
- `npm run tauri:build` - Build production app bundle
- `npm run check` - Run TypeScript and Svelte checks
- `npm run lint` - Run linters
- `npm test` - Run tests (not yet implemented)

## Technology Stack

- **Frontend**: SvelteKit 2.x + TypeScript
- **Backend**: Rust (Tauri 2.x)
- **Build Tool**: Vite 6.x
- **Desktop Framework**: Tauri 2.x

## Architecture

See detailed architecture documentation in [`docs/architecture/`](docs/architecture/):

- [00-overview.md](docs/architecture/00-overview.md) - Architecture overview
- [05-implementation-roadmap.md](docs/architecture/05-implementation-roadmap.md) - Development roadmap (~12 weeks, 9 phases)
- [06-technology-decisions.md](docs/architecture/06-technology-decisions.md) - Technology choices and rationale
- [07-design-system.md](docs/architecture/07-design-system.md) - Design system guidelines
- [08-ui-design.md](docs/architecture/08-ui-design.md) - UI specifications

## Development Roadmap

```
Phase 0: Dev Container            ✅ (Skipped - using local environment)
Phase 1: Hello World              ✅ Complete (~0.5 weeks)
Phase 2: UI with Mocks            ✅ Complete (~1.5 weeks)
Phase 3: Core Infrastructure      🔄 Next (~2.4 weeks)
Phase 4: Registration + Audio     📋 Planned (~1.7 weeks)
Phase 5: Call Flows               📋 Planned (~3.3 weeks)
Phase 6: Call Controls            📋 Planned (~0.6 weeks)
Phase 7: Windows Platform         📋 Planned (~1.3 weeks)
Phase 8: Production Polish        📋 Planned (~1.3 weeks)
```

**Total Estimated Duration**: ~12 weeks for MVP

## Current Phase: Phase 2 Complete ✅

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

### Demo the UI

1. Navigate between screens using the sidebar
2. Enter a phone number in the dialer
3. Click "Call" to trigger a mock call flow
4. View call state transitions in the active call screen
5. Browse mock contacts and call history
6. Explore settings and audio device selection (UI only, no real audio yet)

## Next Steps: Phase 3

Phase 3 will implement:

- Secure credential storage (macOS Keychain integration)
- Audio device enumeration (macOS CoreAudio via `cpal`)
- SIP transport layer with TLS support
- Platform abstractions for security and audio
- Input validation for all Tauri commands

## Project Goals

**Vision**: An open-source VoIP softphone that companies can rebrand and distribute to their customers.

**Target Users**:

- VoIP service providers needing a softphone for customers
- Companies wanting internal VoIP communication tools
- Developers interested in Rust/Tauri/VoIP development

## License

Apache 2.0 - See [LICENSE](LICENSE)

## Contributing

This project is under active development following a structured 12-week roadmap. Please check the [implementation roadmap](docs/architecture/05-implementation-roadmap.md) for current status and planned features.

## IDE Setup

**Recommended**: [VS Code](https://code.visualstudio.com/) with:

- [Svelte for VS Code](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode)
- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Resources

- [Tauri Documentation](https://tauri.app)
- [SvelteKit Documentation](https://kit.svelte.dev)
- [Rust Documentation](https://www.rust-lang.org/learn)
