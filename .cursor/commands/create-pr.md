# Create Pull Request

## Overview

This command validates code and documentation, then creates a comprehensive pull request with a detailed description including summary, testing instructions, and review checklists. The command blocks PR creation if validation fails.

**Usage**: `/create-pr` or `/create-pr <base-branch>` (default: `main`)

**Prerequisites**:

- GitHub CLI (`gh`) must be installed and authenticated
- All changes must be committed (use `/commit` first if needed)

---

## Execution Flow

### Step 1: Check Prerequisites

#### 1.1: Check GitHub CLI Installation

```bash
if ! command -v gh &> /dev/null; then
  echo "❌ GitHub CLI (gh) is not installed."
  echo "Please install it first:"
  echo "  macOS: brew install gh"
  echo "  Or visit: https://cli.github.com/"
  exit 1
fi
```

**If GitHub CLI is not installed**: Notify the user and **STOP** immediately.

#### 1.2: Verify GitHub CLI Authentication

```bash
if ! gh auth status &> /dev/null; then
  echo "❌ GitHub CLI is not authenticated."
  echo "Please run: gh auth login"
  exit 1
fi
```

**If not authenticated**: Notify the user and **STOP** immediately.

#### 1.3: Check Git Repository Status

```bash
# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
  echo "❌ Not in a git repository."
  exit 1
fi

# Check if there are uncommitted changes
if ! git diff-index --quiet HEAD --; then
  echo "⚠️  Warning: You have uncommitted changes."
  echo "Please commit your changes first using '/commit' or 'git commit'"
  exit 1
fi

# Get current branch name (always use active branch)
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" = "main" ] || [ "$CURRENT_BRANCH" = "master" ]; then
  echo "ℹ️  You are on the main branch. There's nothing to push."
  exit 0
fi
```

**If issues found**: Notify the user and **STOP** immediately.

### Step 2: Determine Base Branch

1. **If base branch provided** (e.g., `/create-pr develop`):

   - Use the provided branch: `develop`
   - Validate it exists: `git branch --list develop origin/develop`

2. **If no base branch provided**:
   - Default to `main`
   - Validate it exists: `git branch --list main origin/main`

**If base branch doesn't exist**: Notify the user and **STOP**.

### Step 3: Code Validation

**CRITICAL**: All validation checks must pass. If any check fails, block PR creation.

#### 3.1: Detect Changed Files

```bash
# Get list of changed files compared to base branch
git diff --name-only origin/$BASE_BRANCH...HEAD > /tmp/changed-files.txt

# Categorize changes
RUST_CHANGED=$(grep -E '\.rs$' /tmp/changed-files.txt | wc -l)
TS_CHANGED=$(grep -E '\.(ts|tsx|js|svelte)$' /tmp/changed-files.txt | wc -l)
DOCS_ONLY=$(grep -vE '\.(rs|ts|tsx|js|svelte)$' /tmp/changed-files.txt | grep -E '\.md$' | wc -l)
```

#### 3.2: Compilation Checks

**For Rust changes** (if any `.rs` files changed):

```bash
cd src-tauri && cargo check
```

- Capture exit code and output
- If exit code != 0: **BLOCK PR CREATION**
- Display compilation errors clearly

**For TypeScript/Svelte changes** (if any `.ts`, `.tsx`, `.js`, `.svelte` files changed):

```bash
npm run check
```

- Capture exit code and output
- If exit code != 0: **BLOCK PR CREATION**
- Display compilation errors clearly

**Execution strategy**:

- **If both Rust and TypeScript changed**: Run checks in parallel
- **If only one changed**: Run that check sequentially
- **If only documentation changed**: Skip compilation (but still run other validations)

#### 3.3: Linting Checks

**For Rust changes**:

```bash
cd src-tauri
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

- If formatting fails: **BLOCK PR CREATION**
- If clippy finds warnings: **BLOCK PR CREATION**

**For TypeScript/Svelte changes**:

```bash
npm run lint
```

- If linting fails: **BLOCK PR CREATION**

#### 3.4: Test Execution

**For Rust changes**:

```bash
cd src-tauri && cargo test
```

- Capture exit code and output
- If tests fail: **BLOCK PR CREATION**

**For TypeScript/JavaScript changes**:

```bash
npm test
```

- Capture exit code and output
- If tests fail: **BLOCK PR CREATION**
- **Note**: If `npm test` is not implemented (returns 0 with message), skip this check but note it in PR description

#### 3.5: Validation Summary

Display validation results:

```
✅ Compilation: Passed
✅ Linting: Passed
✅ Tests: Passed
```

If any check fails:

```
❌ Validation Failed
❌ Compilation: Failed (Rust)
   Error: [error details]

PR creation blocked. Please fix the issues above and try again.
```

**STOP** and do not proceed with PR creation.

### Step 4: Documentation Validation

#### 4.1: Check README.md Updates

Analyze changes to determine if README.md should be updated:

1. **Extract PR context**:

   - Get commit messages: `git log origin/$BASE_BRANCH..HEAD --oneline`
   - Get changed files: `git diff --name-only origin/$BASE_BRANCH...HEAD`
   - Analyze commit messages for keywords:
     - Phase completion: "phase", "complete", "finished"
     - New features: "feat", "add", "implement"
     - Status changes: "status", "update status"
     - Architecture changes: "architecture", "refactor"

2. **Check if README.md was modified**:

   ```bash
   git diff --name-only origin/$BASE_BRANCH...HEAD | grep -q README.md
   ```

3. **If README.md was NOT modified but changes suggest it should be**:

   - Check if commits mention phase completion, new features, or status changes
   - If yes: **BLOCK PR CREATION** with message:

     ```
     ❌ Documentation Validation Failed

     Your changes appear to affect information documented in README.md, but README.md was not updated.

     Please update README.md to reflect:
     - [Specific changes that require README update]

     PR creation blocked. Please update README.md and try again.
     ```

4. **If README.md was modified**: Validate it's comprehensive:
   - Check if relevant sections were updated
   - If changes are minimal or incomplete: **WARN** (but don't block)

#### 4.2: Check Documentation Files

Check if changes to code affect documentation in `docs/`:

- If architecture changes: Check if `docs/architecture/` needs updates
- If new features: Check if relevant docs need updates
- **Note**: This is a warning, not a blocker (unless critical)

### Step 5: Push Branch to Remote

```bash
# Check if branch is already pushed
if ! git rev-parse --verify origin/$CURRENT_BRANCH > /dev/null 2>&1; then
  echo "📤 Pushing branch to remote..."
  git push -u origin $CURRENT_BRANCH
else
  # Check if local is ahead of remote
  LOCAL=$(git rev-parse @)
  REMOTE=$(git rev-parse @{u} 2>/dev/null || echo "")
  if [ "$LOCAL" != "$REMOTE" ]; then
    echo "📤 Pushing latest commits to remote..."
    git push
  else
    echo "✅ Branch is up to date on remote"
  fi
fi
```

**If push fails**: Notify the user and **STOP**.

### Step 6: Extract PR Information

#### 6.1: Code Diff Summary

Analyze the diff to extract:

- Key changes (new features, bug fixes, refactoring)
- Breaking changes (API changes, config changes)
- Performance improvements
- Security updates
- Related issues/tickets (from commit messages like "DX-123", "#123", "fixes #123")

#### 6.2: Testing Instructions

Extract testing information from:

1. **Code comments**: Look for `@test`, `@testing`, `TEST:` comments
2. **Test files**: Check if new test files were added
3. **Feature type**: Generate testing steps based on feature type
4. **Changed components**: Identify which UI components changed for manual testing

### Step 7: Generate PR Description

Create a comprehensive PR description with the following structure:

````markdown
## Summary

[Concise 2-3 sentence summary of what this PR does]

## Changes

- [Bullet point 1: Main feature/fix]
- [Bullet point 2: Secondary changes]
- [Bullet point 3: Additional improvements]

## Testing

### How to Test

1. **Setup**:
   ```bash
   git checkout <branch-name>
   npm install
   ```
````

2. **Test Steps**:

   - [Step 1: Specific testing instruction]
   - [Step 2: Specific testing instruction]
   - [Step 3: Specific testing instruction]

3. **Expected Behavior**:
   - [What should happen]
   - [What should NOT happen]

### Test Coverage

- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing completed
- [ ] Edge cases tested

## Review Checklist

### Code Quality

- [ ] Code follows project conventions
- [ ] Functions are focused and single-purpose
- [ ] Variable names are descriptive
- [ ] No code duplication
- [ ] Error handling is appropriate

### Framework Compliance

- [ ] Svelte 5 runes mode (no legacy syntax)
- [ ] Rust code follows idiomatic patterns
- [ ] Tauri IPC follows best practices
- [ ] Design system components used correctly

### Security

- [ ] No hardcoded secrets
- [ ] Input validation present
- [ ] Sensitive data handled securely
- [ ] Tauri capabilities properly configured

### Documentation

- [ ] README.md updated (if applicable)
- [ ] Code comments explain complex logic
- [ ] Architecture docs updated (if applicable)

## Breaking Changes

[If any breaking changes detected, list them here. Otherwise: "None"]

## Related Issues

[If issues/tickets found in commit messages, list them here. Otherwise: "None"]

## Additional Notes

[Any additional context, known limitations, or follow-up actions]

````

### Step 8: Create Pull Request

```bash
# Generate PR title from first commit or branch name
PR_TITLE=$(git log origin/$BASE_BRANCH..HEAD --pretty=format:"%s" | head -n 1 | sed 's/^feat(/feat: /; s/^fix(/fix: /; s/^docs(/docs: /')

# If title is too long, use branch name
if [ ${#PR_TITLE} -gt 72 ]; then
  PR_TITLE=$(echo $CURRENT_BRANCH | sed 's/feat\///; s/fix\///; s/docs\///' | sed 's/-/ /g' | awk '{for(i=1;i<=NF;i++)sub(/./,toupper(substr($i,1,1)),$i)}1')
fi

# Create PR
gh pr create \
  --base $BASE_BRANCH \
  --head $CURRENT_BRANCH \
  --title "$PR_TITLE" \
  --body-file /tmp/pr-description.md
````

**If PR creation fails**: Display error and **STOP**.

### Step 9: Display Success Message

```
✅ Pull Request Created Successfully!

🔗 PR URL: [GitHub PR URL]
📋 Title: [PR Title]
🌿 Branch: [branch-name] → [base-branch]

📝 Next Steps:
- Review the PR description
- Add reviewers if needed
- Monitor CI/CD status
```

---

## Validation Rules Summary

### Blockers (PR Creation Blocked)

1. **GitHub CLI not installed or authenticated**
2. **Uncommitted changes present**
3. **Compilation failures** (Rust or TypeScript)
4. **Linting failures** (Rust or TypeScript)
5. **Test failures** (Rust or TypeScript)
6. **README.md not updated when required** (based on change analysis)

### Warnings (Displayed but Don't Block)

1. **Documentation files may need updates**
2. **Test coverage could be improved**
3. **Breaking changes detected** (listed in PR description)

---

## Implementation Notes

- **Active branch**: Always uses the current active branch
- **Parallel execution**: Run Rust and TypeScript checks in parallel when both changed
- **Smart detection**: Only run checks for changed file types
- **Documentation analysis**: Use commit messages and file changes to determine if README needs updates
- **PR description**: Auto-generate concise description from code analysis (no redundant commit/file lists)
- **Error handling**: Clear error messages with actionable guidance
- **GitHub CLI integration**: Uses `gh` for all GitHub operations

---

## Examples

**Successful PR creation**:

```
/create-pr
→ 🔍 Validating code...
→ ✅ Compilation: Passed (Rust, TypeScript)
→ ✅ Linting: Passed
→ ✅ Tests: Passed
→ 📝 Validating documentation...
→ ✅ Documentation: OK
→ 📤 Pushing branch to remote...
→ 📝 Generating PR description...
→ ✅ Pull Request Created Successfully!
→ 🔗 PR URL: https://github.com/user/repo/pull/123
```

**Validation failure** (blocked):

```
/create-pr
→ 🔍 Validating code...
→ ✅ Compilation: Passed
→ ❌ Linting: Failed (Rust)
   Error: src-tauri/src/main.rs:42:5 - warning: unused variable
→ ❌ PR creation blocked. Please fix linting errors and try again.
```

**Documentation validation failure** (blocked):

```
/create-pr
→ 🔍 Validating code...
→ ✅ All code checks passed
→ 📝 Validating documentation...
→ ❌ Documentation Validation Failed
   Your changes complete Phase 2, but README.md was not updated.
   Please update README.md to reflect Phase 2 completion.
→ ❌ PR creation blocked. Please update README.md and try again.
```

---

## Error Handling

If any step fails:

1. Display clear error message
2. Indicate which step failed
3. Provide guidance on how to fix
4. **DO NOT** proceed with remaining steps
5. **DO NOT** create PR if validation fails

---

**Status**: This command performs comprehensive validation and creates well-documented PRs automatically.
