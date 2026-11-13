#!/bin/sh
set -eu

: "${DRIFT_PROJECT_ROOT:?Run it through drift/drift.sh widget.}"
: "${DRIFT_CLI_DIR:?Run it through drift/drift.sh widget.}"
. "$DRIFT_CLI_DIR/core/common.sh"

# The CI Gates of a pull request, each widget in its own section; with
# --publish, also a sticky comment per gate and a check run per gate.

widgets_dir="$DRIFT_PROJECT_ROOT/.github/widgets"

publish=false
if [ "${1:-}" = "--publish" ]; then
  publish=true
  shift
fi

dashboard_tmp=$(mktemp)
# drift's own EXIT trap prints the queued results, so this one chains to it.
trap 'rm -f "$dashboard_tmp"; drift_flush_results' EXIT

{
  printf '## nsx CI Gates\n\n'

  printf '<details open>\n<summary><b>1. Workflow Gate</b></summary>\n\n'
  sh "$widgets_dir/workflow.sh" "$@"
  printf '\n</details>\n\n'

  printf '<details open>\n<summary><b>2. Contracts Gate</b></summary>\n\n'
  sh "$widgets_dir/contracts.sh" "$@"
  printf '\n</details>\n\n'

  printf '<details open>\n<summary><b>3. Code Quality Gate</b></summary>\n\n'
  sh "$widgets_dir/code-quality.sh" "$@"
  printf '\n</details>\n'
} >"$dashboard_tmp"

cat "$dashboard_tmp"

[ "$publish" = true ] || exit 0

if [ -n "${WIDGET_PULL_REQUEST:-}" ] && [ -n "${GITHUB_REPOSITORY:-}" ]; then
  "$widgets_dir/sticky-comment.sh" "nsx-ci-gates" <"$dashboard_tmp" || drift_warn "Could not update the CI Gates comment"
fi

if [ -n "${WIDGET_SHA:-}" ] && [ -n "${GITHUB_REPOSITORY:-}" ]; then
  sh "$widgets_dir/workflow.sh" "$@" | "$widgets_dir/check-run.sh" "Workflow" "neutral" "Tickets named" || drift_warn "Could not publish the Workflow check run"
  sh "$widgets_dir/contracts.sh" "$@" | "$widgets_dir/check-run.sh" "Contracts" "neutral" "Contract changes" || drift_warn "Could not publish the Contracts check run"
  sh "$widgets_dir/code-quality.sh" "$@" | "$widgets_dir/check-run.sh" "Code Quality" "neutral" "Rules applied" || drift_warn "Could not publish the Code Quality check run"
fi
