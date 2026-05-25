#!/usr/bin/env bash
set -euo pipefail

repo_dir=$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
project_dir=${1:-.}
project_dir=$(CDPATH= cd -- "$project_dir" && pwd)
target_dir="$project_dir/.agents/skills"

skills=()
while IFS= read -r skill_file; do
  skills+=("$(basename "$(dirname "$skill_file")")")
done < <(find "$repo_dir" -mindepth 2 -maxdepth 2 -name SKILL.md -type f | sort)

if ((${#skills[@]} == 0)); then
  printf 'No skills found.\n' >&2
  exit 1
fi

selected=()
for _ in "${skills[@]}"; do
  selected+=(0)
done

cursor=0

cleanup() {
  printf '\033[?25h'
}
trap cleanup EXIT

render() {
  printf '\033[H\033[2J\033[?25l'
  printf 'Select skills to install\n'
  printf 'Space: select/deselect  Enter: install  Up/Down: move\n\n'

  for i in "${!skills[@]}"; do
    marker=' '
    pointer=' '
    if [[ ${selected[$i]} -eq 1 ]]; then
      marker='x'
    fi
    if [[ $i -eq $cursor ]]; then
      pointer='>'
    fi
    printf '%s [%s] %s\n' "$pointer" "$marker" "${skills[$i]}"
  done
}

read_key() {
  local key rest
  IFS= read -rsn1 key
  if [[ $key == $'\x1b' ]]; then
    IFS= read -rsn2 -t 0.1 rest || true
    key+=$rest
  fi
  printf '%s' "$key"
}

while true; do
  render
  key=$(read_key)
  case "$key" in
    ' ')
      if [[ ${selected[$cursor]} -eq 1 ]]; then
        selected[$cursor]=0
      else
        selected[$cursor]=1
      fi
      ;;
    $'\x1b[A')
      if ((cursor > 0)); then
        cursor=$((cursor - 1))
      fi
      ;;
    $'\x1b[B')
      if ((cursor < ${#skills[@]} - 1)); then
        cursor=$((cursor + 1))
      fi
      ;;
    '')
      break
      ;;
  esac
done

picked=()
for i in "${!skills[@]}"; do
  if [[ ${selected[$i]} -eq 1 ]]; then
    picked+=("${skills[$i]}")
  fi
done

printf '\033[H\033[2J\033[?25h'

if ((${#picked[@]} == 0)); then
  printf 'No skills selected. Nothing installed.\n'
  exit 0
fi

mkdir -p "$target_dir"

for skill in "${picked[@]}"; do
  src="$repo_dir/$skill"
  dest="$target_dir/$skill"
  rm -rf "$dest"
  mkdir -p "$dest"
  tar --exclude='.git' --exclude='target' -C "$src" -cf - . | tar -C "$dest" -xf -
  printf 'Installed %s -> %s\n' "$skill" "$dest"

  if [[ $skill == project-notes ]]; then
    sh "$dest/scripts/install.sh" "$project_dir"
  fi
done

printf '\nDone. Installed %d skill(s) into %s\n' "${#picked[@]}" "$target_dir"
