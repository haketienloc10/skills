#!/bin/sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
skill_dir=$(CDPATH= cd -- "$script_dir/.." && pwd)
target_dir=${1:-.}

mkdir -p "$target_dir/bin" "$target_dir/.project-notes/notes"

gitignore="$target_dir/.gitignore"
if [ ! -f "$gitignore" ]; then
  printf 'bin/\n' > "$gitignore"
elif ! grep -qxF 'bin/' "$gitignore"; then
  if [ -s "$gitignore" ]; then
    printf '\n' >> "$gitignore"
  fi
  printf 'bin/\n' >> "$gitignore"
fi

cargo build --release --manifest-path "$skill_dir/pnotes-cli/Cargo.toml"
cp "$skill_dir/pnotes-cli/target/release/pnotes" "$target_dir/bin/pnotes"
chmod +x "$target_dir/bin/pnotes"

printf 'Installed project-notes into %s\n' "$target_dir"
printf 'Run: %s/bin/pnotes guide\n' "$target_dir"
