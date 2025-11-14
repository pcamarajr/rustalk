# Fix CI Failures

## Overview

This command analyzes CI failures in a GitHub Pull Request, fetches logs from GitHub Actions, identifies root causes, and automatically fixes safe issues (formatting, linting) or provides detailed suggestions. For complex issues, it creates a GitHub issue with analysis.

**Usage**: `/fix-ci #4` or `/fix-ci` (will prompt for PR number)

**Prerequisites**:

- GitHub CLI (`gh`) must be installed and authenticated
- All changes must be committed (use `/commit` first if needed)
- Local environment must match CI environment (Rust, Node.js versions)

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
```

**If issues found**: Notify the user and **STOP** immediately.

### Step 2: Get PR Number

1. **If PR number provided in command** (e.g., `/fix-ci #4`):

   - Extract the number: `4`
   - Use it directly

2. **If no PR number provided**:

   - Prompt user: "Please provide the PR number (e.g., #4):"
   - Wait for user input

3. **Validate PR exists**:
   ```bash
   gh pr view <PR_NUMBER> --json number,title,state,headRefName,baseRefName
   ```

**If PR doesn't exist or is closed**: Notify the user and **STOP**.

### Step 3: Fetch CI Status and Logs

#### 3.1: Get PR Check Runs

```bash
# Get all check runs for the PR
gh pr checks <PR_NUMBER> --json name,status,conclusion,checkSuite,workflowName > /tmp/ci-checks.json

# Get failed checks
gh pr checks <PR_NUMBER> --json name,status,conclusion,checkSuite,workflowName | \
  jq '.[] | select(.conclusion == "failure" or .conclusion == "cancelled")' > /tmp/failed-checks.json
```

#### 3.2: Get Workflow Run Details

For each failed check:

```bash
# Get workflow run ID
WORKFLOW_RUN_ID=$(gh pr checks <PR_NUMBER> --json checkSuite | \
  jq -r '.[] | select(.checkSuite.workflowRun.id != null) | .checkSuite.workflowRun.id' | head -1)

# Get workflow run details
gh run view $WORKFLOW_RUN_ID --json name,status,conclusion,workflowName,jobs > /tmp/workflow-run.json

# Get job details
gh run view $WORKFLOW_RUN_ID --json jobs | \
  jq -r '.jobs[] | select(.conclusion == "failure") | .id' > /tmp/failed-job-ids.txt
```

#### 3.3: Fetch Job Logs

For each failed job:

```bash
# Get job logs
JOB_ID=$(cat /tmp/failed-job-ids.txt | head -1)
gh run view $WORKFLOW_RUN_ID --log-failed > /tmp/ci-logs.txt

# Also get specific job logs
gh api repos/:owner/:repo/actions/jobs/$JOB_ID/logs --jq '.' > /tmp/job-$JOB_ID-logs.txt
```

**Store logs for analysis**: Save all logs to `/tmp/ci-logs-<timestamp>.txt` for reference.

### Step 4: Analyze CI Failures

Analyze the logs to identify failure types. Categorize each failure:

#### 4.1: Failure Categories

**Category A: Auto-fixable (Safe to fix automatically)**

- **Rust formatting** (`cargo fmt --all -- --check` failed): Run `cargo fmt --all` in `src-tauri/`
- **TypeScript/JavaScript formatting**: Run `npm run lint -- --fix` or `npx prettier --write`
- **Simple linting issues** (auto-fixable ESLint rules): Run `npm run lint -- --fix`

**Category B: Suggest fixes (Requires code changes)**

- **Rust compilation errors**: Analyze error messages, suggest fixes with file paths and line numbers
- **Rust clippy warnings** (non-auto-fixable): Analyze warnings, suggest fixes with code examples
- **TypeScript compilation errors**: Analyze error messages, suggest fixes with file paths and line numbers
- **Test failures**: Analyze test output, identify failing tests, suggest fixes

**Category C: Complex issues (Create GitHub issue)**

- **Build failures**: May require dependency updates or environment changes
- **Flaky tests**: Requires investigation (timing/environment issues)
- **Environment/dependency issues**: Missing dependencies, version mismatches

#### 4.2: Parse Logs for Specific Errors

For each failed job, extract:

1. **Job name** (frontend-check, backend-check, build-macos)
2. **Failed step name** (e.g., "Check formatting", "Run clippy", "Run TypeScript check")
3. **Error messages** (extract from logs)
4. **File paths and line numbers** (if available)
5. **Error type** (compilation, test, lint, format, build)

**Example log parsing**:

```bash
# Extract Rust formatting errors
grep -A 5 "Check formatting" /tmp/ci-logs.txt | grep -E "\.rs:" | \
  sed 's/^.*\(src-tauri\/.*\.rs:[0-9]*\).*$/\1/' > /tmp/formatting-errors.txt

# Extract Rust compilation errors
grep -E "error\[|error:" /tmp/ci-logs.txt | \
  grep -E "\.rs:[0-9]+:[0-9]+" > /tmp/compilation-errors.txt

# Extract TypeScript errors
grep -E "error TS[0-9]+" /tmp/ci-logs.txt | \
  grep -E "\.(ts|tsx|svelte):[0-9]+:[0-9]+" > /tmp/ts-errors.txt
```

### Step 5: Checkout PR Branch

```bash
# Get PR branch name
PR_BRANCH=$(gh pr view <PR_NUMBER> --json headRefName -q .headRefName)

# Checkout the branch
git fetch origin $PR_BRANCH
git checkout $PR_BRANCH

# Ensure branch is up to date
git pull origin $PR_BRANCH
```

**If checkout fails**: Notify the user and **STOP**.

### Step 6: Apply Auto-fixes

For each **Category A** (auto-fixable) issue, apply fixes and verify:

**Rust formatting** (if "Check formatting" step failed):
```bash
cd src-tauri && cargo fmt --all && cd ..
cargo fmt --all -- --check  # Verify
```

**TypeScript/JavaScript formatting/linting** (if linting step failed):
```bash
npm run lint -- --fix
# Or if that doesn't work:
npx prettier --write "src/**/*.{ts,tsx,js,svelte}"
npm run lint  # Verify
```

If verification passes: ✅ Issue fixed. If still fails: Move to Category B (suggest fixes).

### Step 7: Generate Fix Suggestions for Category B Issues

For each **Category B** (suggest fixes) issue, provide:

- **Issue type**: Compilation error, Clippy warning, Test failure, etc.
- **File path and line number**: Exact location
- **Error message**: Full error text
- **Suggested fix**: Code change with before/after examples
- **Context**: Why the fix is needed

**Example suggestion format**:

```
🚫 **Compilation Error** in `src-tauri/src/domain/traits/credential_store.rs:42`

**Error:**
```
error[E0308]: mismatched types
  --> src-tauri/src/domain/traits/credential_store.rs:42:15
   |
42 | fn store(&self, key: &str, value: &str) -> Result<(), String>;
   |               ^^^^^ expected `&String`, found `&str`
```

**Suggested Fix:**
```rust
// ❌ Current
fn store(&self, key: &str, value: &str) -> Result<(), String>;

// ✅ Should be
fn store(&self, key: &String, value: &String) -> Result<(), String>;
```

**Context:** The trait method signature doesn't match the implementation.
```

### Step 8: Commit and Push Fixes

If any auto-fixes were applied (Category A):

```bash
# Stage fixed files
git add -A

# Create commit (conventional format)
if [ -f /tmp/formatting-fixed.txt ]; then
  git commit -m "style(ci): fix formatting issues from CI"
elif [ -f /tmp/linting-fixed.txt ]; then
  git commit -m "fix(ci): resolve linting errors from CI"
else
  git commit -m "fix(ci): resolve CI failures"
fi

# Push changes
git push origin $PR_BRANCH
```

**Display confirmation**: `✅ Auto-fixes committed and pushed`

### Step 9: Create GitHub Issue for Complex Issues

If any **Category C** (complex) issues were found:

**Generate issue content** with:
- **Title**: `[CI Failure] <job-name>: <brief-description>`
- **Body**: PR link, failed job(s)/step(s), full error logs, root cause analysis, investigation steps, related files

**Create issue**:
```bash
gh issue create \
  --title "[CI Failure] <job-name>: <brief-description>" \
  --body-file /tmp/issue-body.md \
  --label "bug,ci"
```

**Display confirmation**: `📋 GitHub Issue Created: <issue-url>`

### Step 10: Display Summary

```
🔍 CI Failure Analysis Complete

✅ Auto-fixed: <count> issue(s)
💡 Fix suggestions: <count> issue(s)
📋 GitHub Issues created: <count>

Next Steps:
1. Review auto-fixes (committed and pushed)
2. Apply suggested fixes for Category B issues
3. Monitor CI status: gh pr checks <PR_NUMBER>
```

---

## Failure Type Detection

**Rust formatting**: `cargo fmt --all -- --check` failed → Auto-fix: `cargo fmt --all`

**Rust clippy**: Warnings in logs → Suggest fixes (or auto-fix if simple like unused imports)

**Rust compilation**: `error[E...]` in logs → Suggest specific code changes

**Rust tests**: `test <name> ... FAILED` → Analyze output and suggest fixes

**TypeScript compilation**: `error TS[0-9]+` → Suggest type fixes

**TypeScript linting**: ESLint errors → Auto-fix if rule is auto-fixable, else suggest

**Build failures**: Compilation/linking errors → Create GitHub issue

---

## Implementation Notes

- Use `grep`, `sed`, `awk`, and `jq` to parse CI logs
- Only auto-fix formatting and auto-fixable linting rules
- Use `gh` for all GitHub operations
- Follow `/commit` command patterns for commits
- Save logs to `/tmp/` for reference

---

## Examples

**Auto-fix formatting**:
```
/fix-ci #4
→ 🔍 Fetching CI status for PR #4...
→ ❌ Found failures in backend-check: Check formatting
→ 🔧 Auto-fixing Rust formatting...
→ ✅ Formatting fixed: 3 files
→ 📝 Committed: style(ci): fix formatting issues from CI
→ 📤 Pushed to origin/feat/design-credential-store-trait
```

**Suggest fixes**:
```
/fix-ci #4
→ 🔍 Fetching CI status for PR #4...
→ ❌ Found failures in backend-check: Run clippy
→ 💡 Found 2 issues requiring fixes:
   🚫 `src-tauri/src/domain/traits/credential_store.rs:42` - unused variable
   🚫 `src-tauri/src/domain/traits/credential_store.rs:78` - use of `unwrap()`
```

**Create issue**:
```
/fix-ci #4
→ 🔍 Fetching CI status for PR #4...
→ ❌ Found failures in build-macos: Build Tauri app
→ ⚠️  Complex issue detected
→ 📋 Created GitHub issue: #123
```

---

## Error Handling

If any step fails:
1. Display clear error message
2. Indicate which step failed
3. Provide guidance on how to fix
4. **DO NOT** proceed if critical (e.g., can't fetch CI status)
5. **DO** continue with available information if non-critical

---

**Status**: This command analyzes CI failures, auto-fixes safe issues, suggests fixes for code issues, and creates GitHub issues for complex problems.

