# Commit Changes

## Overview

This command creates conventional commit messages and commits changes. Supports two modes: commit only files changed in this chat session, or commit all changes.

**Usage**:

- `/commit chat` - Commit only files changed in this conversation
- `/commit all` - Stage all changes and commit
- `/commit staged` - Commit staged changes

**Conventional Commits**: Uses title-only format (no body). Auto-detects type: `feat`, `fix`, `docs`, `refactor`, `style`, `test`, `chore`.

---

## Execution Flow

### Step 1: Determine Mode

1. **If mode is "chat"**:

   - Identify files that were edited in this conversation using tool tracking
   - If no files tracked, check git status for modified files (likely chat changes)
   - Stage only those files

2. **If mode is "all"**:

   - Stage all changes: `git add -A`
   - Include both staged and unstaged files

3. **If mode is "staged"**:
   - Commit staged changes: `git commit -m "<message>"`

### Step 2: Check for Changes

```bash
git status --porcelain
```

**If no changes found**: Notify user "No changes to commit" and stop.

### Step 3: Analyze Changes and Group

For each file or group of related files:

1. **Get file diff**:

   ```bash
   git diff --cached <file>  # for staged
   git diff <file>           # for unstaged (then stage it)
   ```

2. **Analyze changes**:

   - New files → likely `feat`
   - Deleted files → likely `refactor` or `chore`
   - Modified files → analyze content:
     - Bug fixes, error handling → `fix`
     - New functionality → `feat`
     - Documentation only → `docs`
     - Code structure changes → `refactor`
     - Formatting, linting → `style`
     - Tests → `test`
     - Build, config → `chore`

3. **Group related files**:
   - Files in same directory/feature area → group together
   - Same file type changes (e.g., all `.md` files) → group if related
   - Unrelated changes → separate commits

### Step 4: Generate Commit Messages

For each group of files:

1. **Determine scope** (optional, if clear):

   - Directory name (e.g., `commands`, `components`)
   - Feature name (e.g., `dialer`, `settings`)
   - File type if all same (e.g., `docs`)

2. **Generate message**:
   - Format: `<type>(<scope>): <description>`
   - Description: Concise summary of what changed (max 72 chars)
   - Use imperative mood: "add feature" not "added feature"

**Examples**:

- `feat(commands): add README validation to review-pr`
- `fix(components): resolve dialer input validation bug`
- `docs: update architecture roadmap`
- `refactor(stores): simplify call state management`

### Step 5: Execute Commits

For each group:

1. **Stage files** (if not already staged):

   ```bash
   git add <file1> <file2> ...
   ```

2. **Create commit**:

   ```bash
   git commit -m "<generated_message>"
   ```

3. **Display summary**:
   ```
   ✅ Committed: <type>(<scope>): <description>
   Files: <file1>, <file2>, ...
   ```

### Step 6: Final Summary

Display all commits created:

```
📦 Commit Summary

✅ feat(commands): add README validation to review-pr
   Files: .cursor/commands/review-pr.md

✅ docs: update architecture roadmap
   Files: docs/architecture/05-implementation-roadmap.md
```

---

## Commit Type Detection Rules

### `feat`

- New files with functionality
- New features, components, commands
- New capabilities added

### `fix`

- Bug fixes
- Error handling improvements
- Correcting incorrect behavior

### `docs`

- README updates
- Documentation files only
- Comments that explain code

### `refactor`

- Code restructuring without behavior change
- Improving code organization
- Removing dead code

### `style`

- Formatting changes
- Linting fixes (whitespace, quotes, etc.)
- No logic changes

### `test`

- Adding or modifying tests
- Test configuration changes

### `chore`

- Build configuration
- Dependencies updates
- CI/CD changes
- Config files

---

## Implementation Notes

- **Tool tracking**: Track files edited via `search_replace`, `write`, `edit_notebook` tools in this conversation
- **Fallback**: If tool tracking unavailable, use git status to detect modified files for "chat" mode
- **Grouping heuristic**:
  - Same directory → same group
  - Same feature area → same group
  - Different areas → separate commits
- **Message generation**: Keep descriptions concise, use imperative mood, focus on "what" not "why"
- **Error handling**: If commit fails, show error and don't proceed with remaining commits

---

## Examples

**Single change**:

```
/commit chat
→ ✅ Committed: feat(commands): add commit command
   Files: .cursor/commands/commit.md
```

**Multiple related changes**:

```
/commit chat
→ ✅ Committed: feat(components): add dialer keyboard shortcuts
   Files: src/lib/components/dialer/Dialer.svelte, src/lib/hooks/useDialerKeyboard.ts
```

**Multiple unrelated changes** (auto-grouped):

```
/commit all
→ ✅ Committed: feat(commands): add README validation
   Files: .cursor/commands/review-pr.md

→ ✅ Committed: docs: update setup instructions
   Files: docs/development/setup.md

→ ✅ Committed: fix(components): resolve dialer input bug
   Files: src/lib/components/dialer/PhoneNumberInput.svelte
```
