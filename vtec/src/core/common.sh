#!/bin/sh
# shellcheck disable=SC2034 # Its variables are read by the scripts that source it.

: "${VTEC_PROJECT_ROOT:?VTEC_PROJECT_ROOT must be set by vtec/src/vtec.sh}"
: "${VTEC_CLI_DIR:?VTEC_CLI_DIR must be set by vtec/src/vtec.sh}"

. "$VTEC_CLI_DIR/core/output.sh"

# Git hooks live under vtec/hooks.
VTEC_HOOKS_DIR=vtec/hooks

vtec_die() {
  vtec_print_error "vtec: $1"
  exit "${2:-1}"
}

vtec_warn() { vtec_print_warning "vtec: $*"; }
vtec_has() { command -v "$1" >/dev/null 2>&1; }
vtec_need() { vtec_has "$1" || vtec_die "Required command not found: $1" 127; }

vtec_git_checkout() {
  git -C "$VTEC_PROJECT_ROOT" rev-parse --is-inside-work-tree >/dev/null 2>&1
}
