#!/bin/sh
set -eu

: "${VTEC_PROJECT_ROOT:?Run it through vtec/vtec.sh widget.}"
: "${VTEC_CLI_DIR:?Run it through vtec/vtec.sh widget.}"
. "$VTEC_CLI_DIR/core/common.sh"

# How much of the tree applies each rule file in .claude/rules, as vtec
# quality coverage measures it. Its detectors are patterns, so the shares are
# estimates.

coverage=$("$VTEC_PROJECT_ROOT/vtec/vtec.sh" quality coverage) ||
  vtec_die "vtec quality coverage could not measure the rules."

cat <<EOF_WIDGET
### Code Quality Gate

$(printf '%s\n' "$coverage" | sed '1,2d')
EOF_WIDGET
