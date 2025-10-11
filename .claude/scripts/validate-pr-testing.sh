#!/bin/bash
# RUSTALK PR Testing Instructions Validation Script
# Usage: ./validate-pr-testing.sh <PR_NUMBER>

set -e

PR_NUMBER=$1

if [ -z "$PR_NUMBER" ]; then
  echo "Usage: $0 <PR_NUMBER>"
  exit 1
fi

echo "🔍 Validating PR #${PR_NUMBER} testing instructions..."

# Get PR body
PR_BODY=$(gh pr view $PR_NUMBER --json body --jq .body)

if [ -z "$PR_BODY" ]; then
  echo "❌ Could not fetch PR body"
  exit 1
fi

# Initialize validation tracking
MISSING_SECTIONS=()
INCOMPLETE_SECTIONS=()
WARNINGS=()
VALIDATION_PASSED=true

# Check for required sections
echo ""
echo "📋 Checking required sections..."

check_section() {
  local section_name=$1
  local search_pattern=$2

  if echo "$PR_BODY" | grep -q "$search_pattern"; then
    echo "  ✅ $section_name"
  else
    echo "  ❌ $section_name - MISSING"
    MISSING_SECTIONS+=("$section_name")
    VALIDATION_PASSED=false
  fi
}

check_section "Prerequisites" "### Prerequisites"
check_section "Setup from Scratch" "### Setup from Scratch"
check_section "How to Test" "### How to Test"
check_section "Running Automated Tests" "### Running Automated Tests"
check_section "Expected Build Times" "### Expected Build Times"
check_section "Troubleshooting" "### Troubleshooting"

# Check for placeholder content
echo ""
echo "🔎 Checking for placeholder content..."

if echo "$PR_BODY" | grep -q "Feature 1: \[Feature Name\]"; then
  echo "  ⚠️ Feature testing sections contain placeholders"
  WARNINGS+=("Feature testing sections not customized")
fi

if echo "$PR_BODY" | grep -q -E "Required:\s*-\s*-\s*-"; then
  echo "  ⚠️ Prerequisites section not filled out"
  INCOMPLETE_SECTIONS+=("Prerequisites")
fi

if echo "$PR_BODY" | grep -q "~X minutes"; then
  echo "  ⚠️ Build time estimates not provided"
  INCOMPLETE_SECTIONS+=("Expected Build Times")
fi

if echo "$PR_BODY" | grep -q "\[Common problem\]"; then
  echo "  ⚠️ Troubleshooting section contains placeholders"
  INCOMPLETE_SECTIONS+=("Troubleshooting")
fi

# Check for specific testing details
echo ""
echo "🧪 Checking testing detail quality..."

if echo "$PR_BODY" | grep -q "cargo test"; then
  echo "  ✅ Rust test commands documented"
else
  echo "  ⚠️ Rust test commands not explicitly documented"
  WARNINGS+=("Rust test commands should be explicitly documented")
fi

if echo "$PR_BODY" | grep -q "npm test"; then
  echo "  ✅ Frontend test commands documented"
else
  echo "  ⚠️ Frontend test commands not explicitly documented"
  WARNINGS+=("Frontend test commands should be explicitly documented")
fi

# Check for version numbers in prerequisites
if echo "$PR_BODY" | grep -q -E "(Rust|Node|macOS|Windows) [0-9]+"; then
  echo "  ✅ Tool versions specified"
else
  echo "  ⚠️ Tool versions should be specified in prerequisites"
  WARNINGS+=("Specify exact tool versions in prerequisites")
fi

# Generate validation report
echo ""
echo "================================================"
echo "📊 VALIDATION REPORT"
echo "================================================"

if [ ${#MISSING_SECTIONS[@]} -gt 0 ]; then
  echo ""
  echo "❌ MISSING REQUIRED SECTIONS:"
  for section in "${MISSING_SECTIONS[@]}"; do
    echo "  - $section"
  done
fi

if [ ${#INCOMPLETE_SECTIONS[@]} -gt 0 ]; then
  echo ""
  echo "⚠️ INCOMPLETE SECTIONS:"
  for section in "${INCOMPLETE_SECTIONS[@]}"; do
    echo "  - $section (contains placeholders)"
  done
fi

if [ ${#WARNINGS[@]} -gt 0 ]; then
  echo ""
  echo "💡 SUGGESTIONS:"
  for warning in "${WARNINGS[@]}"; do
    echo "  - $warning"
  done
fi

if [ "$VALIDATION_PASSED" = true ] && [ ${#INCOMPLETE_SECTIONS[@]} -eq 0 ]; then
  echo ""
  echo "✅ ALL VALIDATION CHECKS PASSED"
  echo ""
  echo "Your PR includes comprehensive testing instructions!"
  exit 0
else
  echo ""
  echo "❌ VALIDATION FAILED"
  echo ""
  echo "Please update your PR to include all required testing sections."
  echo "Refer to .github/pull_request_template.md for the complete template."
  exit 1
fi
