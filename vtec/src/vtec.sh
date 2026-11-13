#!/bin/sh
set -eu

VTEC_CLI_DIR=$(CDPATH='' cd -P "$(dirname "$0")" && pwd)
VTEC_PROJECT_ROOT=$(git -C "$VTEC_CLI_DIR" rev-parse --show-toplevel)
export VTEC_CLI_DIR VTEC_PROJECT_ROOT

. "$VTEC_CLI_DIR/core/common.sh"

vtec_usage_error() {
  vtec_print_error "vtec: $1"
  vtec_print_error "Run 'vtec --help' for usage."
  exit 2
}

command=${1:---help}
[ "$#" -eq 0 ] || shift

commands="$VTEC_CLI_DIR/commands"

case "$command" in
  --help | -h) exec "$commands/help.sh" "$@" ;;
  git)
    subcommand=${1:-}
    [ "$#" -eq 0 ] || shift
    case "$subcommand" in
      setup) exec "$commands/git-setup.sh" "$@" ;;
      commit-msg) exec "$commands/git-commit-msg.sh" "$@" ;;
      lint-history) exec "$commands/git-lint-history.sh" "$@" ;;
      *) vtec_usage_error "Usage: vtec git <setup|commit-msg|lint-history>" ;;
    esac
    ;;
  quality)
    subcommand=${1:-}
    [ "$#" -eq 0 ] || shift
    case "$subcommand" in
      check) exec "$commands/quality-check.sh" "$@" ;;
      coverage) exec "$commands/quality-coverage.sh" "$@" ;;
      *) vtec_usage_error "Usage: vtec quality <check|coverage>" ;;
    esac
    ;;
  widget)
    subcommand=${1:-}
    [ "$#" -eq 0 ] || shift
    widgets="$VTEC_PROJECT_ROOT/.github/widgets"
    case "$subcommand" in
      check-run) exec "$widgets/check-run.sh" "$@" ;;
      sticky-comment) exec "$widgets/sticky-comment.sh" "$@" ;;
      workflow) exec "$widgets/workflow.sh" "$@" ;;
      contracts) exec "$widgets/contracts.sh" "$@" ;;
      code-quality) exec "$widgets/code-quality.sh" "$@" ;;
      dashboard) exec "$widgets/dashboard.sh" "$@" ;;
      *) vtec_usage_error "Usage: vtec widget <check-run|sticky-comment|workflow|contracts|code-quality|dashboard>" ;;
    esac
    ;;
  --*) vtec_usage_error "Unknown option: $command" ;;
  *) vtec_usage_error "Unknown command: $command" ;;
esac
