#!/bin/sh
# Runs the nsx binary for this machine: the one the package ships in dist/,
# or, in the repository, the one cargo built in target/release/.
set -eu

unsupported() {
  printf '%s\n' 'nsx runs on macOS and Linux, on x64 and arm64.' >&2
  exit 1
}

# npm's node_modules/.bin may reach this file through a symbolic link.
source=$0
while [ -L "$source" ]; do
  folder=$(CDPATH='' cd -P -- "$(dirname -- "$source")" && pwd)
  link=$(readlink "$source")
  case "$link" in
    /*) source=$link ;;
    *) source=$folder/$link ;;
  esac
done

package=$(dirname -- "$(CDPATH='' cd -P -- "$(dirname -- "$source")" && pwd)")

case "$(uname -s 2>/dev/null || true)" in
  Darwin) os=darwin ;;
  Linux) os=linux ;;
  *) unsupported ;;
esac

case "$(uname -m 2>/dev/null || true)" in
  arm64 | aarch64) arch=arm64 ;;
  x86_64 | amd64) arch=amd64 ;;
  *) unsupported ;;
esac

binary=$package/dist/nsx-$os-$arch
[ -f "$binary" ] || binary=$package/target/release/nsx

if [ ! -x "$binary" ]; then
  printf '%s\n' "nsx has no binary for $os/$arch: reinstall @langdrift/nsx, or run cargo build --release in the repository." >&2
  exit 1
fi

exec "$binary" "$@"
