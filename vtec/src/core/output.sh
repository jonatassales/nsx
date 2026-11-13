#!/bin/sh
# shellcheck disable=SC2034 # Its variables are read by the scripts that source it.

# vtec's output: each result is queued while a command runs and printed when
# it exits, errors and warnings on standard error. NO_COLOR turns colours off.

vtec_stdout_is_tty() {
  command -v tty >/dev/null 2>&1 && tty -s <&1 2>/dev/null
}

if [ "${NO_COLOR+x}" != x ] && { [ "${VTEC_FORCE_COLOR:-0}" = 1 ] || vtec_stdout_is_tty; }; then
  VTEC_COLOR_YELLOW=$(printf '\033[33m')
  VTEC_COLOR_GREEN=$(printf '\033[32m')
  VTEC_COLOR_RED=$(printf '\033[31m')
  VTEC_COLOR_CYAN=$(printf '\033[36m')
  VTEC_COLOR_RESET=$(printf '\033[0m')
else
  VTEC_COLOR_YELLOW=
  VTEC_COLOR_GREEN=
  VTEC_COLOR_RED=
  VTEC_COLOR_CYAN=
  VTEC_COLOR_RESET=
fi

vtec_result_file=
vtec_results_flushed=false

vtec_queue_result() {
  kind=$1
  shift
  if [ -z "$vtec_result_file" ]; then
    vtec_result_file=$(mktemp "${TMPDIR:-/tmp}/vtec-result.XXXXXX")
  fi
  printf '%s\t%s\n' "$kind" "$*" >>"$vtec_result_file"
}

vtec_emit_result() {
  kind=$1
  shift
  case "$kind" in
    success) printf '%s✅ %s%s\n' "$VTEC_COLOR_GREEN" "$*" "$VTEC_COLOR_RESET" ;;
    warning) printf '%s⚠️ %s%s\n' "$VTEC_COLOR_YELLOW" "$*" "$VTEC_COLOR_RESET" >&2 ;;
    error) printf '%s❌ %s%s\n' "$VTEC_COLOR_RED" "$*" "$VTEC_COLOR_RESET" >&2 ;;
    info) printf '%sℹ️ %s%s\n' "$VTEC_COLOR_CYAN" "$*" "$VTEC_COLOR_RESET" ;;
  esac
}

vtec_flush_results() {
  [ "$vtec_results_flushed" = false ] || return 0
  vtec_results_flushed=true
  [ -n "$vtec_result_file" ] || return 0

  while IFS='	' read -r kind text; do
    vtec_emit_result "$kind" "$text"
  done <"$vtec_result_file"
  rm -f "$vtec_result_file"
}

vtec_print_success() { vtec_queue_result success "$*"; }
vtec_print_warning() { vtec_queue_result warning "$*"; }
vtec_print_error() { vtec_queue_result error "$*"; }
vtec_print_info() { vtec_queue_result info "$*"; }

if [ "${vtec_exit_trap_set:-0}" != 1 ]; then
  vtec_exit_trap_set=1
  trap vtec_flush_results EXIT
  trap 'exit 130' INT
  trap 'exit 143' TERM
fi
