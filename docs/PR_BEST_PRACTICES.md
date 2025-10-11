# Pull Request Best Practices - RUSTALK

This guide provides best practices and examples for creating high-quality pull requests in the RUSTALK project.

## 🎯 Purpose

Pull requests should be:
- **Testable**: Anyone can verify changes without hunting for information
- **Complete**: All context and instructions provided
- **Professional**: Clear, concise, well-documented
- **Reviewable**: Appropriately sized and scoped

## ✅ The Perfect PR

### What Makes a Great PR?

1. **Clear Title**: Describes what changed in one line
2. **Comprehensive Description**: Explains why and how
3. **Complete Testing Instructions**: 0-to-1 setup guide
4. **Appropriate Scope**: Focused on one feature/fix
5. **High Test Coverage**: Meets 85%/80% targets
6. **Clean Commits**: Atomic, well-messaged commits
7. **Documentation**: Updated for changes

### Example: Excellent PR Title & Description

**Bad Title:**
```
fix stuff
```

**Good Title:**
```
feat(sip): add TLS support for secure registration
```

**Excellent Description:**
```markdown
## Summary
Implements TLS encryption for SIP registration to meet security requirements
defined in SEC-6.4 (see /docs/architecture/05-implementation-roadmap.md).

## Problem
Current SIP registration sends credentials in plaintext, violating our
security guidelines and making it unsuitable for production use.

## Solution
- Added rustls-based TLS transport for SIP connections
- Implemented certificate validation with custom CA support
- Updated SipClient trait to support both TCP and TLS transports
- Added comprehensive tests for TLS handshake and certificate validation

## Technical Details
- Uses rustls 0.23 with tokio-rustls for async TLS
- Supports SIPS URI scheme (sips:user@domain.com)
- Falls back to TCP for development mode (configurable)
- Certificate pinning available for enhanced security

## Breaking Changes
None - TLS support is additive and enabled via SIPS URI scheme.
```

## 📋 Testing Instructions Template

### The Gold Standard

Use this template for **every PR**:

```markdown
## 🧪 Testing Instructions

### Prerequisites

**Required:**
- Tool 1: Version X.Y+ (check: `tool --version`)
- Tool 2: Version A.B+ (check: `tool --version`)
- Platform: macOS 11+ / Windows 10+

**Optional:**
- Tool for advanced testing (if applicable)

### Setup from Scratch (0 to 1)

1. **Clone and checkout:**
   ```bash
   git fetch origin pull/<PR_NUMBER>/head:pr-<BRANCH_NAME>
   git checkout pr-<BRANCH_NAME>
   ```

2. **Install dependencies:**
   ```bash
   npm install
   cd src-tauri && cargo check && cd ..
   ```

3. **Configure for testing** (if needed):
   ```bash
   # Any special configuration steps
   cp .env.example .env
   # Edit .env and set TEST_SIP_SERVER=sip.example.com
   ```

4. **Build:**
   ```bash
   npm run dev  # or npm run tauri:build
   ```

### How to Test

#### Feature 1: [Specific Feature Name]

**What to test:** [Brief description of what this feature does]

**Steps:**
1. Open the application
2. Navigate to [specific location]
3. Perform [specific action]
4. Observe [expected behavior]

**Expected result:**
- [Specific observable outcome 1]
- [Specific observable outcome 2]
- [What should NOT happen]

**How to verify:**
```bash
# Commands to verify the feature works
tail -f ~/.rustalk/logs/app.log | grep "SUCCESS"
```

**Screenshot/Video:**
[Include if visual changes]

#### Feature 2: [Next Feature]
[Repeat pattern above]

### Running Automated Tests

```bash
# All tests
npm run test:all

# Specific test suites
cargo test sip::tls  # Test TLS functionality
npm test -- --grep "auth"  # Test auth components
```

**Expected test output:**
```
running 12 tests
test sip::tls::test_handshake ... ok
test sip::tls::test_cert_validation ... ok
...
test result: ok. 12 passed; 0 failed; 0 ignored
```

### Expected Build Times

- Initial cargo build: ~X minutes
- npm install: ~X seconds
- Test execution: ~X seconds

### Troubleshooting

**Issue:** [Specific error message or problem]
**Solution:**
```bash
# Exact commands to fix
cargo clean && cargo build
```

**Issue:** [Another common problem]
**Solution:** [Step-by-step fix]
```

## 🔍 Review Checklist

Before requesting review, verify:

### Code Quality
- [ ] All tests pass locally
- [ ] Code formatted (`cargo fmt`, `npm run format`)
- [ ] No linter warnings (`cargo clippy`, `npm run lint`)
- [ ] No commented-out code (unless explicitly explained)
- [ ] No TODO comments (create issues instead)

### Testing
- [ ] Test coverage meets targets (85% Rust, 80% frontend)
- [ ] Edge cases covered
- [ ] Error paths tested
- [ ] Integration tests for cross-layer functionality

### Documentation
- [ ] Code comments explain "why", not "what"
- [ ] Complex algorithms documented
- [ ] Public API documented
- [ ] Architecture docs updated (if applicable)

### Testing Instructions
- [ ] Prerequisites listed with versions
- [ ] 0-to-1 setup instructions complete
- [ ] Each feature has testing steps
- [ ] Expected results clearly stated
- [ ] Automated test commands provided
- [ ] Build time expectations set
- [ ] Troubleshooting section filled

### Security
- [ ] No hardcoded credentials
- [ ] Input validation for all user input
- [ ] No SQL injection vulnerabilities
- [ ] Secure storage for sensitive data
- [ ] TLS/encryption where required

## 📏 PR Size Guidelines

### Ideal PR Size

**Lines changed**: 200-400 lines (sweet spot for thorough review)
**Maximum recommended**: 800 lines
**Absolute maximum**: 1000 lines (split into multiple PRs if possible)

### When to Split a PR

Split large PRs into smaller ones when:
- Implementing multiple independent features
- Refactoring + adding features (separate refactor PR first)
- Changes span multiple architectural layers
- Review would take >1 hour

### Example of Good Splitting

**Bad (one massive PR):**
```
feat: add complete SIP + audio + UI
- 2500 lines changed
- Hard to review
- High risk of bugs
```

**Good (split into logical chunks):**
```
PR #1: feat(sip): add SIP client trait and basic registration (300 lines)
PR #2: feat(sip): add TLS support for SIP (250 lines)
PR #3: feat(audio): implement audio device selection (400 lines)
PR #4: feat(ui): add registration UI components (350 lines)
PR #5: feat(integration): wire up SIP + audio + UI (200 lines)
```

## 🎨 Commit Message Excellence

### Format

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Examples

**Simple feature:**
```
feat(sip): add TLS certificate validation
```

**With body:**
```
feat(sip): add TLS certificate validation

Implements custom CA certificate support and certificate pinning
for enhanced security. Uses rustls for memory-safe TLS.

Closes RUST-45
```

**Breaking change:**
```
feat(api)!: change SipConfig structure

BREAKING CHANGE: SipConfig now requires separate tls field instead
of auto-detecting from URI. Update all SipConfig instantiations to
include explicit tls: true/false.

Migration:
- Old: SipConfig { uri: "sips:..." }
- New: SipConfig { uri: "sips:...", tls: true }
```

### Commit Best Practices

1. **One logical change per commit**: Don't mix refactoring with features
2. **Test after each commit**: Each commit should pass all tests
3. **Write clear messages**: Future you will thank you
4. **Reference issues**: Use `Closes #123` or `Fixes #456`
5. **Explain "why" in body**: Not "what" (code shows that)

## 🔄 Review Process

### Responding to Feedback

**When reviewer requests changes:**

1. **Acknowledge**: Thank reviewer for feedback
2. **Ask for clarification**: If feedback is unclear
3. **Make changes**: Address all feedback points
4. **Comment on resolution**: Explain what you changed
5. **Re-request review**: When ready

**Example response:**
```markdown
@reviewer Thanks for the thorough review!

✅ Fixed TLS handshake timeout issue (increased to 10s)
✅ Added integration test for certificate validation
✅ Updated documentation with CA certificate setup

📝 Regarding the connection pooling suggestion: I've created issue #123
to track this enhancement separately, as it's a larger refactor that
deserves its own PR.

Ready for re-review!
```

### When to Merge

Merge when:
- ✅ All reviewers approved
- ✅ All CI checks passing
- ✅ No unresolved conversations
- ✅ Testing instructions verified by reviewer
- ✅ Documentation complete

**Don't merge if:**
- ❌ Any tests failing
- ❌ Unresolved review comments
- ❌ CI checks not complete
- ❌ Merge conflicts present

## 📊 Examples from RUSTALK

### Example 1: Infrastructure Setup (PR #1)

**What makes it good:**
- ✅ Comprehensive testing instructions (0-to-1 setup)
- ✅ Clear expected build times
- ✅ Troubleshooting section with solutions
- ✅ Links to relevant documentation
- ✅ Atomic commits with clear messages

**Testing instructions excerpt:**
```markdown
### Prerequisites
- Rust 1.70+ (check: `rustc --version`)
- Node.js 18+ (check: `node --version`)

### Setup from Scratch
1. Clone and checkout...
2. Install dependencies...
3. Build the project...

### Expected Build Times
- Initial cargo check: ~2-3 minutes
- npm install: ~30-60 seconds

### Troubleshooting
**Issue:** cargo check fails with network errors
**Solution:** Try `cargo update` first...
```

### Example 2: Feature Implementation

**Good structure:**
```markdown
## Summary
Implements SIP registration with secure credential storage per
Phase 1 requirements (SEC-6.2).

## Changes
- CredentialStore trait for platform abstraction
- macOS Keychain integration via keyring crate
- SipClient uses stored credentials for registration
- UI components for credential management

## Testing Instructions

### Prerequisites
- macOS 11+ (Keychain integration requires macOS)
- Test SIP server credentials (or use provided test account)

### Feature 1: Credential Storage
**Steps:**
1. Open Settings > Account
2. Enter SIP credentials:
   - Server: sip.test.rustalk.dev
   - Username: test@example.com
   - Password: testpass123
3. Click "Save Credentials"

**Expected:**
- Success message appears
- Credentials stored in Keychain
- Password not visible in app logs

**Verify:**
```bash
# Check Keychain for stored credential
security find-internet-password -s "rustalk"
# Should show entry without revealing password
```

### Feature 2: SIP Registration
...
```

## 🚫 Common Mistakes to Avoid

### Mistake 1: Incomplete Testing Instructions

**Bad:**
```markdown
## Testing
Run the app and test the feature.
```

**Good:**
```markdown
## Testing Instructions

### Prerequisites
- Rust 1.70+
- Test SIP server access

### Setup from Scratch
[Detailed steps...]

### How to Test
[Step-by-step for each feature...]
```

### Mistake 2: Vague Descriptions

**Bad:**
```markdown
## Description
Updated SIP code to fix some issues.
```

**Good:**
```markdown
## Description
Fixed SIP registration timeout when server takes >5s to respond.

**Root Cause:** Registration timeout was hardcoded to 3s, causing
failures with slow servers.

**Solution:** Made timeout configurable (default: 10s) and added
retry logic with exponential backoff.
```

### Mistake 3: Missing Prerequisites

**Bad:**
```markdown
1. Run `cargo test`
```
*(Fails if dependencies not installed)*

**Good:**
```markdown
### Prerequisites
- Rust 1.70+ installed
- ALSA dev libraries (Linux): `apt-get install libasound2-dev`

### Steps
1. Install dependencies: `cargo check`
2. Run tests: `cargo test`
```

### Mistake 4: No Troubleshooting

**Bad:**
*(No troubleshooting section at all)*

**Good:**
```markdown
### Troubleshooting

**Issue:** Tests fail with "connection refused"
**Solution:** Ensure test SIP server is running:
```bash
docker-compose up test-sip-server
```

**Issue:** Build fails with "linking error"
**Solution:** Install platform dependencies:
- macOS: `xcode-select --install`
- Linux: `apt-get install build-essential`
```

## 🎓 Learning Resources

### Internal Resources
- [CONTRIBUTING.md](../CONTRIBUTING.md) - Full contribution guide
- [CLAUDE.md](../CLAUDE.md) - AI agent development guidelines
- [Architecture Docs](../docs/architecture/) - System design
- [PR Template](../.github/pull_request_template.md) - Use this for all PRs

### External Resources
- [Conventional Commits](https://www.conventionalcommits.org/)
- [How to Write a Git Commit Message](https://chris.beams.io/posts/git-commit/)
- [The Art of the Pull Request](https://hackernoon.com/the-art-of-pull-requests-6f0f099850f9)

## 🤖 Using AI Agents for PR Reviews

### Automated PR Review

Run multi-agent PR review swarm:

```bash
npx claude-flow@alpha command rustalk/pr-review --pr <PR_NUMBER>
```

This deploys specialized agents:
- **Testing Validator**: Checks testing instruction completeness
- **Security Agent**: Scans for vulnerabilities
- **Architecture Agent**: Verifies clean architecture compliance
- **Production Validator**: Ensures deployment readiness
- **Documentation Agent**: Validates documentation quality

### Manual Validation

Before requesting review, validate your PR locally:

```bash
# Check testing instructions completeness
.claude/scripts/validate-pr-testing.sh <PR_NUMBER>

# This will verify all required sections are present:
# ✅ Prerequisites
# ✅ Setup from Scratch
# ✅ How to Test
# ✅ Running Automated Tests
# ✅ Expected Build Times
# ✅ Troubleshooting
```

## 📝 Quick Reference

### PR Submission Checklist

```markdown
- [ ] Title follows conventional commits format
- [ ] Description explains why and how
- [ ] Testing instructions complete (all 6 sections)
- [ ] Prerequisites listed with versions
- [ ] 0-to-1 setup steps documented
- [ ] Each feature has testing steps
- [ ] Expected results documented
- [ ] Automated tests documented
- [ ] Build times documented
- [ ] Troubleshooting section complete
- [ ] All tests passing
- [ ] Code formatted and linted
- [ ] Coverage targets met
- [ ] Documentation updated
- [ ] Commits are atomic and well-messaged
- [ ] PR size reasonable (<800 lines)
```

---

**Remember**: Great PRs make great codebases. Take the time to write clear, complete, testable pull requests and you'll save everyone time in the long run!

🚀 Happy coding!
