# Rustalk

## Vision

An open-source, white-label VoIP desktop application built with Rust and Tauri. Companies can rebrand it with their own identity and distribute it to their customers.

## What It Does

### Core Functionality

- Make and receive VoIP calls using standard SIP protocol
- Audio communication with device selection (microphone/speakers)
- Desktop application for Windows, macOS, and Linux
- Dialer interface for entering phone numbers
- Contact list management
- Call history
- Basic call controls (answer, hangup, mute)

### White-Label Features

- Customizable branding (logo, company name, colors)
- Configurable connection settings for different SIP providers
- Company-specific installer/distribution

## Technology Stack

**Backend**: Rust

- Real-time performance and memory safety
- Async runtime for concurrent operations
- Direct system access for audio and networking

**Frontend**: SvelteKit + TypeScript

- Modern UI components with file-based routing
- Type-safe development
- Component-based architecture

**Desktop Framework**: Tauri

- Lightweight cross-platform wrapper
- Native performance
- Small distribution size

**Protocols**: SIP, RTP

- Standard VoIP protocols for interoperability
- Works with existing SIP servers (Asterisk, FreeSWITCH, etc.)

## Target Audience

**Primary**: VoIP service providers who need a softphone to offer their customers

**Secondary**:

- Companies wanting internal VoIP communication tools
- Developers interested in Rust/Tauri/VoIP development
- Open source community

## License

Apache 2.0 - free to use, modify, and distribute commercially

## What Makes This Different

- **Open Source**: No vendor lock-in, community-driven
- **White-Label Ready**: Built from the ground up for rebranding
- **Modern Technology**: Rust + Tauri + SvelteKit instead of older C++/Electron approaches
- **Incremental Development**: Built with human-in-the-loop validation and testable deliverables at each phase

## Architecture

The project architecture is fully documented in `docs/architecture/`. Key documents:

- **[00-overview.md](docs/architecture/00-overview.md)** - High-level architecture overview
- **[05-implementation-roadmap.md](docs/architecture/05-implementation-roadmap.md)** - 7-phase development plan (~12 weeks)
- **[07-design-system.md](docs/architecture/07-design-system.md)** - Design system guidelines
- **[08-ui-design.md](docs/architecture/08-ui-design.md)** - UI design specifications

The architecture follows Clean Architecture principles with 5 distinct layers, prioritizing pure Rust implementations and cross-platform support (macOS first, then Windows, Linux future).
