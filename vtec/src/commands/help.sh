#!/bin/sh
set -eu
. "$VTEC_CLI_DIR/core/common.sh"
[ "$#" -eq 0 ] || vtec_die "Help does not accept arguments." 2

command_line() {
  printf '  %-44s %s\n' "$1" "$2"
}

printf '%s\n' 'vtec: the tooling of the nsx repository'
printf '\n%s\n' 'Commands:'
command_line 'git [setup|commit-msg|lint-history]' 'Install and run the commit guardrails'
command_line 'quality [check|coverage]' 'Non-compliant paths and rule coverage'
command_line 'widget [<name>|check-run|sticky-comment]' 'Print PR widgets and update check runs or comments'
