#!/bin/sh
set -eu
. "$VTEC_CLI_DIR/core/common.sh"
. "$VTEC_CLI_DIR/core/commit-rules.sh"
message=${1:-}
if [ -z "$message" ] || [ "$#" -ne 1 ]; then
  vtec_die "Usage: vtec git commit-msg <message-file>" 2
fi
cd "$VTEC_PROJECT_ROOT"

# Git's comment lines never reach the commit.
clean=$(mktemp)
# vtec's own EXIT trap prints the queued results, so this one chains to it.
trap 'rm -f "$clean"; vtec_flush_results' EXIT
grep -v '^#' "$message" >"$clean" || true

# 1. No AI-generated signature, trailer or footer.
if vtec_has_ai_signature <"$clean"; then
  vtec_die "$VTEC_AI_SIGNATURE_ERROR"
fi

# 2. Changes to the crate name the ticket they implement.
if git diff --cached --name-only | vtec_touches_code && ! vtec_names_ticket <"$clean"; then
  vtec_die "$VTEC_TICKET_ERROR"
fi

# 3. Conventional Commits.
vtec_is_conventional <"$clean" || vtec_die "$VTEC_CONVENTIONAL_ERROR"
