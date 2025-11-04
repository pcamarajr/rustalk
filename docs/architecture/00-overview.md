# RUSTALK MVP Architecture Overview

**Version:** 1.0
**Date:** 2025-10-03
**Status:** Architecture Complete

## Executive Summary

RUSTALK is an open-source, white-label VoIP desktop application built with **Rust + Tauri + SvelteKit**. This document provides the architectural foundation for the MVP, designed for incremental, human-in-the-loop development with testable deliverables at each phase.

## Architecture Principles

1. **Clean Architecture**: Clear separation of concerns across 5 distinct layers
2. **Pure Rust**: Prioritize Rust-native libraries over FFI bindings
3. **Test-Driven**: 85%+ backend coverage, 80%+ frontend coverage
4. **Platform-First**: macOS primary, Windows secondary, Linux future
5. **Async-Native**: Tokio runtime for all I/O operations
6. **Type-Safe**: Leverage Rust and TypeScript's type systems

## Technology Stack

### Backend (Rust + Tauri)

- **Framework**: Tauri v1.x
- **SIP Protocol**: `rsip` (pure Rust parser/generator) + custom async layer
- **Audio I/O**: `cpal` (RustAudio, cross-platform)
- **RTP/Media**: `webrtc-rs` (RTP modules only)
- **Secure Storage**: `keyring` crate (Keychain/Credential Manager)
- **TLS**: `rustls` for SIPS support
- **Async Runtime**: Tokio

### Frontend (SvelteKit + TypeScript)

- **Framework**: SvelteKit (file-based routing)
- **Language**: TypeScript (strict mode)
- **Styling**: TailwindCSS
- **State**: Svelte stores (built-in)
- **IPC**: Tauri invoke API

### Testing

- **Rust**: `cargo-nextest`, `cargo-llvm-cov`, `mockall`
- **Frontend**: Vitest, Svelte Testing Library
- **E2E**: Playwright (macOS + Windows)
- **CI/CD**: GitHub Actions

## MVP Feature Scope

### Must-Have (Blockers)

1. SIP registration with credentials
2. Outbound calls (dial + initiate)
3. Inbound calls (receive + answer)
4. Call controls (answer, hangup, mute)
5. Audio device selection
6. Secure credential storage

### Timeline

- **Total Duration**: ~12 weeks (474 hours)
- **Phase 0**: Development Environment Setup (0.5 weeks, 20 hours)
- **Phase 1**: Project Scaffolding - "Hello World" (0.5 weeks, 20 hours)
- **Phase 2**: UI Foundation with Mock Actions (1.5 weeks, 60 hours)
- **Phase 3**: Core Infrastructure (2.4 weeks, 94 hours)
- **Phase 4**: SIP Registration + Audio Selection (1.7 weeks, 66 hours)
- **Phase 5**: Call Flows (3.3 weeks, 130 hours)
- **Phase 6**: Call Controls (0.6 weeks, 24 hours)
- **Phase 7**: Windows Platform Support (1.3 weeks, 32 hours)
- **Phase 8**: Production Polish (1.3 weeks, 28 hours)

## Architecture Layers

```
┌──────────────────────────────────────────┐
│     Presentation (SvelteKit UI)          │  /src
├──────────────────────────────────────────┤
│     IPC Boundary (Tauri Commands)        │  /src-tauri/src/commands
├──────────────────────────────────────────┤
│     Application (Business Logic)         │  /src-tauri/src/services
├──────────────────────────────────────────┤
│     Domain (Core Entities)               │  /src-tauri/src/domain
├──────────────────────────────────────────┤
│     Infrastructure (SIP, RTP, Audio)     │  /src-tauri/src/infrastructure
└──────────────────────────────────────────┘
```

## Key Design Decisions

### 1. SIP Library: rsip

- **Decision**: Use `rsip` as foundation, build custom async layer
- **Rationale**: Pure Rust, excellent type safety, RFC-compliant
- **Trade-off**: Significant custom development vs. complete stack
- **Risk**: Estimated 5-7 weeks of SIP stack development
- **Mitigation**: SPARC TDD methodology, phased implementation

### 2. Audio: cpal

- **Decision**: Use `cpal` for all audio I/O
- **Rationale**: Cross-platform, pure Rust, well-documented, battle-tested
- **Platforms**: CoreAudio (macOS), WASAPI (Windows)
- **Trade-off**: Callback-based API requires Tokio bridge
- **Mitigation**: Well-established pattern using channels

### 3. RTP: webrtc-rs

- **Decision**: Use RTP/RTCP modules from `webrtc-rs`
- **Rationale**: Most complete pure Rust RTP implementation
- **Risk**: Early development stage, may require upstream contributions
- **Mitigation**: Thorough testing, budget for bug fixes

### 4. Architecture Style: Clean + Hexagonal

- **Decision**: Clean Architecture with Hexagonal Pattern
- **Rationale**: Testability, platform abstraction, framework independence
- **Benefit**: Core logic isolated from external dependencies

## Documentation Structure

- **00-overview.md** (this file): High-level architecture summary
- **01-layers.md**: Detailed layer responsibilities
- **05-implementation-roadmap.md**: 8-phase development plan
- **06-technology-decisions.md**: Library research and rationale
- **07-design-system.md**: Design system guidelines, components, and patterns
- **08-ui-design.md**: UI design specifications and screen layouts
- **02-backend-modules.md**: Rust module structure (coming soon)
- **03-frontend-structure.md**: SvelteKit organization (coming soon)
- **04-testing-strategy.md**: Comprehensive testing approach (coming soon)
- **07-build-ci-pipeline.md**: Build and deployment automation (coming soon)

## Next Steps

1. **Phase 0**: Set up development container environment with Rust, Node.js, Tauri prerequisites
2. **Phase 1**: Initialize Tauri + SvelteKit project and create "Hello World" demo
3. **Phase 2**: Implement UI foundation with design system and mock actions (reference [07-design-system.md](07-design-system.md) and [08-ui-design.md](08-ui-design.md))
4. **Phase 3**: Begin core infrastructure (security, audio, SIP transport) with TDD approach

## Development Methodology

This architecture supports incremental development with:

- **Foundation First**: Dev container → Hello World → UI mocks before core functionality
- **Testable Deliverables**: Each phase produces a demonstrable, working deliverable
- **Human Validation**: Clear checkpoints for review before proceeding
- **Design-Driven**: UI foundation established early with design system integration
- **Iterative Refinement**: Build → Test → Validate → Refine cycle

---

**Architecture Status**: Complete and ready for implementation
**Development Approach**: Incremental, human-in-the-loop development
