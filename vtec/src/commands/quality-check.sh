#!/bin/sh
set -eu
. "$VTEC_CLI_DIR/core/common.sh"
[ "$#" -le 1 ] || vtec_die "Usage: vtec quality check [rule]" 2
cd "$VTEC_PROJECT_ROOT"

# Each path that does not apply a rule file in .claude/rules, in red, with the
# rule and the violation: the same detectors as vtec quality coverage. A rule
# (14-rust.rule.md, rust.rule.md or rust) narrows it to that file. The
# detectors are patterns, so the list is one to review; it never fails.

workdir=$(mktemp -d)
# vtec's own EXIT trap prints the queued results, so this one chains to it.
trap 'rm -rf "$workdir"; vtec_flush_results' EXIT

sh "$VTEC_CLI_DIR/quality/run-detectors.sh" "$workdir" "$@" || exit $?

total=0

while IFS= read -r rule; do
  [ -s "$workdir/findings/$rule" ] || continue

  sort -u "$workdir/findings/$rule" >"$workdir/listing"

  while IFS='	' read -r path violation; do
    vtec_print_error "$rule · $path: $violation"
    total=$((total + 1))
  done <"$workdir/listing"
done <"$workdir/rules"

if [ "$total" -eq 0 ]; then
  vtec_print_success "Every path applies the rules in .claude/rules."

  exit 0
fi

noun=findings
[ "$total" -ne 1 ] || noun=finding

vtec_print_warning "$total $noun against the rules in .claude/rules."
