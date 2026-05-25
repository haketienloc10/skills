#!/usr/bin/env bash
set -euo pipefail

repo_tarball_url="https://github.com/haketienloc10/skills/archive/refs/heads/main.tar.gz"
target_dir="."

usage() {
  cat <<'USAGE'
Usage:
  install-skills.sh [--directory PATH]

Options:
  --directory PATH  Install skills into PATH instead of the current directory.
  -h, --help        Show this help.
USAGE
}

while (($# > 0)); do
  case "$1" in
    --directory)
      if (($# < 2)); then
        printf 'Missing value for --directory\n' >&2
        exit 1
      fi
      target_dir=$2
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      printf 'Unknown argument: %s\n' "$1" >&2
      usage >&2
      exit 1
      ;;
  esac
done

tmp_dir=$(mktemp -d)
cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

curl -fsSL "${repo_tarball_url}?$(date +%s)" | tar -xz -C "$tmp_dir" --strip-components=1
bash "$tmp_dir/install.sh" "$target_dir"
