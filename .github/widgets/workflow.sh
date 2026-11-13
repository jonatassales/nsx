#!/bin/sh
set -eu

: "${VTEC_PROJECT_ROOT:?Run it through vtec/vtec.sh widget.}"
: "${VTEC_CLI_DIR:?Run it through vtec/vtec.sh widget.}"
. "$VTEC_CLI_DIR/core/common.sh"

# The ticket the commit range names in its Ticket trailers, with its title,
# status and spec, as the ticket file states them.

base_sha=${1:-${BASE_SHA:-}}
head_sha=${2:-${HEAD_SHA:-HEAD}}

range=$head_sha
[ -z "$base_sha" ] || range="$base_sha..$head_sha"

messages=$(git log --format=%B "$range") || vtec_die "git log could not read $range."

tickets=$(printf '%s\n' "$messages" |
  sed -n -E 's/^[[:space:]]*Ticket:[[:space:]]*([0-9]{4})[[:space:]]*$/\1/p' | sort -u)

printf '### Workflow Gate\n\n'

if [ -z "$tickets" ]; then
  printf '%s\n' 'No commit in this range names a ticket.'
  exit 0
fi

printf '| Ticket | Title | Status | Spec |\n| :--- | :--- | :--- | :--- |\n'

for ticket in $tickets; do
  file=$(find "$VTEC_PROJECT_ROOT/.agents/tickets" -type f -name "$ticket-*.ticket.md" | head -n 1)
  if [ -z "$file" ]; then
    printf '| %s | not in the tree | unknown | unknown |\n' "$ticket"
    continue
  fi

  title=$(sed -n -E '1s/^# [0-9]{4}: //p' "$file")
  status=$(sed -n -E 's/^\*\*Status:\*\*[[:space:]]*([a-z-]+).*/\1/p' "$file" | head -n 1)
  spec=$(sed -n -E 's/^\*\*Spec:\*\*[[:space:]]*(None|[^ .]+).*/\1/p' "$file" | head -n 1)
  printf '| %s | %s | %s | %s |\n' "$ticket" "$title" "${status:-unknown}" "${spec:-unknown}"
done
