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

**Frontend**: React + TypeScript

- Modern UI components
- Type-safe development

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
- **Modern Technology**: Rust + Tauri instead of older C++/Electron approaches
- **AI-Developed**: Built entirely by autonomous AI agents using claude-flow

---

**Note to AI Agents**: This is the project vision. Design and build everything needed to make this real.
