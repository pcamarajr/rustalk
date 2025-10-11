# RUSTALK - AI-Developed VoIP Desktop Application

## 🎯 Project Overview

**RUSTALK** is an open-source, white-label VoIP desktop application built entirely by autonomous AI agents.

**Tech Stack:**

- **Backend**: Rust + Tauri (desktop framework)
- **Frontend**: SvelteKit + TypeScript
- **Platforms**: macOS (primary), Windows (secondary), Linux (future)
- **Development**: AI agent teams with SPARC methodology

**See `RUSTALK.md` for full product vision.**

---

## 📁 File Organization

**NEVER save to root folder. Use these directories:**

- `/src-tauri` - Rust backend code (SIP, audio, Tauri commands)
- `/src` - SvelteKit frontend code
- `/tests` - E2E tests (Playwright)
- `/docs` - Architecture and feature documentation
- `/scripts` - Build and automation scripts
- `/.github` - CI/CD workflows
- `/.claude` - Agent definitions and commands

---

## 🏗️ Tech Stack Specifics

### Rust Backend (`/src-tauri`)

- **Framework**: Tauri v1.x
- **SIP**: Agent-researched library (prefer well-documented, async-ready)
- **Audio**: Platform-specific (CoreAudio/macOS, WASAPI/Windows)
- **Async**: Tokio runtime
- **Storage**: Keychain (macOS), Credential Manager (Windows)

### SvelteKit Frontend (`/src`)

- **Framework**: SvelteKit (file-based routing, minimal boilerplate)
- **Language**: TypeScript (strict mode)
- **Styling**: TailwindCSS or utility-first CSS framework
- **State**: Svelte stores (built-in)
- **IPC**: Tauri invoke API

### Testing Strategy

- **Rust**: `cargo test` + `cargo-nextest` (85%+ coverage)
- **Frontend**: Vitest unit tests (80%+ coverage)
- **E2E**: Playwright (macOS + Windows)
- **CI/CD**: GitHub Actions

---

## 🧪 Test-Driven Development

**Always test-first:**

1. Write failing test
2. Implement minimum code to pass
3. Refactor for quality
4. Achieve coverage targets

**Coverage Requirements:**

- Rust backend: 85%+
- SvelteKit frontend: 80%+
- E2E: All critical user paths

---

## 📚 MVP Feature Scope

### Must-Have (Blocker)

1. SIP registration with credentials
2. Outbound calls (dial + initiate)
3. Inbound calls (receive + answer)
4. Call controls (answer, hangup, mute)
5. Audio device selection
6. Secure credential storage

### Post-MVP

- Contact list
- Call history
- DTMF support
- Call transfer

---

## 🔐 Security Guidelines

- **Credentials**: Use platform keychain APIs, never plaintext
- **SIP**: TLS/SIPS only, validate certificates
- **Input**: Validate all Tauri commands
- **Secrets**: Never hardcode API keys or passwords
- **Audit**: Run `cargo audit` regularly

---

## 🤖 AI Agent Development

### Available Agents

**Core agents** are in `.claude/agents/core/`:

- `coder`, `tester`, `reviewer`, `planner`, `researcher`

**RUSTALK-specific agents** are in `.claude/agents/rustalk/`:

- `sip-specialist` - SIP protocol expert
- `tauri-engineer` - Tauri integration specialist
- `audio-engineer` - Audio systems specialist

**See `.claude/agents/` for full list of 54+ available agents.**

### Custom Commands

**RUSTALK workflows** are in `.claude/commands/rustalk/`:

- `feature-sparc` - Full SPARC cycle for a feature
- `linear-pr` - Create PR linked to Linear issue
- `platform-build` - macOS/Windows build automation

**Use commands via:** `npx claude-flow@alpha command <command-name>`

---

## 🔄 Development Workflow

### Phase 1: Initial Planning

Run once for full project architecture:

```bash
npx claude-flow sparc run specification "RUSTALK MVP architecture"
npx claude-flow sparc run architecture "Rust + Tauri + SvelteKit design"
```

### Phase 2: Feature Development

For each feature, run full SPARC cycle:

```bash
# Use custom RUSTALK command
npx claude-flow@alpha command rustalk/feature-sparc --feature "SIP Registration"

# Or manual SPARC
npx claude-flow sparc tdd "SIP Registration feature"
```

### Phase 3: Linear + GitHub Integration

```bash
# Create PR linked to Linear issue (semi-automated)
npx claude-flow@alpha command rustalk/linear-pr --issue "RUST-123"
```

---

## 🚀 Platform Builds

### macOS (Primary)

- **Target**: macOS 11+, Universal Binary (Intel + Apple Silicon)
- **Signing**: Apple Developer ID required
- **Distribution**: DMG with notarization

### Windows (Secondary)

- **Target**: Windows 10+, x64
- **Installer**: NSIS (Tauri default)
- **Distribution**: GitHub Releases

### Commands

```bash
# Development
pnpm dev              # Run SvelteKit + Tauri dev mode

# Testing
cargo nextest run     # Rust tests (faster)
pnpm test             # SvelteKit tests
pnpm test:e2e         # Playwright E2E

# Production
cargo tauri build     # Platform-specific build
```

---

## 🎯 Decision Criteria for Agents

When choosing libraries/approaches, prioritize:

1. **Documentation quality** - Well-documented = higher success
2. **Boilerplate/examples** - More examples = easier implementation
3. **Battle-tested** - Production-proven over cutting-edge
4. **Async/await support** - Must work with Tokio
5. **Type safety** - Strong typing reduces bugs
6. **Community** - Active community = more support

**Example**: For SIP library, prefer `rsip` (pure Rust, well-documented, async) over FFI bindings.

---

## 🔗 Linear + GitHub Integration

### Linear Issues

- **Epic format**: `[EPIC] Feature Area` (e.g., `[EPIC] SIP Integration`)
- **Feature format**: `Feature Name` with SPARC-formatted description
- **Labels**: `epic`, `feature`, `rustalk`, `macos`, `windows`

### GitHub PRs (Semi-Automated)

- Agents create **draft PRs** via `rustalk/linear-pr` command
- PRs auto-link to Linear: `Closes RUST-123`
- Human review and merge required
- Linear auto-closes on merge (via GitHub integration)

---

## 📋 Code Style

- **Rust**: Follow `rustfmt` + `clippy` suggestions
- **TypeScript**: ESLint + Prettier
- **Files**: Keep under 500 lines
- **Naming**: Clear, descriptive names
- **Comments**: Explain "why", not "what"

---

## 🚨 Critical Rules (Enforced by Hooks)

1. **Concurrent execution**: Batch all operations in single messages
2. **No root files**: Always organize in subdirectories
3. **Test-first**: Write tests before implementation
4. **Memory coordination**: Use MCP memory tools for agent coordination
5. **Hooks**: All agents must run pre/post hooks via claude-flow

**See `.claude/settings.json` for hook configurations.**

---

## 📖 Documentation

- **Architecture**: `/docs/architecture/` - System design decisions
- **Features**: `/docs/features/` - SPARC specifications
- **API**: Auto-generated from code (`cargo doc`, SvelteDoc)
- **White-label**: `/docs/white-label/` - Branding guide

---

## 🔧 Quick Start for Agents

When working on RUSTALK:

1. **Check context**: Read `RUSTALK.md` and relevant `/docs`
2. **Use RUSTALK agents**: Prefer `.claude/agents/rustalk/*` specialists
3. **Run SPARC**: Use `rustalk/feature-sparc` command for features
4. **Batch operations**: Always use single message for parallel work
5. **Coordinate**: Use MCP memory tools to share context
6. **Link to Linear**: Create PRs via `rustalk/linear-pr` command

---

## 📞 Resources

- **Vision**: See `RUSTALK.md` for product goals
- **Agents**: See `.claude/agents/` for all available agents
- **Commands**: See `.claude/commands/rustalk/` for workflows
- **Tauri Docs**: https://tauri.app/v1/guides/
- **SvelteKit Docs**: https://kit.svelte.dev/docs
- **Claude Flow**: https://github.com/ruvnet/claude-flow

---

**Remember**: This is an AI-developed project. Autonomous agents build everything using SPARC methodology and swarm coordination.

---

# important-instruction-reminders

- Do what has been asked; nothing more, nothing less
- NEVER create files unless absolutely necessary
- ALWAYS prefer editing existing files
- NEVER proactively create documentation files
- Never save working files to root folder
- Follow RUSTALK tech stack (Rust + Tauri + SvelteKit)
- Always use test-first development (TDD)
