#!/bin/sh
# shellcheck disable=SC2034 # Its variables are read by the scripts that source it.

# drift's output: each result is queued while a command runs and printed when
# it exits, errors and warnings on standard error. NO_COLOR turns colours off.

drift_stdout_is_tty() {
  command -v tty >/dev/null 2>&1 && tty -s <&1 2>/dev/null
}

if [ "${NO_COLOR+x}" != x ] && { [ "${DRIFT_FORCE_COLOR:-0}" = 1 ] || drift_stdout_is_tty; }; then
  DRIFT_COLOR_YELLOW=$(printf '\033[33m')
  DRIFT_COLOR_GREEN=$(printf '\033[32m')
  DRIFT_COLOR_RED=$(printf '\033[31m')
  DRIFT_COLOR_CYAN=$(printf '\033[36m')
  DRIFT_COLOR_RESET=$(printf '\033[0m')
else
  DRIFT_COLOR_YELLOW=
  DRIFT_COLOR_GREEN=
  DRIFT_COLOR_RED=
  DRIFT_COLOR_CYAN=
  DRIFT_COLOR_RESET=
fi

drift_result_file=
drift_results_flushed=false

drift_queue_result() {
  kind=$1
  shift
  if [ -z "$drift_result_file" ]; then
    drift_result_file=$(mktemp "${TMPDIR:-/tmp}/drift-result.XXXXXX")
  fi
  printf '%s\t%s\n' "$kind" "$*" >>"$drift_result_file"
}

drift_emit_result() {
  kind=$1
  shift
  case "$kind" in
    success) printf '%s✅ %s%s\n' "$DRIFT_COLOR_GREEN" "$*" "$DRIFT_COLOR_RESET" ;;
    warning) printf '%s⚠️ %s%s\n' "$DRIFT_COLOR_YELLOW" "$*" "$DRIFT_COLOR_RESET" >&2 ;;
    error) printf '%s❌ %s%s\n' "$DRIFT_COLOR_RED" "$*" "$DRIFT_COLOR_RESET" >&2 ;;
    info) printf '%sℹ️ %s%s\n' "$DRIFT_COLOR_CYAN" "$*" "$DRIFT_COLOR_RESET" ;;
  esac
}

drift_flush_results() {
  [ "$drift_results_flushed" = false ] || return 0
  drift_results_flushed=true
  [ -n "$drift_result_file" ] || return 0

  while IFS='	' read -r kind text; do
    drift_emit_result "$kind" "$text"
  done <"$drift_result_file"
  rm -f "$drift_result_file"
}

drift_print_success() { drift_queue_result success "$*"; }
drift_print_warning() { drift_queue_result warning "$*"; }
drift_print_error() { drift_queue_result error "$*"; }
drift_print_info() { drift_queue_result info "$*"; }

if [ "${drift_exit_trap_set:-0}" != 1 ]; then
  drift_exit_trap_set=1
  trap drift_flush_results EXIT
  trap 'exit 130' INT
  trap 'exit 143' TERM
fi
