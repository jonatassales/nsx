#!/bin/sh
# shellcheck disable=SC2034 # Its variables are read by the scripts that source it.

# Commit guardrails shared by `drift git commit-msg` (the commit being
# written) and `drift git lint-history` (every commit in a range).
# Each check reads a commit message, or a list of changed paths, on stdin.

DRIFT_AI_SIGNATURE_ERROR='Remove the AI-generated signature, trailer or footer.'
DRIFT_TICKET_ERROR="Changes under src/, tests/, schema/, langdrift/ or bin/ need a 'Ticket: <NNNN>' trailer naming a ticket in .agents/tickets/ or in its Git history."
DRIFT_CONVENTIONAL_ERROR='Write the subject as Conventional Commits: type(scope): subject, lowercase, at most 72 characters, no trailing period, then a blank line.'

# True when the message carries an AI-generated signature, trailer or footer.
drift_has_ai_signature() {
  ai_tools='claude|anthropic|copilot|openai|chatgpt|codex|cursor|gemini'
  grep -Eiq "^co-authored-by:.*($ai_tools)|^claude-session:|generated with.*($ai_tools)"
}

# True when a changed path lies under the crate's code.
drift_touches_code() {
  grep -E '^(src|tests|schema|langdrift|bin)/' | grep -Evq '/\.gitkeep$'
}

# True when the message's Ticket trailer names a ticket of the tickets folder:
# one in the tree, or one that left the tree and that the history of HEAD
# added.
drift_names_ticket() {
  ticket=$(git interpret-trailers --parse | awk -F': *' 'tolower($1) == "ticket" { print $2; exit }')
  case "$ticket" in
    '' | *[!0-9]*) return 1 ;;
  esac
  for candidate in "$DRIFT_PROJECT_ROOT"/.agents/tickets/"$ticket"-*.ticket.md; do
    [ -f "$candidate" ] && return 0
  done

  added=$(git -C "$DRIFT_PROJECT_ROOT" log -1 --diff-filter=A --format=%H HEAD -- ".agents/tickets/$ticket-*.ticket.md")

  [ -n "$added" ]
}

# True when the message follows Conventional Commits: a typed subject in
# lowercase within 72 characters, without a trailing period, and a blank
# second line. Git's own merge and revert subjects pass.
drift_is_conventional() {
  awk '
    NR == 1 {
      subject = $0
      if (subject ~ /^(Merge|Revert) /) { merge = 1; next }
      types = "build|chore|ci|docs|feat|fix|perf|refactor|revert|style|test"
      if (subject !~ ("^(" types ")(\\([a-z0-9-]+\\))?!?: [a-z0-9`]")) bad = 1
      if (length(subject) > 72 || subject ~ /\.$/) bad = 1
      next
    }
    NR == 2 && !merge && $0 != "" { bad = 1 }
    END { exit bad }
  '
}
