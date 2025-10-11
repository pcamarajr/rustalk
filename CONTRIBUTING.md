# Contributing to RUSTALK

Thank you for your interest in contributing to RUSTALK! This guide will help you understand our development workflow, standards, and best practices.

## 🎯 Project Overview

RUSTALK is an AI-developed, open-source VoIP desktop application built with:
- **Backend**: Rust + Tauri
- **Frontend**: SvelteKit + TypeScript
- **Development**: AI agents with SPARC methodology
- **Testing**: TDD with 85%+ backend, 80%+ frontend coverage

See [RUSTALK.md](RUSTALK.md) for the product vision and [CLAUDE.md](CLAUDE.md) for AI development guidelines.

## 🚀 Getting Started

### Prerequisites

- **Rust**: 1.70 or later
- **Node.js**: 18.x or later
- **Git**: For version control
- **Platform**: macOS 11+ (primary), Windows 10+ (secondary), or Linux (development)

### Setup

```bash
# Clone the repository
git clone https://github.com/pcamarajr/rustalk.git
cd rustalk

# Install dependencies
npm install
cd src-tauri && cargo check && cd ..

# Run tests
cargo test
npm test

# Start development mode
npm run dev
```

## 📝 Development Workflow

### 1. Create a Feature Branch

```bash
# Always branch from main
git checkout main
git pull origin main

# Create feature branch
git checkout -b feature/your-feature-name
```

### 2. Follow SPARC Methodology

For significant features, use the SPARC workflow:

```bash
# Use RUSTALK custom command for full SPARC cycle
npx claude-flow@alpha command rustalk/feature-sparc --feature "Your Feature Name"
```

**SPARC Phases:**
1. **Specification**: Define requirements and acceptance criteria
2. **Pseudocode**: Design algorithm and logic flow
3. **Architecture**: Plan structure and dependencies
4. **Refinement**: Implement with TDD
5. **Completion**: Final testing and documentation

### 3. Write Tests First (TDD)

**Always write tests before implementation:**

```rust
// Rust example: src-tauri/src/domain/entities/credential_test.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credential_validation() {
        // Arrange
        let cred = Credential::new("user", "pass");

        // Act
        let result = cred.validate();

        // Assert
        assert!(result.is_ok());
    }
}
```

```typescript
// TypeScript example: src/lib/stores/auth.test.ts
import { describe, it, expect } from 'vitest'
import { authStore } from './auth'

describe('authStore', () => {
  it('should initialize with logged out state', () => {
    expect(authStore.isAuthenticated()).toBe(false)
  })
})
```

### 4. Follow Code Standards

**Rust:**
```bash
# Format code
cargo fmt

# Lint code
cargo clippy -- -D warnings

# Run tests
cargo test
```

**TypeScript/SvelteKit:**
```bash
# Format code
npm run format

# Lint code
npm run lint

# Run tests
npm test
```

### 5. Commit Conventions

We use [Conventional Commits](https://www.conventionalcommits.org/):

```bash
# Format: <type>(<scope>): <description>

# Examples:
feat(sip): add registration with TLS support
fix(audio): resolve microphone selection bug
docs(api): update credential storage documentation
test(integration): add end-to-end call flow test
chore(deps): update rsip to 0.5
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `test`: Adding or updating tests
- `refactor`: Code refactoring
- `chore`: Maintenance tasks
- `perf`: Performance improvements

See [.github/COMMIT_CONVENTION.md](.github/COMMIT_CONVENTION.md) for detailed guidelines.

## 📋 Pull Request Guidelines

### PR Checklist

Before submitting a PR, ensure:

- [ ] All tests pass (`cargo test` and `npm test`)
- [ ] Code is formatted (`cargo fmt` and `npm run format`)
- [ ] No linter warnings (`cargo clippy` and `npm run lint`)
- [ ] Test coverage meets targets (85% Rust, 80% frontend)
- [ ] Documentation is updated
- [ ] **Testing instructions are complete** (see below)

### Required: Testing Instructions

**Every PR MUST include comprehensive testing instructions.** Use our PR template which includes:

#### 1. Prerequisites
List all required tools with versions:
```markdown
**Required:**
- Rust 1.70+
- Node.js 18+
- [Any feature-specific requirements]
```

#### 2. Setup from Scratch (0 to 1)
Provide step-by-step setup for someone who has never run the project:
```markdown
1. Clone and checkout:
   ```bash
   git fetch origin pull/<PR_NUMBER>/head:pr-<PR_NUMBER>
   git checkout pr-<PR_NUMBER>
   ```

2. Install dependencies:
   ```bash
   npm install
   cargo check
   ```

3. Build the project:
   ```bash
   npm run dev
   ```
```

#### 3. How to Test Each Feature
Break down testing by feature with clear steps:
```markdown
#### Feature: SIP Registration
1. **Steps to test:**
   - Open the app
   - Navigate to Settings > Account
   - Enter SIP credentials (use test server: sip.example.com)
   - Click "Register"

2. **Expected result:**
   - Status changes to "Registered"
   - Green indicator appears
   - No error messages

3. **How to verify:**
   ```bash
   # Check logs for successful registration
   tail -f ~/.rustalk/logs/app.log | grep "SIP REGISTER"
   ```
```

#### 4. Running Automated Tests
```markdown
```bash
# Run Rust tests
cargo test

# Run frontend tests
npm test

# Run E2E tests
npm run test:e2e
```
```

#### 5. Expected Build Times
Help reviewers know if something is wrong:
```markdown
- Initial cargo build: ~2-3 minutes
- npm install: ~30-60 seconds
- Development mode startup: ~5-10 seconds
```

#### 6. Troubleshooting
Document common issues and solutions:
```markdown
**Issue:** cargo check fails with "linking error"
**Solution:** Install Xcode Command Line Tools: `xcode-select --install`
```

### PR Template

Our PR template (`.github/pull_request_template.md`) includes all required sections. **Do not skip sections** - they're there to ensure reviewers can test your changes.

### Automated PR Validation

We use GitHub Actions to automatically validate PRs:

- ✅ **Testing instructions check**: Ensures all required sections are present
- ✅ **Code quality**: Runs `cargo fmt`, `cargo clippy`, `npm run lint`
- ✅ **Build verification**: Ensures code compiles
- ✅ **Test execution**: Runs all test suites

PRs that fail validation will receive automated comments with guidance.

## 🧪 Testing Standards

### Coverage Targets

- **Rust backend**: 85%+ line coverage
- **Frontend**: 80%+ line coverage
- **E2E**: All critical user paths covered

### Test Types

1. **Unit Tests**: Test individual functions/components in isolation
2. **Integration Tests**: Test interactions between modules
3. **E2E Tests**: Test complete user workflows with Playwright

### Running Tests

```bash
# Rust unit tests
cargo test

# Rust tests with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html

# Frontend unit tests
npm test

# Frontend tests with coverage
npm test -- --coverage

# E2E tests
npm run test:e2e

# All tests
npm run test:all
```

## 🏗️ Architecture Guidelines

### Clean Architecture Principles

RUSTALK follows clean architecture with 5 layers:

1. **Domain Layer** (`src-tauri/src/domain/`)
   - Zero external dependencies
   - Pure business logic
   - Entities, events, domain traits

2. **Application Layer** (`src-tauri/src/services/`)
   - Orchestrates domain and infrastructure
   - Use case implementations

3. **IPC Boundary** (`src-tauri/src/commands/`)
   - Tauri commands (frontend ↔ backend)
   - Input validation

4. **Infrastructure Layer** (`src-tauri/src/infrastructure/`)
   - External integrations (SIP, audio, storage)
   - Implements domain traits

5. **Presentation Layer** (`src/`)
   - SvelteKit UI components
   - State management with stores

### File Organization

**Never save files to the root directory.** Use these directories:

- `/src-tauri` - Rust backend code
- `/src` - SvelteKit frontend code
- `/tests` - E2E tests (Playwright)
- `/docs` - Architecture and feature documentation
- `/scripts` - Build and automation scripts
- `/.github` - CI/CD workflows
- `/.claude` - AI agent definitions and commands

## 🔒 Security Guidelines

- **Never commit credentials** or secrets to the repository
- **Use platform keychain APIs** for secure storage (macOS Keychain, Windows Credential Manager)
- **Always use TLS/SIPS** for SIP communications
- **Validate all input** from the frontend in Tauri commands
- **Run `cargo audit`** regularly to check for dependency vulnerabilities

## 🤖 AI Agent Development

### Available Agents

**Core agents** (`.claude/agents/core/`):
- `coder`, `tester`, `reviewer`, `planner`, `researcher`

**RUSTALK-specific agents** (`.claude/agents/rustalk/`):
- `sip-specialist` - SIP protocol expert
- `tauri-engineer` - Tauri integration specialist
- `audio-engineer` - Audio systems specialist

### Custom Commands

Use RUSTALK workflows (`.claude/commands/rustalk/`):

```bash
# Full SPARC cycle for a feature
npx claude-flow@alpha command rustalk/feature-sparc --feature "Feature Name"

# Create PR linked to Linear issue
npx claude-flow@alpha command rustalk/linear-pr --issue "RUST-123"

# Platform-specific build
npx claude-flow@alpha command rustalk/platform-build
```

### Agent Coordination

Agents use MCP memory tools for coordination:

```javascript
// Store findings in shared memory
mcp__claude-flow__memory_usage({
  action: "store",
  namespace: "rustalk",
  key: "feature-status",
  value: JSON.stringify({ status: "in-progress", tests: "passing" })
})

// Retrieve context from other agents
mcp__claude-flow__memory_usage({
  action: "retrieve",
  namespace: "rustalk",
  key: "architecture-decisions"
})
```

## 📚 Documentation

### What to Document

- **Architecture decisions**: Add to `/docs/architecture/`
- **Feature specifications**: Follow SPARC format in `/docs/features/`
- **API changes**: Update relevant README files
- **Breaking changes**: Highlight in PR description

### Documentation Format

Use Markdown with clear structure:

```markdown
# Feature Name

## Overview
[Brief description]

## Architecture
[Design decisions]

## API
[Interface specifications]

## Testing
[How to test this feature]

## Examples
[Usage examples]
```

## 🔗 Linear Integration

We use Linear for issue tracking:

1. **Create Linear issue** before starting work
2. **Link PR to issue** in PR description: `Closes RUST-123`
3. **Use issue labels**: `epic`, `feature`, `bug`, `rustalk`, `macos`, `windows`

## ❓ Questions?

- **Architecture questions**: See `/docs/architecture/`
- **Development questions**: Check [CLAUDE.md](CLAUDE.md)
- **Bug reports**: Open a GitHub issue
- **Feature requests**: Create a Linear issue

## 🎉 Recognition

Contributors will be recognized in:
- Project README
- Release notes
- Linear issue acknowledgments

Thank you for contributing to RUSTALK! 🚀

---

**Remember**: This is an AI-developed project. We use autonomous agents, SPARC methodology, and swarm coordination for development. Your contributions should align with these principles.
