---
name: pr-review
description: Multi-agent PR review orchestration for RUSTALK with comprehensive testing validation
tools: mcp__claude-flow__swarm_init, mcp__claude-flow__agent_spawn, mcp__claude-flow__task_orchestrate, Bash, Read, Write, Grep
type: review
color: '#9B59B6'
priority: high
hooks:
  pre: |
    echo "🔍 Initializing RUSTALK PR Review Swarm..."
    gh auth status || (echo "❌ GitHub CLI not authenticated" && exit 1)
  post: |
    echo "✅ PR Review Complete"
    echo "📊 Review summary posted to PR"
---

# RUSTALK PR Review Swarm

Multi-agent orchestration for comprehensive pull request reviews with focus on testing instructions and deployment readiness.

## Usage

```bash
# Review current PR
npx claude-flow@alpha command rustalk/pr-review --pr <PR_NUMBER>

# Review with specific focus areas
npx claude-flow@alpha command rustalk/pr-review --pr <PR_NUMBER> --focus security,testing,docs
```

## Parameters

- `--pr` (required): Pull request number
- `--focus` (optional): Comma-separated focus areas (security, testing, docs, performance, architecture)
- `--depth` (optional): Review depth (quick, standard, comprehensive) - default: standard
- `--post-comment` (optional): Automatically post review as PR comment - default: true

## Agent Team

### 1. Testing Validator Agent

**Role**: Validate testing instructions completeness

**Checks:**

- ✓ Prerequisites section complete with versions
- ✓ 0-to-1 setup instructions present
- ✓ Feature testing steps documented
- ✓ Expected results clearly stated
- ✓ Automated test commands included
- ✓ Build time expectations set
- ✓ Troubleshooting section present

**Output:**

```markdown
## 🧪 Testing Instructions Review

### ✅ Complete

- Prerequisites listed with versions
- Setup from scratch documented

### ⚠️ Missing

- Expected build times not documented
- Troubleshooting section empty

### 📝 Suggestions

1. Add estimated build time: ~2 minutes for initial cargo build
2. Include common issues like "cargo check fails due to missing deps"
```

### 2. Security Agent

**Role**: Security vulnerability scanning

**Checks:**

- Credential handling (no hardcoded secrets)
- TLS/SIPS enforcement
- Input validation
- Dependency vulnerabilities
- Authentication/authorization
- Secure storage usage

**Integration:**

```bash
# Run cargo audit
cargo audit

# Check for secrets
git diff main... | grep -i "password\|secret\|api_key\|token"
```

### 3. Architecture Agent

**Role**: Clean architecture compliance

**Checks:**

- Layer separation (Domain/Application/Infrastructure)
- Dependency direction correctness
- SOLID principles adherence
- File organization (no root files)
- Module structure

**References:**

- `/docs/architecture/01-layers.md`
- `CLAUDE.md` file organization rules

### 4. Production Validator Agent

**Role**: Deployment readiness verification

**Checks:**

- All features fully implemented (no TODOs)
- Tests passing with coverage targets
- Build succeeds on target platforms
- Documentation complete
- No commented-out code
- Error handling present

### 5. Documentation Agent

**Role**: Documentation completeness

**Checks:**

- Testing instructions complete
- API changes documented
- Architecture docs updated
- README updated if needed
- Code comments adequate

## Review Workflow

### Phase 1: Initialize Swarm

```javascript
// Initialize hierarchical swarm
mcp__claude -
  flow__swarm_init({
    topology: 'hierarchical',
    maxAgents: 5,
    strategy: 'specialized',
  });

// Spawn specialized review agents
const agents = [
  'testing-validator',
  'security',
  'architecture',
  'production-validator',
  'documentation',
];

for (const agentType of agents) {
  (await mcp__claude) -
    flow__agent_spawn({
      type: agentType,
      capabilities: [agentType, 'code-review', 'rustalk-specific'],
    });
}
```

### Phase 2: Gather PR Context

```bash
# Get PR data using gh CLI
PR_NUMBER=$1
PR_DATA=$(gh pr view $PR_NUMBER --json title,body,files,labels,commits)
PR_DIFF=$(gh pr diff $PR_NUMBER)
PR_FILES=$(gh pr view $PR_NUMBER --json files --jq '.files[].path')

# Store context in memory for agents
mcp__claude-flow__memory_usage \
  --action store \
  --namespace "rustalk/pr-review" \
  --key "pr-${PR_NUMBER}-context" \
  --value "$PR_DATA"
```

### Phase 3: Distribute Review Tasks

```javascript
// Orchestrate parallel review tasks
const reviewTasks = [
  {
    agent: 'testing-validator',
    task: 'Validate testing instructions completeness in PR body and template compliance',
    priority: 'critical',
  },
  {
    agent: 'security',
    task: 'Scan code changes for security vulnerabilities and verify secure practices',
    priority: 'critical',
  },
  {
    agent: 'architecture',
    task: 'Verify clean architecture compliance and file organization',
    priority: 'high',
  },
  {
    agent: 'production-validator',
    task: 'Verify all features fully implemented and deployment ready',
    priority: 'high',
  },
  {
    agent: 'documentation',
    task: 'Ensure documentation is complete and accurate',
    priority: 'medium',
  },
];

// Execute tasks in parallel
(await mcp__claude) -
  flow__task_orchestrate({
    task: JSON.stringify(reviewTasks),
    strategy: 'parallel',
    priority: 'high',
  });
```

### Phase 4: Aggregate & Report

```javascript
// Retrieve review results from memory
const results = {
  testing: await getAgentResults('testing-validator'),
  security: await getAgentResults('security'),
  architecture: await getAgentResults('architecture'),
  production: await getAgentResults('production-validator'),
  documentation: await getAgentResults('documentation'),
};

// Generate review summary
const summary = generateReviewSummary(results);

// Post to PR if enabled
if (options.postComment) {
  await postPRComment(PR_NUMBER, summary);
}
```

## Review Comment Template

```markdown
# 🤖 RUSTALK PR Review - Multi-Agent Analysis

**PR #{{ pr_number }}**: {{ pr_title }}
**Review Date**: {{ timestamp }}
**Agents Deployed**: 5 (Testing, Security, Architecture, Production, Documentation)

---

## 📊 Review Summary

| Agent                   | Status       | Critical Issues | Warnings       | Suggestions       |
| ----------------------- | ------------ | --------------- | -------------- | ----------------- |
| 🧪 Testing Validator    | {{ status }} | {{ critical }}  | {{ warnings }} | {{ suggestions }} |
| 🔒 Security             | {{ status }} | {{ critical }}  | {{ warnings }} | {{ suggestions }} |
| 🏗️ Architecture         | {{ status }} | {{ critical }}  | {{ warnings }} | {{ suggestions }} |
| ✅ Production Validator | {{ status }} | {{ critical }}  | {{ warnings }} | {{ suggestions }} |
| 📚 Documentation        | {{ status }} | {{ critical }}  | {{ warnings }} | {{ suggestions }} |

**Overall Status**: {{ overall_status }}

---

## 🔴 Critical Issues ({{ critical_count }})

{{ critical_issues }}

---

## 🟡 Warnings ({{ warning_count }})

{{ warnings }}

---

## 💡 Suggestions ({{ suggestion_count }})

{{ suggestions }}

---

## ✅ What's Good

{{ positive_findings }}

---

## 📋 Detailed Agent Reports

<details>
<summary>🧪 Testing Validator Report</summary>

{{ testing_report }}

</details>

<details>
<summary>🔒 Security Report</summary>

{{ security_report }}

</details>

<details>
<summary>🏗️ Architecture Report</summary>

{{ architecture_report }}

</details>

<details>
<summary>✅ Production Validator Report</summary>

{{ production_report }}

</details>

<details>
<summary>📚 Documentation Report</summary>

{{ documentation_report }}

</details>

---

## 🎯 Recommended Actions

- [ ] {{ action_1 }}
- [ ] {{ action_2 }}
- [ ] {{ action_3 }}

---

**Review Quality**: {{ quality_score }}/10
**Estimated Fix Time**: {{ fix_time }}

🤖 Generated with [Claude Flow](https://github.com/ruvnet/claude-flow)
Powered by RUSTALK Multi-Agent Review Swarm
```

## Testing Validation Rules

### Required Sections Checklist

```yaml
required_sections:
  prerequisites:
    - tool_versions_specified: true
    - platform_requirements: true
    - optional_dependencies_noted: true

  setup_instructions:
    - clone_checkout_steps: true
    - dependency_installation: true
    - build_commands: true
    - from_scratch: true # Must start from clean state

  feature_testing:
    - per_feature_breakdown: true
    - step_by_step_instructions: true
    - expected_results: true
    - verification_commands: true

  automated_tests:
    - rust_test_command: true
    - frontend_test_command: true
    - e2e_test_command: true
    - coverage_targets: true

  build_expectations:
    - initial_build_time: true
    - install_time: true
    - dev_mode_startup_time: true

  troubleshooting:
    - common_issues_documented: true
    - solutions_provided: true
```

### Validation Script

```bash
#!/bin/bash
# .claude/scripts/validate-pr-testing.sh

PR_BODY=$(gh pr view $1 --json body --jq .body)

# Check for required sections
MISSING_SECTIONS=()

echo "$PR_BODY" | grep -q "### Prerequisites" || MISSING_SECTIONS+=("Prerequisites")
echo "$PR_BODY" | grep -q "### Setup from Scratch" || MISSING_SECTIONS+=("Setup from Scratch")
echo "$PR_BODY" | grep -q "### How to Test" || MISSING_SECTIONS+=("How to Test")
echo "$PR_BODY" | grep -q "### Running Automated Tests" || MISSING_SECTIONS+=("Running Automated Tests")
echo "$PR_BODY" | grep -q "### Expected Build Times" || MISSING_SECTIONS+=("Expected Build Times")
echo "$PR_BODY" | grep -q "### Troubleshooting" || MISSING_SECTIONS+=("Troubleshooting")

if [ ${#MISSING_SECTIONS[@]} -gt 0 ]; then
  echo "❌ Missing testing instruction sections:"
  printf '  - %s\n' "${MISSING_SECTIONS[@]}"
  exit 1
else
  echo "✅ All required testing sections present"
fi
```

## Integration with GitHub Actions

```yaml
# .github/workflows/pr-review-check.yml
name: RUSTALK PR Review Check

on:
  pull_request:
    types: [opened, synchronize, edited]

jobs:
  validate-testing-instructions:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Setup GitHub CLI
        run: echo "${{ secrets.GITHUB_TOKEN }}" | gh auth login --with-token

      - name: Validate Testing Instructions
        run: |
          bash .claude/scripts/validate-pr-testing.sh ${{ github.event.pull_request.number }}

      - name: Post validation result
        if: failure()
        run: |
          gh pr comment ${{ github.event.pull_request.number }} \
            --body "❌ Testing instructions incomplete. Please follow the PR template."
```

## Memory Coordination

All agents store their findings in shared memory for coordination:

```javascript
// Testing validator stores findings
mcp__claude -
  flow__memory_usage({
    action: 'store',
    namespace: 'rustalk/pr-review',
    key: `pr-${PR_NUMBER}-testing-validation`,
    value: JSON.stringify({
      status: 'warning',
      missing_sections: ['Expected Build Times'],
      suggestions: ['Add build time estimates'],
      timestamp: Date.now(),
    }),
  });

// Security agent retrieves context
const testingContext =
  (await mcp__claude) -
  flow__memory_usage({
    action: 'retrieve',
    namespace: 'rustalk/pr-review',
    key: `pr-${PR_NUMBER}-testing-validation`,
  });
```

## Best Practices

1. **Run review early** - Don't wait for PR completion
2. **Focus on critical issues first** - Security and testing
3. **Provide actionable feedback** - Include code examples
4. **Maintain respectful tone** - Review code, not people
5. **Coordinate across agents** - Share context via memory
6. **Post consolidated report** - Single PR comment with all findings

## Examples

### Quick Review

```bash
npx claude-flow@alpha command rustalk/pr-review --pr 1 --depth quick
```

### Comprehensive Security-Focused Review

```bash
npx claude-flow@alpha command rustalk/pr-review \
  --pr 1 \
  --depth comprehensive \
  --focus security,testing
```

### Silent Review (no PR comment)

```bash
npx claude-flow@alpha command rustalk/pr-review \
  --pr 1 \
  --post-comment false
```

## Related Files

- Template: `.github/pull_request_template.md`
- Validation script: `.claude/scripts/validate-pr-testing.sh`
- Workflow: `.github/workflows/pr-review-check.yml`
- Contributing guide: `CONTRIBUTING.md`
