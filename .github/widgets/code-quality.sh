#!/bin/sh
set -eu

: "${DRIFT_PROJECT_ROOT:?Run it through drift/drift.sh widget.}"
: "${DRIFT_CLI_DIR:?Run it through drift/drift.sh widget.}"
. "$DRIFT_CLI_DIR/core/common.sh"

# How much of the tree applies each rule file in .claude/rules, as drift
# quality coverage measures it. Its detectors are patterns, so the shares are
# estimates.

coverage=$("$DRIFT_PROJECT_ROOT/drift/drift.sh" quality coverage) ||
  drift_die "drift quality coverage could not measure the rules."

cat <<EOF_WIDGET
### Code Quality Gate

$(printf '%s\n' "$coverage" | sed '1,2d')
EOF_WIDGET
