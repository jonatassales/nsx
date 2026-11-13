#!/bin/sh
set -eu
. "$DRIFT_CLI_DIR/core/common.sh"
. "$DRIFT_CLI_DIR/core/commit-rules.sh"
message=${1:-}
if [ -z "$message" ] || [ "$#" -ne 1 ]; then
  drift_die "Usage: drift git commit-msg <message-file>" 2
fi
cd "$DRIFT_PROJECT_ROOT"

# Git's comment lines never reach the commit.
clean=$(mktemp)
# drift's own EXIT trap prints the queued results, so this one chains to it.
trap 'rm -f "$clean"; drift_flush_results' EXIT
grep -v '^#' "$message" >"$clean" || true

# 1. No AI-generated signature, trailer or footer.
if drift_has_ai_signature <"$clean"; then
  drift_die "$DRIFT_AI_SIGNATURE_ERROR"
fi

# 2. Changes to the crate name the ticket they implement.
if git diff --cached --name-only | drift_touches_code && ! drift_names_ticket <"$clean"; then
  drift_die "$DRIFT_TICKET_ERROR"
fi

# 3. Conventional Commits.
drift_is_conventional <"$clean" || drift_die "$DRIFT_CONVENTIONAL_ERROR"
