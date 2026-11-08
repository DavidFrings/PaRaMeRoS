#!/usr/bin/env bash
# Configures the repo's GitHub settings with the gh CLI. Safe to run again.
#
# Requirements: `gh auth login` with admin rights on the repo. Run it after the rust-template PR is merged
# (the ruleset forbids direct pushes to main, and the required checks must exist).
#
# Usage: ./scripts/github-setup.sh [OWNER/REPO]
set -euo pipefail

REPO="${1:-$(gh repo view --json nameWithOwner --jq .nameWithOwner)}"

# Branch ruleset to update (or create). Rulesets stack, so this script merges into the existing one
# instead of adding a second ruleset next to it. Its admin bypass must stay: see the create branch below.
RULESET_NAME="Main"

# Required checks, named as they appear on a PR:
# - "api / result": <job ID in ci.yml> / <job name in rust-ci's rust.yml>; sums up fmt, clippy, test, doc, msrv.
# - the other names are the `name:` of the jobs in .github/workflows/ci.yml.
# Security audits and the Docker scan are left out on purpose: a newly published advisory should not block
# unrelated PRs. If a name is wrong, PRs hang on "Expected - waiting for status": check the names on the first PR.
REQUIRED_CHECKS=(
  "api / result"
  "API / Test"
  "Frontend / Format"
  "Frontend / Lint"
  "Frontend / Type Check"
  "Frontend / Test"
)

step() { printf '\n==> %s\n' "$*"; }

# Runs a command and only prints a note if it fails
# (e.g. because the feature is not available on the GitHub plan or for private repos).
try() {
  local out
  if ! out=$("$@" 2>&1); then
    printf '    ! failed: %s\n      %s\n' "$*" "$(head -n1 <<<"$out")" >&2
  fi
}

step "Merge settings for $REPO"
try gh repo edit "$REPO" \
  --enable-squash-merge \
  --enable-merge-commit=false \
  --enable-rebase-merge=false \
  --enable-auto-merge \
  --delete-branch-on-merge \
  --enable-wiki=false
# Squash commit: PR title as title, PR description as body
try gh api -X PATCH "repos/$REPO" -f squash_merge_commit_title=PR_TITLE -f squash_merge_commit_message=PR_BODY

step "Labels"
label() { try gh label create "$1" --repo "$REPO" --color "$2" --description "$3" --force; }
label "bug"             "d73a4a" "Something isn't working"
label "enhancement"     "a2eeef" "New feature or request"
label "breaking-change" "b60205" "Breaks backwards compatibility"
label "documentation"   "0075ca" "Improvements or additions to documentation"
label "chore"           "cfd3d7" "Refactoring and maintenance"
label "ci"              "ededed" "CI, build and tooling"
label "dependencies"    "0366d6" "Dependency updates"
label "security"        "ee0701" "Security-related"
label "needs-triage"    "fbca04" "Needs to be looked at"
label "skip-changelog"  "ffffff" "Exclude from generated release notes"
label "question"        "d876e3" "Further information is requested"
label "good first issue" "7057ff" "Good for newcomers"
label "help wanted"     "008672" "Extra attention is needed"

step "Security features"
try gh api -X PUT "repos/$REPO/vulnerability-alerts"
try gh api -X PUT "repos/$REPO/automated-security-fixes"
try gh api -X PUT "repos/$REPO/private-vulnerability-reporting"
try gh api -X PATCH "repos/$REPO" --input - <<<'{"security_and_analysis":{"secret_scanning":{"status":"enabled"},"secret_scanning_push_protection":{"status":"enabled"}}}'

step "Actions: workflows get read-only access by default"
try gh api -X PUT "repos/$REPO/actions/permissions/workflow" -f default_workflow_permissions=read -F can_approve_pull_request_reviews=false

step "Ruleset \"$RULESET_NAME\" for the default branch"
checks_json=$(printf '{ "context": "%s" },' "${REQUIRED_CHECKS[@]}")
checks_json="[ ${checks_json%,} ]"
rules=$(cat <<JSON
[
  { "type": "deletion" },
  { "type": "non_fast_forward" },
  { "type": "required_linear_history" },
  {
    "type": "pull_request",
    "parameters": {
      "required_approving_review_count": 0,
      "dismiss_stale_reviews_on_push": false,
      "require_code_owner_review": false,
      "require_last_push_approval": false,
      "required_review_thread_resolution": false
    }
  },
  {
    "type": "required_status_checks",
    "parameters": {
      "strict_required_status_checks_policy": false,
      "required_status_checks": $checks_json
    }
  }
]
JSON
)

# Merge $add into an existing ruleset: existing rules, bypass_actors, conditions and enforcement stay
# as they are. Rules are deduped by type (the existing one wins), except required_status_checks, whose
# contexts become the union of both. Only the fields the PUT endpoint accepts are kept.
# shellcheck disable=SC2016 # jq program, not shell expansion
MERGE_JQ='
  .rules as $old
  | {name, target, enforcement, conditions, bypass_actors,
     rules: (reduce $add[] as $r ($old;
       if any(.[]; .type == $r.type) then
         if $r.type == "required_status_checks" then
           map(if .type == "required_status_checks" then
                 (.parameters.required_status_checks // []) as $have
                 | .parameters.required_status_checks = $have + [
                     $r.parameters.required_status_checks[]
                     | select(.context as $c | $have | any(.context == $c) | not)
                   ]
               else . end)
         else . end
       else . + [$r] end))}'

existing=$(gh api "repos/$REPO/rulesets" --jq ".[] | select(.name == \"$RULESET_NAME\") | .id" 2>/dev/null || true)
if [[ -n "$existing" ]]; then
  payload=$(gh api "repos/$REPO/rulesets/$existing" | jq --argjson add "$rules" "$MERGE_JQ")
  try gh api -X PUT "repos/$REPO/rulesets/$existing" --input - <<<"$payload"
else
  # Admins (RepositoryRole 5) may bypass via pull request: the version-bump PR from release.yml is
  # opened with GITHUB_TOKEN, which triggers no CI, so its required checks never report and only
  # this bypass lets it be merged.
  payload=$(jq -n --arg name "$RULESET_NAME" --argjson rules "$rules" '{
    name: $name,
    target: "branch",
    enforcement: "active",
    conditions: { ref_name: { include: ["~DEFAULT_BRANCH"], exclude: [] } },
    bypass_actors: [ { actor_id: 5, actor_type: "RepositoryRole", bypass_mode: "pull_request" } ],
    rules: $rules
  }')
  try gh api -X POST "repos/$REPO/rulesets" --input - <<<"$payload"
fi

printf '\nDone. Check the settings at https://github.com/%s/settings\n' "$REPO"
