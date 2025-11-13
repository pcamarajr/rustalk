# Technology Decisions - RUSTALK MVP

**Research Date:** 2025-10-03
**Decision Authority:** Collaborative technology evaluation and decision-making

## Decision Summary

| Component      | Choice                | Alternatives Considered         | Status      |
| -------------- | --------------------- | ------------------------------- | ----------- |
| SIP Library    | `rsip` + custom async | rvoip, pjsip-rs                 | ✅ Approved |
| Audio I/O      | `cpal`                | coreaudio-rs, platform-specific | ✅ Approved |
| RTP/Media      | `webrtc-rs` (modules) | Custom RTP, rvoip               | ✅ Approved |
| Secure Storage | `keyring`             | platform-specific               | ✅ Approved |
| TLS            | `rustls`              | native-tls                      | ✅ Approved |
| Async Runtime  | Tokio                 | async-std                       | ✅ Approved |

---

## 1. SIP Protocol Library

### ✅ APPROVED: rsip + Custom Async Layer

**Repository:** https://github.com/Televiska/rsip
**Version:** Latest on crates.io
**License:** MIT

#### Decision Criteria Scores

- **Documentation Quality:** ⭐⭐⭐⭐ (Good)
- **Examples/Boilerplate:** ⭐⭐⭐ (Moderate)
- **Battle-Tested:** ⭐⭐⭐ (Moderate - 100 stars, active dev)
- **Async/Tokio Support:** ⭐⭐ (Requires custom work)
- **Type Safety:** ⭐⭐⭐⭐⭐ (Excellent - nom parser)
- **Community:** ⭐⭐⭐ (Moderate - 24 forks, active)

#### Pros

- ✅ Pure Rust implementation (no FFI)
- ✅ Zero-cost abstractions with lazy header parsing
- ✅ Follows RFC 3261 specifications closely
- ✅ Strongly typed SIP message components
- ✅ Intended as foundational library for Rust SIP ecosystem
- ✅ Good API documentation on docs.rs

#### Cons

- ⚠️ NOT a complete SIP server/client (parser/generator only)
- ⚠️ Requires building full async networking layer with Tokio
- ⚠️ Limited production usage examples
- ⚠️ Significant development effort (estimated 40-60 hours)

#### Mitigation Strategy

- Use SPARC TDD methodology to incrementally build async layer
- Phase 1: Parser/generator integration (16h)
- Phase 1: Transport layer with TLS (20h)
- Phase 2: Transaction layer (12h)
- Phase 3: Dialog layer (remaining hours in call flows)

#### Companion Library: rsip-dns

**Repository:** https://github.com/vasilakisfil/rsip-dns
**Purpose:** RFC 3263 DNS SRV lookups for SIP
**Async Support:** ✅ Yes (ResolvableExt trait)

---

### ❌ REJECTED: rvoip

**Repository:** https://lib.rs/crates/rvoip
**Version:** 0.1.26 (August 2025)
**License:** MIT

#### Why Rejected

- 🚫 **Alpha stage - NOT PRODUCTION READY**
- 🚫 Explicitly marked as unstable, APIs subject to change
- 🚫 Released July 2025 (too new, insufficient battle testing)
- 🚫 Known issues remain per documentation

#### Positive Aspects (Future Consideration)

- ✨ All-in-one solution: SIP + RTP + audio codecs + processing
- ✨ RFC 3261 compliant
- ✨ Multiple codecs (G.711, G.722, Opus, G.729)
- ✨ Advanced audio processing (echo cancellation, VAD)
- ✨ Simple API with excellent examples
- ✨ Built for async/await with Tokio

#### Recommendation

**Monitor for beta/stable release.** If rvoip reaches production-ready status during Phase 2-3, consider migration path. This would eliminate significant custom SIP stack work.

---

### ❌ REJECTED: pjsip-rs / pjproject-sys

**Type:** FFI bindings to PJSIP C library
**Crates:** Multiple fragmented/abandoned projects

#### Why Rejected

- 🚫 FFI to C library (not idiomatic Rust)
- 🚫 Requires unsafe code throughout
- 🚫 Poor async/Tokio integration
- 🚫 Complex C build system
- 🚫 Multiple competing/abandoned Rust binding projects
- 🚫 Goes against project's pure Rust preference
- 🚫 Limited Rust-specific documentation

#### Positive Aspects

- ✅ PJSIP is industry-standard, battle-tested
- ✅ Complete SIP/RTP/audio solution
- ✅ Extensive features and codec support

**Recommendation:** Only consider as last resort if pure Rust solutions prove inadequate after Phase 2.

---

## 2. Audio I/O Library

### ✅ APPROVED: cpal (Cross-Platform Audio Library)

**Repository:** https://github.com/RustAudio/cpal
**Stars:** 3,300+
**Contributors:** 177
**License:** Apache 2.0

#### Decision Criteria Scores

- **Documentation Quality:** ⭐⭐⭐⭐⭐ (Excellent)
- **Examples/Boilerplate:** ⭐⭐⭐⭐ (Good - multiple examples)
- **Battle-Tested:** ⭐⭐⭐⭐⭐ (Production-proven)
- **Async/Tokio Support:** ⭐⭐⭐ (Callback-based, requires bridge)
- **Type Safety:** ⭐⭐⭐⭐⭐ (Excellent)
- **Community:** ⭐⭐⭐⭐⭐ (RustAudio - very active)

#### Platform Support

- **macOS:** CoreAudio (native, full-featured)
- **Windows:** WASAPI (native), ASIO (optional for ultra-low latency)
- **Linux:** ALSA, JACK (future support)

#### Pros

- ✅ **Best choice** for cross-platform pure Rust audio
- ✅ Part of RustAudio community (excellent ecosystem)
- ✅ Battle-tested in production applications
- ✅ Stable API with clear upgrade path
- ✅ Pure Rust - no FFI complexity
- ✅ Comprehensive documentation and examples

#### Cons

- ⚠️ Callback-based API (requires async bridge to Tokio)
- ⚠️ Some platform-specific quirks (documented)

#### Integration Pattern

```rust
// Callback → Tokio bridge using channels
let (tx, rx) = tokio::sync::mpsc::channel(1024);

stream.play_with_callback(move |data| {
    // Audio callback (real-time thread)
    tx.try_send(data).ok();
});

// Tokio task receives audio data
tokio::spawn(async move {
    while let Some(data) = rx.recv().await {
        // Process in async context
    }
});
```

**Estimated Integration Effort:** 10-12 hours (well-documented pattern)

---

### ❌ REJECTED: coreaudio-rs

**Reason:** macOS-only, `cpal` provides better cross-platform abstraction while using CoreAudio internally on macOS.

---

## 3. RTP/Media Library

### ✅ APPROVED: webrtc-rs (RTP/RTCP Modules)

**Repository:** https://github.com/webrtc-rs/webrtc
**Version:** v0.14.0
**Stars:** 4,700+
**License:** MIT/Apache 2.0

#### Decision Criteria Scores

- **Documentation Quality:** ⭐⭐⭐⭐ (Good)
- **Examples/Boilerplate:** ⭐⭐⭐⭐ (Good - multiple examples)
- **Battle-Tested:** ⭐⭐⭐ (Moderate - Pion rewrite)
- **Async/Tokio Support:** ⭐⭐⭐⭐ (Good - built for async)
- **Type Safety:** ⭐⭐⭐⭐⭐ (Excellent)
- **Community:** ⭐⭐⭐⭐ (Good - active development)

#### Pros

- ✅ Most complete pure Rust RTP/RTCP implementation
- ✅ Modular design - can use RTP modules without full WebRTC stack
- ✅ Async/await support throughout
- ✅ Rewrite of production-proven Pion stack (Go)
- ✅ Active development and maintenance

#### Cons

- ⚠️ Early development stage (not explicitly production-ready)
- ⚠️ May require upstream bug fixes/contributions
- ⚠️ Some features still in development

#### Risk Mitigation

- Thorough testing in Phase 2-3
- Budget 8-12 hours for potential bug fixes
- Contribute fixes upstream if needed
- Fallback: Build minimal RTP stack (adds 40+ hours)

#### Usage

```rust
use webrtc_rtp as rtp;

// RTP session setup
let session = rtp::Session::new(...);
session.write_rtp(&packet).await?;
```

---

## 4. Secure Storage

### ✅ APPROVED: keyring Crate

**Repository:** https://github.com/hwchen/keyring-rs
**Crates.io:** keyring
**License:** MIT/Apache 2.0

#### Platform Support

- **macOS:** Keychain Services API
- **Windows:** Credential Manager API
- **Linux:** Secret Service API (future)

#### Pros

- ✅ Cross-platform with single API
- ✅ Uses platform-native secure storage
- ✅ Well-documented
- ✅ Active maintenance
- ✅ Simple API

#### Example

```rust
use keyring::Entry;

let entry = Entry::new("rustalk", "sip_password")?;
entry.set_password("secret123")?;
let password = entry.get_password()?;
```

**Estimated Integration Effort:** 6-8 hours

---

## 5. TLS Library

### ✅ APPROVED: rustls

**Repository:** https://github.com/rustls/rustls
**License:** Apache 2.0/ISC/MIT

#### Pros

- ✅ Pure Rust TLS implementation
- ✅ Memory-safe (no OpenSSL CVEs)
- ✅ Excellent async/Tokio integration
- ✅ Modern TLS 1.2/1.3 support
- ✅ Used in production (Cloudflare, etc.)

#### Cons

- ⚠️ Smaller ecosystem than OpenSSL

**Decision:** Prefer rustls for SIPS (SIP over TLS) unless compatibility issues arise.

---

## 6. Async Runtime

### ✅ APPROVED: Tokio

**Repository:** https://github.com/tokio-rs/tokio
**Reason:** Industry standard, best ecosystem, Tauri default

---

## Technology Stack Summary

### Final Approved Stack

```toml
# Cargo.toml dependencies (estimated)
[dependencies]
# Framework
tauri = "2"
tokio = { version = "1", features = ["full"] }

# SIP
rsip = "0.5"
# rsip-dns for DNS SRV

# Audio
cpal = "0.15"

# RTP/Media
webrtc-rtp = "0.14"
webrtc-sdp = "0.14"

# Security
keyring = "2"
rustls = "0.21"

# Utilities
serde = { version = "1", features = ["derive"] }
thiserror = "1"
async-trait = "0.1"
```

---

## Alternative Approach: WebRTC-Based

If SIP interoperability is **not** a hard requirement, consider:

### WebRTC Stack

- **Signaling:** Custom WebRTC signaling server (instead of SIP)
- **Media:** `webrtc-rs` (full stack)
- **Audio:** `cpal` (same)

#### Pros

- ✅ More modern protocol
- ✅ Better NAT traversal (ICE/STUN/TURN)
- ✅ More complete Rust stack available

#### Cons

- ❌ Not traditional VoIP/SIP
- ❌ May not work with existing SIP providers
- ❌ Requires custom signaling server

**Recommendation:** Stick with SIP for MVP (wider compatibility), consider WebRTC for future versions.

---

## Risk Assessment

### High Risk

| Risk                         | Impact         | Probability | Mitigation                                |
| ---------------------------- | -------------- | ----------- | ----------------------------------------- |
| rsip async layer too complex | Schedule delay | Medium      | SPARC TDD, phased approach, monitor rvoip |
| webrtc-rs bugs               | Quality issues | Medium      | Thorough testing, upstream contributions  |

### Medium Risk

| Risk                     | Impact         | Probability | Mitigation                                  |
| ------------------------ | -------------- | ----------- | ------------------------------------------- |
| cpal audio latency       | UX degradation | Low         | Low-level API, small buffers, jitter buffer |
| Platform-specific issues | Compatibility  | Medium      | Platform abstraction, test early on Windows |

### Low Risk

| Risk                 | Impact             | Probability | Mitigation              |
| -------------------- | ------------------ | ----------- | ----------------------- |
| keyring API changes  | Code churn         | Low         | Stable API, pin version |
| rustls compatibility | Integration issues | Very Low    | Well-tested with Tokio  |

---

## Future Considerations

### When rvoip Reaches Stability

- **Timeline:** Monitor for beta/v1.0 (likely 6-12 months)
- **Action:** Evaluate migration path from rsip → rvoip
- **Benefit:** Eliminate custom SIP stack maintenance

### Codec Support

- **MVP:** G.711 (simplest, widely supported)
- **Future:** Opus (high quality), G.722 (wideband)
- **Research:** Existing Rust codec crates vs. FFI

### Additional Libraries

| Component      | Library   | Status   |
| -------------- | --------- | -------- |
| DTMF           | TBD       | Post-MVP |
| Call recording | TBD       | Post-MVP |
| NAT traversal  | STUN/TURN | Post-MVP |

---

## Technology Stack Summary

**Status:** All technology decisions finalized and approved for implementation.

**Risk Acceptance:** The team accepts moderate risk with rsip custom async development, mitigated by incremental development methodology and phased implementation plan.

**Timeline Impact:** ~12 weeks for MVP with this stack (acceptable).
