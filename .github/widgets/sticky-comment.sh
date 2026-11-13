#!/bin/sh
set -eu
. "$VTEC_CLI_DIR/core/common.sh"
[ "$#" -eq 1 ] || vtec_die "Usage: vtec widget sticky-comment <key> <body.md" 2
vtec_need gh
vtec_need jq

# One comment per key on a pull request, edited in place on every run instead
# of piling up. A hidden marker names the key, and only a comment a bot wrote
# with that marker is edited, so nobody's own comment is overwritten. The job
# that publishes it needs the pull-requests: write permission.

key=$1

case "$key" in
  '' | -* | *- | *--* | *[!a-z0-9-]*) vtec_die "The key is kebab-case." 2 ;;
esac

case "${GITHUB_REPOSITORY:-}" in
  */*/* | /* | */ | *[!A-Za-z0-9._/-]*) vtec_die "GITHUB_REPOSITORY must name the repository as owner/name." 2 ;;
  */*) ;;
  *) vtec_die "GITHUB_REPOSITORY must name the repository as owner/name." 2 ;;
esac

[ -n "${WIDGET_PULL_REQUEST:-}" ] || vtec_die "WIDGET_PULL_REQUEST must be the number of the pull request to comment on." 2

case "$WIDGET_PULL_REQUEST" in
  *[!0-9]*) vtec_die "WIDGET_PULL_REQUEST is a number." 2 ;;
esac

body_file=$(mktemp)
# vtec's own EXIT trap prints the queued results, so this one chains to it.
trap 'rm -f "$body_file"; vtec_flush_results' EXIT

[ ! -t 0 ] || vtec_die "Pipe the Markdown comment into stdin." 2
cat >"$body_file"
[ -s "$body_file" ] || vtec_die "The comment body on stdin is empty." 2

marker="<!-- vtec-widget:$key -->"
issue_path="repos/$GITHUB_REPOSITORY/issues/$WIDGET_PULL_REQUEST"

# The marker sits inside the jq program as text, which is safe only because
# the key was checked as kebab-case above.
matching_comment_ids=$(gh api --paginate "$issue_path/comments" \
  --jq ".[] | select(.user.type == \"Bot\" and (.body | startswith(\"$marker\"))) | .id") ||
  vtec_die "Could not read the comments of pull request $WIDGET_PULL_REQUEST."

existing_comment_id=$(printf '%s\n' "$matching_comment_ids" | head -n 1)

# GitHub rejects a comment over 65536 characters, so a longer one is cut at
# 65400, which leaves room for the note that says so.
payload=$(jq -Rs --arg marker "$marker" '
  ($marker + "\n" + .) as $body
  | {body: (if ($body | length) > 65536 then $body[0:65400] + "\n\n_Truncated: the comment passed the GitHub limit._" else $body end)}
' <"$body_file")

if [ -n "$existing_comment_id" ]; then
  printf '%s' "$payload" | gh api --method PATCH "repos/$GITHUB_REPOSITORY/issues/comments/$existing_comment_id" --input - --jq .html_url ||
    vtec_die "Could not update the $key comment on pull request $WIDGET_PULL_REQUEST."

  exit 0
fi

printf '%s' "$payload" | gh api --method POST "$issue_path/comments" --input - --jq .html_url ||
  vtec_die "Could not post the $key comment on pull request $WIDGET_PULL_REQUEST."
