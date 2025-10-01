---
name: feature-sparc
description: Run complete SPARC cycle for a RUSTALK feature
usage: npx claude-flow@alpha command rustalk/feature-sparc --feature "<feature-name>"
params:
  - name: feature
    required: true
    description: Feature name (e.g., "SIP Registration")
---

# RUSTALK Feature SPARC Workflow

Executes a full SPARC (Specification, Pseudocode, Architecture, Refinement, Completion) cycle for a RUSTALK feature with TDD.

## Workflow Phases

### 1. Specification (S)
- Analyze feature requirements
- Define acceptance criteria
- Document edge cases and error scenarios
- Create feature spec in `/docs/features/<feature-slug>.md`

### 2. Pseudocode (P)
- Design algorithm and data flow
- Define state machines (for SIP flows)
- Plan error handling strategy
- Document in spec file

### 3. Architecture (A)
- Design Rust module structure
- Define Tauri command API
- Plan frontend component structure
- Create architecture diagram

### 4. Refinement (R) - TDD Implementation
- Write failing Rust tests
- Implement Rust backend to pass tests
- Write failing SvelteKit tests
- Implement frontend to pass tests
- Achieve 85%+ backend, 80%+ frontend coverage

### 5. Completion (C)
- Integration testing (E2E with Playwright)
- Code review
- Documentation update
- Create draft PR linked to Linear

## Agent Coordination

This command spawns multiple agents in parallel:

```javascript
// Step 1: Specification & Architecture (parallel)
Task("Specification Agent", "Create detailed spec for <feature>", "specification")
Task("Architecture Agent", "Design system architecture for <feature>", "architecture")
Task("SIP Specialist", "If SIP-related, design protocol flow", "sip-specialist")

// Step 2: TDD Implementation (parallel)
Task("Test Engineer 1", "Write Rust unit tests for <feature>", "tester")
Task("Rust Coder", "Implement Rust backend for <feature>", "coder")
Task("Tauri Engineer", "Create Tauri commands for <feature>", "tauri-engineer")
Task("Frontend Coder", "Build SvelteKit components for <feature>", "coder")
Task("Test Engineer 2", "Write E2E tests for <feature>", "tester")

// Step 3: Review & Documentation (parallel)
Task("Code Reviewer", "Review all code for quality and security", "reviewer")
Task("API Docs", "Generate API documentation", "api-docs")
```

## Memory Coordination

All agents share context via MCP memory:

```javascript
// Store feature specification
mcp__claude-flow__memory_usage {
  action: "store",
  key: "rustalk/features/<feature-slug>/spec",
  namespace: "rustalk",
  value: JSON.stringify({ ... })
}

// Share architecture decisions
mcp__claude-flow__memory_usage {
  action: "store",
  key: "rustalk/features/<feature-slug>/architecture",
  namespace: "rustalk",
  value: JSON.stringify({ modules, commands, components })
}
```

## Example Usage

```bash
# SIP feature
npx claude-flow@alpha command rustalk/feature-sparc --feature "SIP Registration"

# Audio feature
npx claude-flow@alpha command rustalk/feature-sparc --feature "Audio Device Selection"

# UI feature
npx claude-flow@alpha command rustalk/feature-sparc --feature "Call Dialer Interface"
```

## Deliverables

After completion, you will have:

1. **Documentation**:
   - `/docs/features/<feature-slug>.md` - Complete spec
   - `/docs/architecture/<feature-slug>-design.md` - Architecture

2. **Code**:
   - `/src-tauri/src/<module>/` - Rust implementation
   - `/src-tauri/tests/<module>_test.rs` - Rust tests
   - `/src/lib/components/<Component>.svelte` - Frontend
   - `/tests/e2e/<feature>.spec.ts` - E2E tests

3. **Metrics**:
   - Test coverage reports
   - Performance benchmarks
   - Memory coordination logs

## Next Steps

After SPARC completion:
1. Review all generated code and tests
2. Run full test suite: `cargo nextest run && pnpm test && pnpm test:e2e`
3. Create PR: `npx claude-flow@alpha command rustalk/linear-pr --feature "<feature>"`
