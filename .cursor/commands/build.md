# Build New Feature

## Overview

This command creates a new feature branch following conventional branch naming (`feat/`, `fix/`, `docs/`, etc.) and immediately starts implementing based on your instructions. The command can read from documentation files or search the docs directory for relevant information.

**Usage**:

- `/build <instructions>` - Create branch and start implementing immediately
  - Example: `/build add user authentication from roadmap`
  - Example: `/build implement Phase 3 audio features`
  - Example: `/build fix dialer input validation bug`

**Conventional Branch Naming**: Auto-detects type from description: `feat`, `fix`, `docs`, `refactor`, `style`, `test`, `chore`.

---

## Execution Flow

### Step 1: Parse Instructions

Extract from user input (everything after `/build`):

1. **Feature description**: The main task/feature to implement
2. **Documentation references**: Look for phrases like:
   - "from roadmap" → read `docs/architecture/05-implementation-roadmap.md`
   - "from architecture" → read `docs/architecture/00-overview.md`
   - "from design system" → read `docs/architecture/07-design-system.md`
   - "from [filename]" → read that specific file
3. **Base branch**: Look for "from branch X" - if not specified, default to `main`

### Step 2: Determine Feature Type and Branch Name

Analyze the description to determine branch type:

**Type detection keywords**:

- `feat` (default): "add", "implement", "create", "new feature", "feature", "build"
- `fix`: "fix", "bug", "resolve", "correct", "repair", "error"
- `docs`: "documentation", "docs", "readme", "update docs", "document"
- `refactor`: "refactor", "restructure", "reorganize", "cleanup", "simplify"
- `style`: "format", "style", "linting", "whitespace", "prettier"
- `test`: "test", "testing", "coverage", "add tests"
- `chore`: "chore", "config", "build", "dependencies", "setup"

**Generate branch name**:

- Convert description to kebab-case
- Format: `<type>/<kebab-case-description>`
- Examples:
  - "add user authentication" → `feat/add-user-authentication`
  - "fix dialer input bug" → `fix/dialer-input-validation-bug`
  - "update documentation" → `docs/update-documentation`

### Step 3: Create Branch

1. **Determine base branch**: Use `main` unless user specified "from branch X"
2. **Verify base branch exists**:
   ```bash
   git branch --list <base-branch>
   git branch -r --list origin/<base-branch>
   ```
3. **Create and switch to new branch**:
   ```bash
   git checkout <base-branch>
   git pull origin <base-branch>  # Ensure up to date
   git checkout -b <branch-name>
   ```

**Display confirmation**:

```
✅ Created branch: <branch-name>
📋 Base: <base-branch>
🎯 Feature: <description>
```

### Step 4: Gather Documentation Context

1. **If specific file mentioned**:

   - Read the mentioned file(s)
   - Extract relevant sections based on keywords in description

2. **If no specific file mentioned**:

   - Search `docs/` directory for relevant content:
     - Search for keywords from description
     - Prioritize: `docs/architecture/05-implementation-roadmap.md` (roadmap)
     - Check: `docs/architecture/00-overview.md` (architecture)
     - Check: `docs/architecture/07-design-system.md` (design system)
     - Check: `docs/architecture/08-ui-design.md` (UI design)

3. **Extract relevant context**:
   - Find sections related to the feature
   - Include task IDs, requirements, acceptance criteria
   - Include architecture patterns, design guidelines

### Step 5: Start Implementation

Immediately begin implementing based on:

1. **User instructions** (from Step 1)
2. **Documentation context** (from Step 4)
3. **Project structure** (analyze codebase for patterns)
4. **Architecture guidelines** (from `docs/architecture/`)

**Implementation approach**:

- Follow existing code patterns and conventions
- Reference architecture documentation for layer structure
- Use design system guidelines for UI components
- Follow Svelte patterns from `docs/development/svelte-patterns.md`
- Implement incrementally with testable deliverables
- Create appropriate tests as you go

---

## Documentation Search Strategy

**Priority order**:

1. Explicit file references (if user mentions specific file)
2. Roadmap (`docs/architecture/05-implementation-roadmap.md`) - for phase/feature references
3. Architecture overview (`docs/architecture/00-overview.md`) - for high-level context
4. Design system (`docs/architecture/07-design-system.md`) - for UI/component work
5. UI design (`docs/architecture/08-ui-design.md`) - for screen/flow work
6. Layer documentation (`docs/architecture/01-layers.md`) - for architecture patterns
7. Development patterns (`docs/development/svelte-patterns.md`) - for Svelte-specific guidance

**Extract keywords** from description: feature names, phase references, task IDs, component names, technical terms

---

## Examples

**Feature from roadmap**:

```
/build implement Phase 3 audio engine from roadmap
→ ✅ Created branch: feat/implement-phase-3-audio-engine
→ 📋 Base: main
→ 🎯 Feature: implement Phase 3 audio engine
→ 📖 Reading: docs/architecture/05-implementation-roadmap.md
→ 🚀 Starting implementation...
```

**Bug fix**:

```
/build fix dialer input validation bug
→ ✅ Created branch: fix/dialer-input-validation-bug
→ 📋 Base: main
→ 🎯 Feature: fix dialer input validation bug
→ 🚀 Starting implementation...
```

**Documentation update**:

```
/build update README with Phase 2 completion
→ ✅ Created branch: docs/update-readme-with-phase-2-completion
→ 📋 Base: main
→ 🎯 Feature: update README with Phase 2 completion
→ 🚀 Starting implementation...
```

**Feature from specific branch**:

```
/build add SIP registration from branch develop
→ ✅ Created branch: feat/add-sip-registration
→ 📋 Base: develop
→ 🎯 Feature: add SIP registration
→ 🚀 Starting implementation...
```

**Feature with architecture context**:

```
/build create audio device selector from design system
→ ✅ Created branch: feat/create-audio-device-selector
→ 📋 Base: main
→ 🎯 Feature: create audio device selector
→ 📖 Reading: docs/architecture/07-design-system.md
→ 🚀 Starting implementation...
```
