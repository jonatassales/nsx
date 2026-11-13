#!/bin/sh
set -eu

DRIFT_CLI_DIR=$(CDPATH='' cd -P "$(dirname "$0")" && pwd)
DRIFT_PROJECT_ROOT=$(git -C "$DRIFT_CLI_DIR" rev-parse --show-toplevel)
export DRIFT_CLI_DIR DRIFT_PROJECT_ROOT

. "$DRIFT_CLI_DIR/core/common.sh"

drift_usage_error() {
  drift_print_error "drift: $1"
  drift_print_error "Run 'drift --help' for usage."
  exit 2
}

command=${1:---help}
[ "$#" -eq 0 ] || shift

commands="$DRIFT_CLI_DIR/commands"

case "$command" in
  --help | -h) exec "$commands/help.sh" "$@" ;;
  git)
    subcommand=${1:-}
    [ "$#" -eq 0 ] || shift
    case "$subcommand" in
      setup) exec "$commands/git-setup.sh" "$@" ;;
      commit-msg) exec "$commands/git-commit-msg.sh" "$@" ;;
      lint-history) exec "$commands/git-lint-history.sh" "$@" ;;
      *) drift_usage_error "Usage: drift git <setup|commit-msg|lint-history>" ;;
    esac
    ;;
  quality)
    subcommand=${1:-}
    [ "$#" -eq 0 ] || shift
    case "$subcommand" in
      check) exec "$commands/quality-check.sh" "$@" ;;
      coverage) exec "$commands/quality-coverage.sh" "$@" ;;
      *) drift_usage_error "Usage: drift quality <check|coverage>" ;;
    esac
    ;;
  widget)
    subcommand=${1:-}
    [ "$#" -eq 0 ] || shift
    widgets="$DRIFT_PROJECT_ROOT/.github/widgets"
    case "$subcommand" in
      check-run) exec "$widgets/check-run.sh" "$@" ;;
      sticky-comment) exec "$widgets/sticky-comment.sh" "$@" ;;
      workflow) exec "$widgets/workflow.sh" "$@" ;;
      contracts) exec "$widgets/contracts.sh" "$@" ;;
      code-quality) exec "$widgets/code-quality.sh" "$@" ;;
      dashboard) exec "$widgets/dashboard.sh" "$@" ;;
      *) drift_usage_error "Usage: drift widget <check-run|sticky-comment|workflow|contracts|code-quality|dashboard>" ;;
    esac
    ;;
  --*) drift_usage_error "Unknown option: $command" ;;
  *) drift_usage_error "Unknown command: $command" ;;
esac
