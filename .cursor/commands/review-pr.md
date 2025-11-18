# Automated PR Review with GitHub CLI Integration

## Overview

This command performs an automated code review of a GitHub Pull Request and posts comments directly to the PR using GitHub CLI. The review analyzes code quality, framework compliance, architecture adherence, and security issues, then automatically posts inline comments for specific issues and a review summary.

**Usage**: `/review-pr #4` or `/review-pr` (will prompt for PR number)

**Prerequisites**: GitHub CLI (`gh`) must be installed and authenticated.

---

## Execution Flow

### Step 1: Check GitHub CLI Installation

**CRITICAL**: Before proceeding, check if GitHub CLI is installed:

```bash
if ! command -v gh &> /dev/null; then
  echo "❌ GitHub CLI (gh) is not installed."
  echo "Please install it first:"
  echo "  macOS: brew install gh"
  echo "  Or visit: https://cli.github.com/"
  exit 1
fi
```

**If GitHub CLI is not installed**: Notify the user and **STOP** the review process immediately.

### Step 2: Verify GitHub CLI Authentication

```bash
if ! gh auth status &> /dev/null; then
  echo "❌ GitHub CLI is not authenticated."
  echo "Please run: gh auth login"
  exit 1
fi
```

### Step 3: Get PR Number

1. **If PR number provided in command** (e.g., `/review-pr #4`):

   - Extract the number: `4`
   - Use it directly

2. **If no PR number provided**:

   - Prompt user: "Please provide the PR number (e.g., #4):"
   - Wait for user input

3. **Validate PR exists**:
   ```bash
   gh pr view <PR_NUMBER> --json number,title,state
   ```

### Step 4: Get PR Diff and Metadata

```bash
# Get PR diff
gh pr diff <PR_NUMBER> > /tmp/pr-diff.txt

# Get PR metadata (title, description, commits) for README checks
gh pr view <PR_NUMBER> --json title,body,commits > /tmp/pr-metadata.json
```

Analyze the diff to identify:

- Changed files
- Line numbers for each change
- Code patterns that need review

Extract PR metadata for README validation:

- PR title and description (for phase completion detection)
- Commit messages (for additional phase completion indicators)
- Changed files (to check if README.md was modified)

### Step 5: Automatic Code Analysis

Analyze the PR diff and metadata against the review criteria below. For each issue found:

1. **Line-specific issues** → Prepare inline comments with file path and line number

**IMPORTANT - README Validation**:

- **ALWAYS** verify README.md reflects the current application state
- If code changes affect documented information but README wasn't updated → **BLOCKER** (see section 10 below)

**CRITICAL REQUIREMENT**: Every issue (blocker, warning, or suggestion) MUST include:

- **Specific file path(s)** where the issue was found
- **Line number(s)** when applicable (for inline comments or when referencing specific code)
- **Component/function names** when relevant

**Never use generic references** like:

- ❌ "Some components are larger"
- ❌ "Various files have issues"
- ❌ "Consider refactoring in several places"

**Always use specific references** like:

- ✅ "`src/lib/components/Dialer.svelte` (245 lines) and `src/lib/components/Settings.svelte` (312 lines) are larger than recommended"
- ✅ "`src/lib/stores/callStore.ts:78` and `src/lib/stores/contactsStore.ts:92` have issues"
- ✅ "Consider refactoring `src/lib/components/ContactList.svelte` and `src/lib/components/ContactDetails.svelte`"

**Review Criteria** (see sections below for details):

- Functionality & Correctness
- Code Quality & Maintainability
- Security
- Architecture Compliance
- Svelte 5 Framework Compliance
- Rust & Tauri Compliance
- Design System & UI Compliance
- Testing & Documentation
- Performance & Optimization
- README & Documentation Updates

### Step 6: Post Comments to GitHub

**CRITICAL**: GitHub CLI's `gh pr comment` does NOT support inline comments (no `--file` or `--line` flags exist). To create inline comments, we must use the GitHub API via `gh api` to create a review with inline comments.

#### Step 6.1: Get PR Head SHA and Prepare Inline Comments

First, get the PR's head commit SHA (required for inline comments):

```bash
# Get PR head commit SHA
PR_HEAD_SHA=$(gh pr view <PR_NUMBER> --json headRefOid -q '.headRefOid')
```

For each line-specific issue found, prepare inline comment data with:

- `path`: File path relative to repo root (e.g., `src/lib/components/Button.svelte`)
- `line`: Line number in the file (must be a line that was changed in the PR)
- `body`: Comment text
- `side`: `"RIGHT"` (for the new version of the file) or `"LEFT"` (for the old version)

**Comment Format for Inline Comments**:

- Start with issue severity: `**🚫 Blocker:**`, `**⚠️ Warning:**`, or `**💡 Suggestion:**`
- **MUST include**: Specific file path and line number in the comment body
- Describe the issue clearly
- Provide fix suggestion
- Reference documentation when applicable

**Example Inline Comment Body**:

````
**🚫 Blocker:** Uses legacy Svelte syntax

This line uses `on:click` which is deprecated in Svelte 5. Please use `onclick` instead.

Reference: `docs/development/svelte-patterns.md#event-handlers`

```svelte
// ❌ Current
<Button on:click={handler}>

// ✅ Should be
<Button onclick={handler}>
````

````

#### Step 6.2: Create Review with Inline Comments

**IMPORTANT**: All inline comments must be submitted as part of a single review. You cannot add inline comments separately.

**CRITICAL - Line Number Requirements**:
- The `line` number must correspond to a line that was **changed** in the PR diff
- Use `side: "RIGHT"` for comments on the new version of the file (after changes)
- Use `side: "LEFT"` for comments on the old version of the file (before changes)
- Line numbers are 1-indexed (first line is 1, not 0)
- If a line wasn't changed, inline comments cannot be placed on it (use general comments instead)

Collect all inline comments and create a review using the GitHub API:

```bash
# Prepare review payload with inline comments
# Build JSON array of comments programmatically
COMMENTS_JSON=$(jq -n \
  --argjson comments "$(jq -n '[{"path":"src/lib/components/Button.svelte","line":42,"side":"RIGHT","body":"**🚫 Blocker:** Uses legacy Svelte syntax\n\nThis line uses `on:click` which is deprecated in Svelte 5. Please use `onclick` instead."},{"path":"src/lib/components/Dialog.svelte","line":15,"side":"RIGHT","body":"**⚠️ Warning:** Consider using `$derived` instead of `$state` for computed values."}]')" \
  '{body:"",event:"COMMENT",comments:$comments}')

# Alternative: Create JSON file if jq is not available
cat > /tmp/review-payload.json <<EOF
{
  "body": "",
  "event": "COMMENT",
  "comments": [
    {
      "path": "src/lib/components/Button.svelte",
      "line": 42,
      "side": "RIGHT",
      "body": "**🚫 Blocker:** Uses legacy Svelte syntax\n\nThis line uses \`on:click\` which is deprecated in Svelte 5. Please use \`onclick\` instead."
    },
    {
      "path": "src/lib/components/Dialog.svelte",
      "line": 15,
      "side": "RIGHT",
      "body": "**⚠️ Warning:** Consider using \`$derived\` instead of \`$state\` for computed values."
    }
  ]
}
EOF

# Submit review with inline comments
gh api repos/{owner}/{repo}/pulls/<PR_NUMBER>/reviews \
  --method POST \
  --input /tmp/review-payload.json \
  --field commit_id="$PR_HEAD_SHA"
```

**Note**:
- The `event` field can be `"COMMENT"`, `"APPROVE"`, or `"REQUEST_CHANGES"` (but for inline comments, typically use `"COMMENT"` and submit final review in Step 7)
- The `commit_id` must be the head commit SHA of the PR (use `headRefOid` from `gh pr view`)
- All inline comments must be in a single API call
- If there are no inline comments, skip this step
- Escape newlines in comment bodies as `\n` when building JSON
- Use backticks in markdown by escaping them as `\`` in JSON strings

### Step 7: Submit Final Review

After posting all inline comments (Step 6.2), automatically submit the final review with overall decision:

**Decision Logic**:

- **If blockers found** → `--request-changes`
- **If only warnings/suggestions** → `--comment` (or `--approve` with comments if minor)
- **If no issues found** → `--approve`

**Note**:
- Inline comments are posted via `gh api` in Step 6.2 (creates a review with inline comments)
- This final step submits the overall review decision (approve/request-changes/comment) with a summary, which will appear as the main review comment

```bash
# Create review summary
SUMMARY="## Code Review Summary

$(count_blockers) blocker(s) found
$(count_warnings) warning(s) found
$(count_suggestions) suggestion(s)

$(concise_summary_of_key_findings)"

# Submit review
if [ $blockers -gt 0 ]; then
  gh pr review <PR_NUMBER> --body "$SUMMARY" --request-changes
elif [ $warnings -gt 0 ] || [ $suggestions -gt 0 ]; then
  gh pr review <PR_NUMBER> --body "$SUMMARY" --comment
else
  gh pr review <PR_NUMBER> --body "$SUMMARY" --approve
fi
```

**Review Summary Format** (Concise):

```
## Code Review Summary

✅ **Overall Assessment:** [Brief assessment]

**Issues Found:**
- 🚫 Blockers: X
- ⚠️ Warnings: Y
- 💡 Suggestions: Z

**Key Findings:**
- [Most critical issue 1]
- [Most critical issue 2]
- [Most critical issue 3]

**Review Details:**
- Inline comments posted for line-specific issues
- Categories reviewed: [List all categories reviewed, including those with no issues if relevant]
```

---

## Review Categories & Criteria

### 1. Functionality & Correctness

#### Blockers (Must Fix)

- Code implements intended functionality correctly
- Edge cases handled appropriately
- Error handling present and appropriate
- No obvious bugs or logic errors
- Changes align with PR description

#### Warnings (Should Fix)

- Input validation comprehensive
- Error messages user-friendly and actionable
- Boundary conditions tested

#### Suggestions (Nice to Have)

- Code handles edge cases gracefully
- User feedback provided for async operations

**Reference**: General code review best practices

---

### 2. Code Quality & Maintainability

#### Blockers (Must Fix)

- Code readable and well-structured
- Functions/classes focused and single-purpose
- Variable and function names descriptive
- No code duplication (DRY principle)
- Follows project conventions and patterns

#### Warnings (Should Fix)

- Complex logic broken down into smaller functions
- Comments explain "why" not "what"
- TypeScript types properly defined (no `any` without justification)
- Rust code follows idiomatic patterns

#### Suggestions (Nice to Have)

- Code is self-documenting
- Consistent formatting (use `cargo fmt` and `npm run lint`)

---

### 3. Security

#### Blockers (Must Fix)

- No hardcoded secrets, API keys, or credentials
- Input validation prevents injection attacks
- Sensitive data handled securely (use `keyring` crate for credentials)
- Authentication/authorization checks present where needed
- Tauri capabilities properly configured (check `capabilities/default.json`)

#### Warnings (Should Fix)

- Error messages don't leak sensitive information
- User input sanitized before processing
- Dependencies up-to-date and secure

#### Suggestions (Nice to Have)

- Security best practices followed
- Rate limiting considered for API calls

---

### 4. Architecture Compliance

**Reference**: `docs/architecture/01-layers.md`, `docs/architecture/00-overview.md`

#### Blockers (Must Fix)

- Code follows Clean Architecture principles (5-layer structure)
- Layer boundaries respected (no direct dependencies between non-adjacent layers)
- Business logic in correct layer
- Platform-specific code properly abstracted

#### Warnings (Should Fix)

- Components follow Islands Architecture pattern (`docs/architecture/09-islands-architecture.md`)
- State management follows documented patterns
- IPC communication uses Tauri invoke API correctly

#### Suggestions (Nice to Have)

- Code organized according to architecture layers
- Future extensibility considered

---

### 5. Svelte 5 Framework Compliance

**Reference**: `docs/development/svelte-patterns.md`, https://svelte.dev/llms.txt

#### Blockers (Must Fix)

- Uses Svelte 5 runes mode syntax (`$state`, `$derived`, `$effect`)
- **NO** legacy `$:` reactive statements (causes runtime errors)
- **NO** legacy `on:click` syntax (use `onclick` instead)
- **NO** legacy `<slot />` syntax (use `{@render children()}` with `{#if}` check)
- Event handlers properly typed (use `Record<string, any>` or explicit event types)

#### Warnings (Should Fix)

- Uses `$derived` for computed values (not `$state`)
- Uses `$effect` for side effects with proper cleanup
- Store subscriptions use `$effect` with cleanup for legacy stores
- Uses `$bindable` for two-way binding props (parent must use `bind:`)
- CSS scoping: Uses `:global()` for classes passed as props

#### Suggestions (Nice to Have)

- Follows component patterns from `docs/development/svelte-patterns.md`
- Dialog state management follows documented patterns (`docs/development/svelte-patterns.md#dialog-and-state-management-patterns`)
- Uses composables from `src/lib/hooks/` for shared functionality

**Common Pitfalls to Check**:

- ❌ `$: computed = value * 2` → ✅ `let computed = $derived(value * 2)`
- ❌ `<Button on:click={handler}>` → ✅ `<Button onclick={handler}>`
- ❌ `<slot />` → ✅ `{#if children}{@render children()}{/if}`
- ❌ Store subscriptions without cleanup → ✅ `$effect(() => { const unsub = store.subscribe(...); return unsub; })`

---

### 6. Rust & Tauri Compliance

**Reference**: https://doc.rust-lang.org/stable/, https://v2.tauri.app/llms.txt

#### Blockers (Must Fix)

- Code compiles without warnings
- Follows Rust ownership and borrowing rules
- Error handling uses `Result<T, E>` appropriately
- Tauri commands use proper error types
- Async code uses Tokio runtime correctly

#### Warnings (Should Fix)

- Uses idiomatic Rust patterns
- Avoids unnecessary `unwrap()` calls (prefer `?` operator or proper error handling)
- Tauri IPC follows security best practices
- Resource cleanup handled (Drop trait, async cleanup)

#### Suggestions (Nice to Have)

- Code optimized for performance
- Memory safety prioritized
- Follows Rust style guide (`cargo fmt`)

**Technology Stack Compliance** (`docs/architecture/06-technology-decisions.md`):

- Uses approved libraries (rsip, cpal, webrtc-rs, keyring, rustls)
- Does NOT use rejected libraries (rvoip, pjsip-rs, coreaudio-rs directly)
- Follows documented integration patterns

---

### 7. Design System & UI Compliance

**Reference**: `docs/architecture/07-design-system.md`, https://www.shadcn-svelte.com/llms.txt, https://tailwindcss.com/

#### Blockers (Must Fix)

- Uses shadcn-svelte components from `src/lib/components/ui/`
- Tailwind CSS classes follow utility-first approach
- Design tokens use Tailwind defaults (spacing, typography, colors)
- White-label colors use CSS custom properties (`--brand-primary`, etc.)

#### Warnings (Should Fix)

- Components follow design system patterns
- Icons use Lucide Svelte (`lucide-svelte`)
- Accessibility attributes present (aria-label, aria-disabled, etc.)
- Responsive design uses Tailwind breakpoints

#### Suggestions (Nice to Have)

- Animations follow documented guidelines (150ms fast, 200ms normal, 300ms slow)
- Loading states use Skeleton component
- Consistent spacing and typography

**Common Patterns**:

- ✅ Use `Button` from `$lib/components/ui/button`
- ✅ Use Tailwind utilities: `p-4`, `text-sm`, `bg-primary`, etc.
- ✅ Use `class:` directive for dynamic classes with scoped CSS
- ✅ Use `:global()` for prop-based classes

---

### 8. Testing & Documentation

#### Blockers (Must Fix)

- Critical paths have test coverage
- Tests pass locally
- No test files skipped or commented out

#### Warnings (Should Fix)

- Unit tests cover edge cases
- Integration tests for complex flows
- Code comments explain complex logic

#### Suggestions (Nice to Have)

- Test coverage meets project standards (85%+ backend, 80%+ frontend)
- Documentation updated if APIs change

---

### 9. Performance & Optimization

#### Blockers (Must Fix)

- No obvious performance bottlenecks
- Async operations don't block UI thread
- Large data sets handled efficiently

#### Warnings (Should Fix)

- Unnecessary re-renders avoided
- Memory leaks prevented (cleanup in `$effect`)
- Images/assets optimized

#### Suggestions (Nice to Have)

- Code splitting considered for large features
- Lazy loading where appropriate

---

### 10. README & Documentation Updates

**Reference**: `README.md`, `docs/`

#### Blockers (Must Fix)

**CRITICAL**: The README must accurately reflect the current state of the application. If code changes affect information documented in the README, the README MUST be updated accordingly.

**Common scenarios that require README updates**:
- Phase completion (roadmap status, current phase section, next steps)
- New features or capabilities added
- Project status changes (badges, CI status)
- Technology stack changes
- Architecture or setup instructions modified

**Validation**: Analyze PR title, description, commits, and code changes. If changes affect documented information in README but README wasn't updated → **BLOCKER**. Post inline comments on `README.md` with specific line numbers and required changes.

#### Warnings (Should Fix)

- Minor documentation inconsistencies
- Missing updates to related documentation files in `docs/` directory

#### Suggestions (Nice to Have)

- README includes examples or screenshots of new features
- Documentation is comprehensive and easy to follow

---

## Critical Decision Review

When reviewing code, be critical about decisions that:

- Contradict documented technology decisions (`docs/architecture/06-technology-decisions.md`)
- Don't follow official framework/library guidelines
- Break established patterns in the codebase
- Introduce technical debt without justification

**If a decision seems wrong**:

1. Check if it's documented in project docs
2. Verify against official framework documentation
3. Question the decision if it doesn't align with best practices
4. Suggest alternatives if the approach is problematic

---

## Implementation Notes

- **Inline comments** should be used for specific line/file issues (most cases) → Use `gh api` to POST to `/repos/{owner}/{repo}/pulls/{pull_number}/reviews` with inline comments in the payload (see Step 6.2)
  - **CRITICAL**: `gh pr comment` does NOT support inline comments (no `--file` or `--line` flags exist)
  - All inline comments must be submitted in a single review API call
  - Requires the PR head commit SHA (`headRefOid`)
- **Final review submission** → Use `gh pr review` with `--approve`/`--request-changes`/`--comment` (overall decision)
- **Review workflow**:
  1. Post inline comments via `gh api` (Step 6.2) - creates a review with inline comments
  2. Submit final review decision via `gh pr review` (Step 7) - sets overall review status (approve/request-changes/comment) with summary
- **CRITICAL - Be specific**: **ALWAYS** include exact file paths and line numbers for every issue found. Never use generic references.
- **File references format**: Use backticks for file paths, e.g., `` `src/lib/components/Button.svelte:42` ``
- **Multiple files**: When multiple files have the same issue, list each one explicitly: `` `file1.svelte:10`, `file2.svelte:25`, `file3.svelte:8` ``
- **Component size issues**: Always list the specific components and their line counts, e.g., "`Dialer.svelte` (245 lines) and `Settings.svelte` (312 lines)"
- **Reference docs**: Link to relevant documentation (e.g., `docs/development/svelte-patterns.md`)
- **Be constructive**: Provide actionable feedback and suggestions
- **Prioritize**: Blockers > Warnings > Suggestions

---

## Error Handling

If any step fails:

1. Display clear error message
2. Indicate which step failed
3. Provide guidance on how to fix
4. Do NOT proceed with remaining steps
5. Do NOT post partial reviews

---

**Status**: This command is fully integrated with GitHub CLI and automatically posts reviews to PRs.
````
