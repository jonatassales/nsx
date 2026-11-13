#!/bin/sh
set -eu
. "$VTEC_CLI_DIR/core/common.sh"
[ "$#" -le 1 ] || vtec_die "Usage: vtec quality coverage [rule]" 2
cd "$VTEC_PROJECT_ROOT"

# How much of the tree applies each rule file in .claude/rules, as a Markdown
# table: of the files a rule's detectors look at, the share with no finding.
# A rule (14-rust.rule.md, rust.rule.md or rust) narrows it to that row.
# The detectors are patterns, so the percentage is an estimate.

workdir=$(mktemp -d)
# vtec's own EXIT trap prints the queued results, so this one chains to it.
trap 'rm -rf "$workdir"; vtec_flush_results' EXIT

sh "$VTEC_CLI_DIR/quality/run-detectors.sh" "$workdir" "$@" || exit $?

printf '## Rules applied\n\n'
printf '| Rule | Applied |\n'
printf '| --- | ---: |\n'

while IFS= read -r rule; do
  if [ ! -f "$workdir/scope/$rule" ]; then
    printf '| %s | not measured |\n' "$rule"
    continue
  fi

  checked=$(sort -u "$workdir/scope/$rule" | wc -l | tr -d ' ')
  flagged=$(cut -f1 "$workdir/findings/$rule" | sort -u | wc -l | tr -d ' ')
  applied=$(awk -v checked="$checked" -v flagged="$flagged" 'BEGIN { if (checked == 0) print "n/a"; else printf "%.1f%%", (checked - flagged) * 100 / checked }')
  printf '| %s | %s |\n' "$rule" "$applied"
done <"$workdir/rules"
