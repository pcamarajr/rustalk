# RUSTALK MVP Architecture Overview

**Version:** 1.0
**Date:** 2025-10-03
**Status:** Approved by Hive Mind Consensus

## Executive Summary

RUSTALK is an open-source, white-label VoIP desktop application built with **Rust + Tauri + SvelteKit**. This document provides the architectural foundation for the MVP, designed entirely by autonomous AI agents using collective intelligence and SPARC methodology.

## Architecture Principles

1. **Clean Architecture**: Clear separation of concerns across 5 distinct layers
2. **Pure Rust**: Prioritize Rust-native libraries over FFI bindings
3. **Test-Driven**: 85%+ backend coverage, 80%+ frontend coverage
4. **Platform-First**: macOS primary, Windows secondary, Linux future
5. **Async-Native**: Tokio runtime for all I/O operations
6. **Type-Safe**: Leverage Rust and TypeScript's type systems

## Technology Stack (Consensus Approved)

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

- **Total Duration**: 9.4 weeks (374 hours)
- **Phase 1**: Core Infrastructure (2.4 weeks)
- **Phase 2**: SIP Registration + Audio Selection (1.7 weeks)
- **Phase 3**: Call Flows (3.3 weeks)
- **Phase 4**: Polish + Windows (2.1 weeks)

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
- **02-backend-modules.md**: Rust module structure
- **03-frontend-structure.md**: SvelteKit organization
- **04-testing-strategy.md**: Comprehensive testing approach
- **05-implementation-roadmap.md**: 4-phase development plan
- **06-technology-decisions.md**: Library research and rationale
- **07-build-ci-pipeline.md**: Build and deployment automation

## Next Steps

1. **Phase 1 Implementation**: Begin core infrastructure (security, audio, SIP transport)
2. **SPARC Specification**: Create detailed SPARC specs for each feature
3. **Test Infrastructure**: Set up testing frameworks and CI pipeline
4. **Library Evaluation**: Prototype `rsip` + Tokio integration

## Hive Mind Coordination

This architecture was designed through collective intelligence:

- **Researcher Agent**: Technology research and library evaluation
- **Analyst Agent**: Architecture design and module structure
- **Tester Agent**: Testing strategy and quality assurance
- **Coder Agent**: Implementation roadmap and phasing

All decisions stored in swarm memory under namespace `rustalk-mvp` for future agent coordination.

---

**Approved by**: Queen Coordinator (Strategic)
**Consensus Algorithm**: Majority
**Worker Count**: 4 specialized agents
**Swarm ID**: swarm-1759490413384-mo4ol70oc
