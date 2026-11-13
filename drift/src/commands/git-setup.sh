#!/bin/sh
set -eu
. "$DRIFT_CLI_DIR/core/common.sh"
[ "$#" -eq 0 ] || drift_die "Usage: drift git setup" 2
drift_git_checkout || drift_die "Not a Git checkout. Run git init first."

git -C "$DRIFT_PROJECT_ROOT" config core.hooksPath "$DRIFT_HOOKS_DIR"
drift_print_success "Git hooks configured: $DRIFT_HOOKS_DIR."
