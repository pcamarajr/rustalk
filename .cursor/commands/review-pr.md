# Code Review Checklist for RUSTALK PRs

## Overview

Comprehensive code review checklist to ensure code quality, framework compliance, and adherence to project standards. This review covers functionality, architecture, security, and compliance with official documentation for all technologies in our stack.

**Reference Documentation:**

- **Svelte 5**: https://svelte.dev/llms.txt
- **Rust**: https://doc.rust-lang.org/stable/
- **Tauri v2**: https://v2.tauri.app/llms.txt
- **shadcn-svelte**: https://www.shadcn-svelte.com/llms.txt
- **Tailwind CSS v4**: https://tailwindcss.com/
- **Project Docs**: `docs/architecture/`, `docs/development/`

---

## Review Categories

### 1. Functionality & Correctness

#### Blockers (Must Fix)

- [ ] Code implements the intended functionality correctly
- [ ] Edge cases are handled appropriately
- [ ] Error handling is present and appropriate
- [ ] No obvious bugs or logic errors
- [ ] Changes align with PR description and requirements

#### Warnings (Should Fix)

- [ ] Input validation is comprehensive
- [ ] Error messages are user-friendly and actionable
- [ ] Boundary conditions are tested

#### Info (Nice to Have)

- [ ] Code handles edge cases gracefully
- [ ] User feedback is provided for async operations

---

### 2. Code Quality & Maintainability

#### Blockers (Must Fix)

- [ ] Code is readable and well-structured
- [ ] Functions/classes are focused and single-purpose
- [ ] Variable and function names are descriptive
- [ ] No code duplication (DRY principle)
- [ ] Follows project conventions and patterns

#### Warnings (Should Fix)

- [ ] Complex logic is broken down into smaller functions
- [ ] Comments explain "why" not "what"
- [ ] TypeScript types are properly defined (no `any` without justification)
- [ ] Rust code follows idiomatic patterns

#### Info (Nice to Have)

- [ ] Code is self-documenting
- [ ] Consistent formatting (use `cargo fmt` and `npm run lint`)

---

### 3. Security

#### Blockers (Must Fix)

- [ ] No hardcoded secrets, API keys, or credentials
- [ ] Input validation prevents injection attacks
- [ ] Sensitive data is handled securely (use `keyring` crate for credentials)
- [ ] Authentication/authorization checks are present where needed
- [ ] Tauri capabilities are properly configured (check `capabilities/default.json`)

#### Warnings (Should Fix)

- [ ] Error messages don't leak sensitive information
- [ ] User input is sanitized before processing
- [ ] Dependencies are up-to-date and secure

#### Info (Nice to Have)

- [ ] Security best practices are followed
- [ ] Rate limiting considered for API calls

---

### 4. Architecture Compliance

**Reference**: `docs/architecture/01-layers.md`, `docs/architecture/00-overview.md`

#### Blockers (Must Fix)

- [ ] Code follows Clean Architecture principles (5-layer structure)
- [ ] Layer boundaries are respected (no direct dependencies between non-adjacent layers)
- [ ] Business logic is in the correct layer
- [ ] Platform-specific code is properly abstracted

#### Warnings (Should Fix)

- [ ] Components follow Islands Architecture pattern (`docs/architecture/09-islands-architecture.md`)
- [ ] State management follows documented patterns
- [ ] IPC communication uses Tauri invoke API correctly

#### Info (Nice to Have)

- [ ] Code is organized according to architecture layers
- [ ] Future extensibility is considered

---

### 5. Svelte 5 Framework Compliance

**Reference**: `docs/development/svelte-patterns.md`, https://svelte.dev/llms.txt

#### Blockers (Must Fix)

- [ ] Uses Svelte 5 runes mode syntax (`$state`, `$derived`, `$effect`)
- [ ] **NO** legacy `$:` reactive statements (causes runtime errors)
- [ ] **NO** legacy `on:click` syntax (use `onclick` instead)
- [ ] **NO** legacy `<slot />` syntax (use `{@render children()}` with `{#if}` check)
- [ ] Event handlers are properly typed (use `Record<string, any>` or explicit event types)

#### Warnings (Should Fix)

- [ ] Uses `$derived` for computed values (not `$state`)
- [ ] Uses `$effect` for side effects with proper cleanup
- [ ] Store subscriptions use `$effect` with cleanup for legacy stores
- [ ] Uses `$bindable` for two-way binding props (parent must use `bind:`)
- [ ] CSS scoping: Uses `:global()` for classes passed as props

#### Info (Nice to Have)

- [ ] Follows component patterns from `docs/development/svelte-patterns.md`
- [ ] Dialog state management follows documented patterns (`docs/development/svelte-patterns.md#dialog-and-state-management-patterns`)
- [ ] Uses composables from `src/lib/hooks/` for shared functionality

**Common Pitfalls to Check:**

- ❌ `$: computed = value * 2` → ✅ `let computed = $derived(value * 2)`
- ❌ `<Button on:click={handler}>` → ✅ `<Button onclick={handler}>`
- ❌ `<slot />` → ✅ `{#if children}{@render children()}{/if}`
- ❌ Store subscriptions without cleanup → ✅ `$effect(() => { const unsub = store.subscribe(...); return unsub; })`

---

### 6. Rust & Tauri Compliance

**Reference**: https://doc.rust-lang.org/stable/, https://v2.tauri.app/llms.txt

#### Blockers (Must Fix)

- [ ] Code compiles without warnings
- [ ] Follows Rust ownership and borrowing rules
- [ ] Error handling uses `Result<T, E>` appropriately
- [ ] Tauri commands use proper error types
- [ ] Async code uses Tokio runtime correctly

#### Warnings (Should Fix)

- [ ] Uses idiomatic Rust patterns
- [ ] Avoids unnecessary `unwrap()` calls (prefer `?` operator or proper error handling)
- [ ] Tauri IPC follows security best practices
- [ ] Resource cleanup is handled (Drop trait, async cleanup)

#### Info (Nice to Have)

- [ ] Code is optimized for performance
- [ ] Memory safety is prioritized
- [ ] Follows Rust style guide (`cargo fmt`)

**Technology Stack Compliance** (`docs/architecture/06-technology-decisions.md`):

- [ ] Uses approved libraries (rsip, cpal, webrtc-rs, keyring, rustls)
- [ ] Does NOT use rejected libraries (rvoip, pjsip-rs, coreaudio-rs directly)
- [ ] Follows documented integration patterns

---

### 7. Design System & UI Compliance

**Reference**: `docs/architecture/07-design-system.md`, https://www.shadcn-svelte.com/llms.txt, https://tailwindcss.com/

#### Blockers (Must Fix)

- [ ] Uses shadcn-svelte components from `src/lib/components/ui/`
- [ ] Tailwind CSS classes follow utility-first approach
- [ ] Design tokens use Tailwind defaults (spacing, typography, colors)
- [ ] White-label colors use CSS custom properties (`--brand-primary`, etc.)

#### Warnings (Should Fix)

- [ ] Components follow design system patterns
- [ ] Icons use Lucide Svelte (`lucide-svelte`)
- [ ] Accessibility attributes are present (aria-label, aria-disabled, etc.)
- [ ] Responsive design uses Tailwind breakpoints

#### Info (Nice to Have)

- [ ] Animations follow documented guidelines (150ms fast, 200ms normal, 300ms slow)
- [ ] Loading states use Skeleton component
- [ ] Consistent spacing and typography

**Common Patterns:**

- ✅ Use `Button` from `$lib/components/ui/button`
- ✅ Use Tailwind utilities: `p-4`, `text-sm`, `bg-primary`, etc.
- ✅ Use `class:` directive for dynamic classes with scoped CSS
- ✅ Use `:global()` for prop-based classes

---

### 8. Testing & Documentation

#### Blockers (Must Fix)

- [ ] Critical paths have test coverage
- [ ] Tests pass locally
- [ ] No test files are skipped or commented out

#### Warnings (Should Fix)

- [ ] Unit tests cover edge cases
- [ ] Integration tests for complex flows
- [ ] Code comments explain complex logic

#### Info (Nice to Have)

- [ ] Test coverage meets project standards (85%+ backend, 80%+ frontend)
- [ ] Documentation is updated if APIs change

---

### 9. Performance & Optimization

#### Blockers (Must Fix)

- [ ] No obvious performance bottlenecks
- [ ] Async operations don't block the UI thread
- [ ] Large data sets are handled efficiently

#### Warnings (Should Fix)

- [ ] Unnecessary re-renders are avoided
- [ ] Memory leaks are prevented (cleanup in `$effect`)
- [ ] Images/assets are optimized

#### Info (Nice to Have)

- [ ] Code splitting is considered for large features
- [ ] Lazy loading where appropriate

---

## Critical Decision Review

When reviewing code, be critical about decisions that:

- [ ] Contradict documented technology decisions (`docs/architecture/06-technology-decisions.md`)
- [ ] Don't follow official framework/library guidelines
- [ ] Break established patterns in the codebase
- [ ] Introduce technical debt without justification

**If a decision seems wrong:**

1. Check if it's documented in project docs
2. Verify against official framework documentation
3. Question the decision if it doesn't align with best practices
4. Suggest alternatives if the approach is problematic

---

## Review Process

1. **Start with Blockers**: Address all blocker issues first
2. **Review by Category**: Go through each category systematically
3. **Check Framework Compliance**: Verify against official docs
4. **Verify Architecture**: Ensure code follows documented architecture
5. **Final Check**: Ensure all critical issues are resolved

---

## Notes

- This checklist should be used for every PR review
- Be thorough but constructive in feedback
- Reference specific documentation when flagging issues
- Prioritize blockers over warnings, warnings over info items
- When in doubt, refer to official documentation or project docs
