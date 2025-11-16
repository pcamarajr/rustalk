# Address PR Review Comments with Validation

## Overview

This command automatically fetches all review comments from a GitHub Pull Request, validates each comment through web search and codebase analysis, then either fixes the issues (with individual commits) or replies to comments that don't require fixes.

**Usage**: `/address-pr-comments #4` or `/address-pr-comments` (will prompt for PR number)

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

**If GitHub CLI is not installed**: Notify the user and **STOP** the process immediately.

### Step 2: Verify GitHub CLI Authentication

```bash
if ! gh auth status &> /dev/null; then
  echo "❌ GitHub CLI is not authenticated."
  echo "Please run: gh auth login"
  exit 1
fi
```

### Step 3: Get PR Number

1. **If PR number provided in command** (e.g., `/address-pr-comments #4`):

   - Extract the number: `4`
   - Use it directly

2. **If no PR number provided**:

   - Prompt user: "Please provide the PR number (e.g., #4):"
   - Wait for user input

3. **Validate PR exists**:
   ```bash
   gh pr view <PR_NUMBER> --json number,title,state
   ```

### Step 4: Fetch All Review Comments

Fetch all review comments from the PR (both inline and general comments):

```bash
# Get all review comments (inline and general)
gh pr view <PR_NUMBER> --json reviews,comments > /tmp/pr-comments.json

# Get inline comments from reviews
gh api repos/{owner}/{repo}/pulls/<PR_NUMBER>/comments > /tmp/pr-inline-comments.json

# Get review comments
gh api repos/{owner}/{repo}/pulls/<PR_NUMBER>/reviews > /tmp/pr-reviews.json
```

**Parse comments to extract**:

- Comment ID (from `id` field)
- Comment body/text (from `body` field)
- File path (for inline comments, from `path` field)
- Line number (for inline comments, from `line` field)
- Comment author (from `user.login` field)
- Comment URL: Construct as `https://github.com/{owner}/{repo}/pull/{PR_NUMBER}#discussion_r{COMMENT_ID}` where `COMMENT_ID` is the numeric `id` field
- Review ID (if part of a review, from `pull_request_review_id` field)
- Thread ID (for resolving review threads, from `thread_id` field or `thread.id` field)
- Thread GraphQL Node ID (for resolving via GraphQL, from `thread.node_id` field or construct from thread ID)

**Note**:

- For inline PR review comments, the URL format is: `https://github.com/{owner}/{repo}/pull/{PR_NUMBER}#discussion_r{COMMENT_ID}`
- The thread's GraphQL node ID is required for the `resolveReviewThread` mutation (found in `thread.node_id` field)
- If `thread.node_id` is not available in the REST API response, you may need to query the GraphQL API to get it, or use the comment's `node_id` if it represents the thread
- To get owner and repo, use: `gh repo view --json owner,name -q '.owner.login + "/" + .name'`

### Step 5: Process Each Comment

For each comment found, follow this workflow:

#### Step 5.1: Analyze Comment Intent

Analyze the comment to determine:

- **Fix request**: Comment suggests a specific code change or fix
- **Question/Concern**: Comment asks a question or raises a concern that needs clarification
- **Praise/Approval**: Comment is positive and doesn't require action

**Classification criteria**:

- Contains action verbs: "fix", "change", "update", "remove", "add", "use", "should", "must"
- Contains code suggestions or examples
- Points to specific lines/files with issues
- Asks questions: "why", "how", "what", "can you explain"

#### Step 5.2: Validate Comment Through Web Search

**CRITICAL**: Before addressing any comment, validate it through web search.

**For fix requests**:

1. Extract the problem/solution mentioned in the comment
2. Determine search strategy based on comment context:
   - If comment describes a problem → Search for the problem (e.g., "memory leak in Svelte $effect cleanup")
   - If comment suggests a solution → Search for the solution (e.g., "how to properly cleanup Svelte $effect subscriptions")
   - If both present → Search both and compare results
3. Perform web search using the extracted query
4. Analyze search results to determine if:
   - The problem is a real issue (if searching problem)
   - The solution is recommended/best practice (if searching solution)
   - The comment is accurate and actionable

**For questions/concerns**:

1. Extract the question or concern
2. Search for relevant information to provide an accurate answer
3. Determine if the concern is valid or if clarification is needed

**Web search validation logic**:

- If search results confirm the comment is accurate → Proceed with fix/reply
- If search results contradict the comment → Prepare a reply explaining why
- If search results are inconclusive → Analyze codebase to make decision

#### Step 5.3: Validate Through Codebase Analysis

**CRITICAL**: After web search, analyze the codebase to verify the issue exists.

**For fix requests**:

1. Locate the file and line mentioned in the comment
2. Read the relevant code section
3. Verify:
   - The issue described actually exists in the code
   - The suggested fix is applicable
   - The fix won't break existing functionality
   - The fix aligns with project patterns and conventions

**For questions/concerns**:

1. Analyze the code to understand the context
2. Determine if the concern is valid based on actual code implementation

**Codebase validation logic**:

- If issue exists and fix is valid → Proceed with implementation
- If issue doesn't exist → Prepare reply explaining why
- If fix is not applicable → Prepare reply with explanation

#### Step 5.4: Decision Making

Based on web search + codebase analysis, decide:

**FIX** (if all true):

- Comment is validated through web search
- Issue exists in codebase
- Fix is applicable and safe
- Comment is a fix request (not just a question)

**REPLY** (if any true):

- Comment is a question that needs clarification
- Comment is a concern that needs explanation
- Web search or codebase analysis shows the comment is incorrect/misunderstood
- Comment is valid but doesn't require code changes

**SKIP** (if):

- Comment is praise/approval only
- Comment is already addressed in a previous fix
- Comment is not actionable

#### Step 5.5: Execute Decision

**If FIX**:

1. Implement the fix in the codebase
2. Verify the fix works (check syntax, run linter if available)
3. Commit the change immediately with a short message and link to the comment:
   ```bash
   git add <changed_files>
   git commit -m "fix: address review comment <COMMENT_URL>"
   ```
   **Note**:
   - Replace `<COMMENT_URL>` with the actual comment URL (e.g., `https://github.com/{owner}/{repo}/pull/{PR_NUMBER}#discussion_r{COMMENT_ID}`)
   - GitHub automatically converts URLs into clickable links in commit messages (no markdown formatting needed)
   - This creates a cleaner commit message while still providing a clickable link to the comment
4. Mark the review thread as resolved with a message (if thread ID is available):

   ```bash
   # First, post a reply comment indicating the issue is resolved
   # For PR review comments, create a new comment in reply to the original
   # Get the full commit SHA for autolinking
   COMMIT_SHA=$(git rev-parse HEAD)
   COMMIT_SHORT=$(git rev-parse --short HEAD)
   gh api repos/{owner}/{repo}/pulls/{PR_NUMBER}/comments \
     --method POST \
     --field body="✅ Resolved in commit ${COMMIT_SHA}" \
     --field in_reply_to={COMMENT_ID}

   # Then, use GraphQL API to resolve the review thread
   # Note: Thread ID needs to be the GraphQL node ID
   # Get the thread's GraphQL node ID from the comment data (look for thread.node_id)
   gh api graphql -f query='
     mutation {
       resolveReviewThread(input: {threadId: "<THREAD_NODE_ID>"}) {
         thread {
           isResolved
         }
       }
     }
   '
   ```

   **Note**:

   - Replace `<THREAD_NODE_ID>` with the GraphQL node ID of the thread (not the REST API numeric ID)
   - The thread's GraphQL node ID can be found in the comment's `thread.node_id` field
   - If `thread.node_id` is not available, you may need to query the GraphQL API to get the thread's node ID from the comment's node ID
   - If the comment is not part of a review thread (e.g., general issue comment), skip the resolve step but still post the reply comment
   - The full commit SHA (`${COMMIT_SHA}`) is used so GitHub automatically converts it into a clickable link to the commit (see [GitHub autolinking docs](https://docs.github.com/en/get-started/writing-on-github/working-with-advanced-formatting/autolinked-references-and-urls))
   - Replace `{COMMENT_ID}` with the numeric ID of the comment being replied to

5. Move to next comment

**If REPLY**:

1. Generate a thoughtful reply based on:
   - Web search findings
   - Codebase analysis
   - Project context
2. Post reply to GitHub:
   ```bash
   # For inline comments (stored as issue comments)
   # Note: Inline PR comments are accessible via the issues API
   # Get the numeric comment ID from the comment data
   gh api repos/{owner}/{repo}/issues/<PR_NUMBER>/comments \
     --method POST \
     --field body="> **Reply to comment #<COMMENT_ID>**
   ```

<reply_text>"

# For review comments (reply to review)

gh api repos/{owner}/{repo}/pulls/<PR_NUMBER>/reviews/<REVIEW_ID>/comments \
 --method POST \
 --field body="<reply_text>"

```
**Note**: Inline PR comments are stored as issue comments. To reply:
- Fetch comments using: `gh pr view <PR_NUMBER> --json comments`
- Extract the numeric `id` field (not the GraphQL `node_id`)
- Post a new issue comment that references the original comment ID
3. Move to next comment

**If SKIP**:
- Log the skip reason
- Move to next comment

### Step 6: Process All Comments

Continue processing all comments from the PR until all are addressed:
- Process comments in order (oldest first, or by file/line order for inline comments)
- Handle both inline comments and general review comments with the same process
- Track which comments were fixed, replied to, or skipped

### Step 7: Final Status

After processing all comments, display a brief status:
- Total comments processed
- Number of fixes applied
- Number of replies posted
- Number of comments skipped

---

## Comment Processing Details

### Inline Comments vs General Comments

**Same process for both**:
- Both go through the same validation workflow (web search + codebase analysis)
- Both can result in fixes or replies
- Both are processed in the same order

**Differences in handling**:
- Inline comments: Include file path and line number in context
- General comments: May require broader codebase analysis
- Reply posting: Different API endpoints (see Step 5.5)

### Web Search Strategy

**AI decides based on comment context**:

1. **Problem-focused comments**: "This code has a memory leak"
- Search: "memory leak in [technology/context]"
- Validate: Does the problem exist?

2. **Solution-focused comments**: "You should use $derived instead of $state"
- Search: "when to use $derived vs $state in Svelte 5"
- Validate: Is the solution recommended?

3. **Mixed comments**: "This has a memory leak, you should cleanup in $effect"
- Search both: problem and solution
- Validate: Both problem and solution

4. **Question comments**: "Why did you use this approach?"
- Search: Relevant context to provide accurate answer
- Validate: Can we provide a good explanation?

### Codebase Analysis Strategy

**For fix requests**:
1. Read the file mentioned in the comment
2. Check the specific line(s) referenced
3. Understand the context (surrounding code, function, component)
4. Verify the issue exists
5. Check if fix aligns with project patterns
6. Ensure fix won't introduce new issues

**For questions/concerns**:
1. Read relevant code sections
2. Understand the implementation
3. Check project documentation/patterns
4. Determine if concern is valid
5. Prepare explanation based on actual code

### Validation Criteria

**Comment is TRUE and needs FIX if**:
- ✅ Web search confirms the problem exists OR solution is recommended
- ✅ Codebase analysis shows the issue exists
- ✅ Fix is applicable and safe
- ✅ Comment is a clear fix request

**Comment needs REPLY if**:
- ❌ Comment is a question requiring explanation
- ❌ Comment is a concern that needs clarification
- ❌ Web search or codebase shows comment is incorrect/misunderstood
- ❌ Comment is valid but doesn't require code changes
- ❌ Fix is not applicable or would cause issues

**Comment should be SKIPPED if**:
- ⏭️ Comment is praise/approval only
- ⏭️ Comment is already addressed
- ⏭️ Comment is not actionable
- ⏭️ Comment is a duplicate

---

## Commit Message Format

**Standard format**: `fix: address review comment <COMMENT_URL>`

**Examples**:
- `fix: address review comment https://github.com/owner/repo/pull/12#discussion_r123456789`
- `fix: address review comment https://github.com/owner/repo/pull/12#discussion_r987654321`

**Note**:
- The commit message is clean and concise, with the URL placed directly in the message
- The comment URL format is: `https://github.com/{owner}/{repo}/pull/{PR_NUMBER}#discussion_r{COMMENT_ID}`
- GitHub automatically converts URLs into clickable links in commit messages (see [GitHub autolinking docs](https://docs.github.com/en/get-started/writing-on-github/working-with-advanced-formatting/autolinked-references-and-urls))
- No markdown formatting needed - just the plain URL
- For inline PR review comments, the URL format uses `#discussion_r{COMMENT_ID}`

---

## Error Handling

If any step fails:

1. Display clear error message
2. Indicate which comment and step failed
3. Provide guidance on how to fix
4. **Continue with next comment** (don't stop entire process)
5. Log failed comments for manual review

**Common errors**:
- Comment references file that doesn't exist → Skip with explanation
- Fix causes syntax errors → Revert fix, reply explaining issue
- Web search fails → Rely on codebase analysis only
- GitHub API fails → Retry once, then skip

---

## Implementation Notes

- **One PR at a time**: Process all comments from a single PR in one run
- **Sequential processing**: Handle comments one by one, commit each fix immediately
- **Auto-post replies**: Replies are automatically posted to GitHub
- **No summary**: Process completes without creating a summary report
- **Validation is critical**: Never fix without validating through web search + codebase analysis
- **Be conservative**: When in doubt, reply instead of fixing

---

## Example Workflow

1. User runs: `/address-pr-comments #42`
2. Command fetches all comments from PR #42
3. For each comment:
- Analyzes: "This uses legacy Svelte syntax, should use `onclick`"
- Web search: "Svelte 5 onclick vs on:click syntax"
- Codebase: Checks if file actually uses `on:click`
- Decision: FIX (validated, issue exists)
- Action: Replaces `on:click` with `onclick`, commits
4. Next comment:
- Analyzes: "Why did you choose this approach?"
- Web search: Context about the approach
- Codebase: Analyzes the implementation
- Decision: REPLY (question, not fix request)
- Action: Posts reply explaining the approach
5. Continues until all comments processed

---

**Status**: This command validates PR review comments through web search and codebase analysis before addressing them.

```
