#!/bin/sh
# shellcheck disable=SC2034 # Its variables are read by the scripts that source it.

: "${DRIFT_PROJECT_ROOT:?DRIFT_PROJECT_ROOT must be set by drift/src/drift.sh}"
: "${DRIFT_CLI_DIR:?DRIFT_CLI_DIR must be set by drift/src/drift.sh}"

. "$DRIFT_CLI_DIR/core/output.sh"

# Git hooks live under drift/hooks.
DRIFT_HOOKS_DIR=drift/hooks

drift_die() {
  drift_print_error "drift: $1"
  exit "${2:-1}"
}

drift_warn() { drift_print_warning "drift: $*"; }
drift_has() { command -v "$1" >/dev/null 2>&1; }
drift_need() { drift_has "$1" || drift_die "Required command not found: $1" 127; }

drift_git_checkout() {
  git -C "$DRIFT_PROJECT_ROOT" rev-parse --is-inside-work-tree >/dev/null 2>&1
}
