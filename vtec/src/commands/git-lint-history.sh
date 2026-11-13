#!/bin/sh
set -eu
. "$VTEC_CLI_DIR/core/common.sh"
. "$VTEC_CLI_DIR/core/commit-rules.sh"
[ "$#" -eq 2 ] || vtec_die "Usage: vtec git lint-history <from> <to>" 2
vtec_need git
cd "$VTEC_PROJECT_ROOT"

# The commit-msg guardrails, applied to every commit in the range, so a
# commit made with --no-verify cannot skip them.
status=0
for commit in $(git rev-list --reverse "$1..$2"); do
  commit_title=$(git log -1 --format='%h %s' "$commit")

  if git log -1 --format=%B "$commit" | vtec_has_ai_signature; then
    vtec_print_error "$commit_title: $VTEC_AI_SIGNATURE_ERROR"
    status=1
  fi

  if git diff-tree --root --no-commit-id --name-only -r "$commit" | vtec_touches_code &&
    ! git log -1 --format=%B "$commit" | vtec_names_ticket; then
    vtec_print_error "$commit_title: $VTEC_TICKET_ERROR"
    status=1
  fi

  if ! git log -1 --format=%B "$commit" | vtec_is_conventional; then
    vtec_print_error "$commit_title: $VTEC_CONVENTIONAL_ERROR"
    status=1
  fi
done

[ "$status" -ne 0 ] || vtec_print_success "Every commit from $1 to $2 passes the guardrails."
exit "$status"
