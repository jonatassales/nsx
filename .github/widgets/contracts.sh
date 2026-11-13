#!/bin/sh
set -eu

: "${VTEC_PROJECT_ROOT:?Run it through vtec/vtec.sh widget.}"
: "${VTEC_CLI_DIR:?Run it through vtec/vtec.sh widget.}"
. "$VTEC_CLI_DIR/core/common.sh"

# Whether the commit range changes the crate, its contracts or the JSON
# Schemas the LangDrift console validates reports against.

base_sha=${1:-${BASE_SHA:-}}
head_sha=${2:-${HEAD_SHA:-HEAD}}

count() {
  [ -n "$base_sha" ] || {
    printf 'unknown'
    return 0
  }

  changed=$(git diff --name-only "$base_sha" "$head_sha" -- "$@") ||
    vtec_die "git diff could not compare $base_sha with $head_sha."

  printf '%s\n' "$changed" | grep -c . || true
}

crate=$(count src)
contracts=$(count src/contract)
schemas=$(count schema)

cat <<EOF_WIDGET
### Contracts Gate

| Part | Files changed |
| :--- | ---: |
| The crate (\`src/\`) | ${crate} |
| The contracts (\`src/contract/\`) | ${contracts} |
| The published schemas (\`schema/\`) | ${schemas} |

A change under \`schema/\` changes what the LangDrift console validates: it needs the console's tests to run against it.
EOF_WIDGET
