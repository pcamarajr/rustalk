# Quick Development Review

## Overview

Lightweight code review checklist for staged files or specific commits. Focuses on critical issues and development best practices to catch problems early in the development cycle.

**Use this for**: Quick checks before committing, reviewing staged changes, or reviewing a specific commit.
**For comprehensive PR reviews**: Use `/review-pr` instead.

**Reference Documentation:**

- **Svelte 5**: https://svelte.dev/llms.txt
- **Rust**: https://doc.rust-lang.org/stable/
- **Tauri v2**: https://v2.tauri.app/llms.txt
- **Project Docs**: `docs/architecture/09-islands-architecture.md`, `docs/development/svelte-patterns.md`

---

## Review by File Type

### Svelte Components (.svelte)

#### Blockers (Must Fix)

- [ ] Uses Svelte 5 runes mode (`$state`, `$derived`, `$effect`)
- [ ] **NO** legacy `$:` reactive statements
- [ ] **NO** legacy `on:click` syntax (use `onclick`)
- [ ] **NO** legacy `<slot />` syntax (use `{@render children()}`)
- [ ] Event handlers are properly typed
- [ ] Store subscriptions include cleanup in `$effect` (if using legacy stores)

#### Warnings (Should Fix)

- [ ] Uses `$derived` for computed values (not `$state`)
- [ ] Uses `$effect` for side effects with cleanup
- [ ] CSS scoping: Uses `:global()` for prop-based classes
- [ ] Uses shadcn-svelte components from `$lib/components/ui/`
- [ ] Follows dialog state management patterns (`docs/development/svelte-patterns.md#dialog-and-state-management-patterns`)

#### Info (Nice to Have)

- [ ] Component follows Islands Architecture pattern (`docs/architecture/09-islands-architecture.md`)
- [ ] Uses composables from `src/lib/hooks/` for shared functionality
- [ ] Accessibility attributes present (aria-label, etc.)

---

### TypeScript/JavaScript Files (.ts, .js)

#### Blockers (Must Fix)

- [ ] No `any` types without justification
- [ ] Proper error handling for async operations
- [ ] TypeScript types are correctly defined
- [ ] No console.log statements in production code (use debug prefix if needed)

#### Warnings (Should Fix)

- [ ] Functions are focused and single-purpose
- [ ] Variable names are descriptive
- [ ] Follows project conventions
- [ ] Uses proper TypeScript strict mode

#### Info (Nice to Have)

- [ ] Code is well-commented where needed
- [ ] Consistent formatting

---

### Rust Files (.rs)

#### Blockers (Must Fix)

- [ ] Code compiles without warnings
- [ ] Follows Rust ownership and borrowing rules
- [ ] Error handling uses `Result<T, E>` appropriately
- [ ] No unnecessary `unwrap()` calls (prefer `?` operator)

#### Warnings (Should Fix)

- [ ] Uses idiomatic Rust patterns
- [ ] Tauri commands use proper error types
- [ ] Async code uses Tokio runtime correctly
- [ ] Resource cleanup is handled

#### Info (Nice to Have)

- [ ] Code follows Rust style guide
- [ ] Uses approved libraries from `docs/architecture/06-technology-decisions.md`

---

### Configuration Files

#### Blockers (Must Fix)

- [ ] Configuration follows project standards
- [ ] No hardcoded secrets or sensitive data
- [ ] Tauri capabilities properly configured

#### Warnings (Should Fix)

- [ ] Configuration is documented if non-standard
- [ ] Dependencies are up-to-date

---

## Quick Checks (All File Types)

### Critical Issues

- [ ] No deprecated syntax or patterns
- [ ] No obvious bugs or logic errors
- [ ] Error handling is present
- [ ] No security vulnerabilities (hardcoded secrets, etc.)

### Code Quality

- [ ] Code is readable and well-structured
- [ ] Follows project conventions
- [ ] No code duplication
- [ ] Proper naming conventions

### Framework Compliance

- [ ] Follows official framework documentation
- [ ] Uses approved libraries and patterns
- [ ] No rejected technologies (per `docs/architecture/06-technology-decisions.md`)

---

## Review Process

1. **Identify file types** in staged files/commit
2. **Check Blockers first** - these must be fixed
3. **Review Warnings** - should be addressed
4. **Quick scan for Info items** - nice to have improvements

---

## Notes

- This is a lightweight review for development workflow
- Focus on catching critical issues early
- For comprehensive reviews, use `/review-pr`
- Be constructive and specific in feedback
