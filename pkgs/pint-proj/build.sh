#!/usr/bin/env bash

# Command to run when pint.toml is found
command_to_run="pint build"

# List of directories to ignore (space-separated)
ignore_dirs=".git node_modules out target"

display_help() {
  echo "Usage: $0 [OPTIONS] <dir>"
  echo "Compiles a pint project or directory of projects."
  echo
  echo "Options:"
  echo "  -w, --workspace Compile a directory of projects"
  echo "  -r, --recurse   Compile a directory of projects recursively searching subdirectoris for pint.toml files"
  echo "  -h, --help      Display this help message and exit"
}

is_workspace_flag() {
  local arg="$1"
  [ "$arg" == "--workspace" ] || [ "$arg" == "-w" ]
}

is_recurse_flag() {
  local arg="$1"
  [ "$arg" == "--recurse" ] || [ "$arg" == "-r" ]
}

# Function to check if a directory should be ignored
should_ignore() {
  local dir="$1"
  for ignore in $ignore_dirs; do
    if [[ "$dir" == *"/$ignore"* ]]; then
      return 0
    fi
  done
  return 1
}

# Main function to search directories
search_dirs() {
  local dir="$1"

  # Check if the current directory should be ignored
  if should_ignore "$dir"; then
    return
  fi

  # Check if pint.toml exists in the current directory
  if [[ -f "$dir/pint.toml" ]]; then
    echo "compiling $dir"
    (
      # Build project
      cd "$dir" || return
      eval "$command_to_run"
    )
  fi

  # Recursively search subdirectories
  for subdir in "$dir"/*; do
    if [[ -d "$subdir" ]]; then
      search_dirs "$subdir"
    fi
  done
}

for arg in "$@"; do
  if [ "$arg" == "--help" ] || [ "$arg" == "-h" ]; then
    display_help
    exit 0
  fi
done

if [ -z "$1" ] || ( (is_workspace_flag "$1" || is_recurse_flag "$1") && [ -z "$2" ]) ||
  ( (! is_workspace_flag "$1" && ! is_recurse_flag "$1") && [ -n "$2" ]); then
  echo "Unknown args: " "$@"
  display_help
  exit 1
fi

if is_workspace_flag "$1"; then
  cd "$2" || exit 1
  for dir in */; do
    if [ -d "$dir" ]; then
      echo "compiling $dir"
      cd "$dir" || continue

      # Build project
      eval "$command_to_run"

      # Return to the parent directory
      cd ..
    fi
  done
elif is_recurse_flag "$1"; then
  search_dirs "$2"
else
  cd "$1" || exit 1
  # Build project
  eval "$command_to_run"
fi
