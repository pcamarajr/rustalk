# RUSTALK

[![CI](https://img.shields.io/badge/CI-Phase%201%20Complete-green)](.github/workflows/ci.yml)

Open-source white-label VoIP desktop application built with Rust and Tauri.

## Project Status

**Phase 1 Complete** ✅ - Hello World with Frontend-Backend Communication

### Completed Features

- ✅ Tauri v2.x + SvelteKit project structure
- ✅ Working frontend-backend IPC communication
- ✅ Production macOS build (.app + .dmg)
- ✅ GitHub Actions CI workflow
- ✅ Basic build infrastructure

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
Phase 2: UI with Mocks            🔄 Next (~1.5 weeks)
Phase 3: Core Infrastructure      📋 Planned (~2.4 weeks)
Phase 4: Registration + Audio     📋 Planned (~1.7 weeks)
Phase 5: Call Flows               📋 Planned (~3.3 weeks)
Phase 6: Call Controls            📋 Planned (~0.6 weeks)
Phase 7: Windows Platform         📋 Planned (~1.3 weeks)
Phase 8: Production Polish        📋 Planned (~1.3 weeks)
```

**Total Estimated Duration**: ~12 weeks for MVP

## Current Phase: Phase 1 Complete ✅

### What Works

- Tauri app launches successfully
- Frontend (SvelteKit) communicates with backend (Rust) via IPC
- `greet` command demonstrates working frontend-to-backend communication
- macOS app bundle builds and runs
- CI workflow configured for automated testing

### Demo the Hello World App

1. Enter a name in the input field
2. Click "Greet" button
3. Backend (Rust) responds with greeting message
4. Message displays in the UI

## Next Steps: Phase 2

Phase 2 will implement:

- Complete UI foundation with design system
- All main screens (dialer, settings, active call, incoming call)
- State management with mock data
- Navigation and routing
- No real SIP/audio yet (mock actions only)

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
