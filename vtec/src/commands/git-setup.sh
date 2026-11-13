#!/bin/sh
set -eu
. "$VTEC_CLI_DIR/core/common.sh"
[ "$#" -eq 0 ] || vtec_die "Usage: vtec git setup" 2
vtec_git_checkout || vtec_die "Not a Git checkout. Run git init first."

git -C "$VTEC_PROJECT_ROOT" config core.hooksPath "$VTEC_HOOKS_DIR"
vtec_print_success "Git hooks configured: $VTEC_HOOKS_DIR."
