{ pkgs, pint }:
pkgs.writeShellScriptBin "compile-pint" ''
  display_help() {
    echo "Usage: $0 [OPTIONS] <dir>"
    echo "Compiles a pint project or directory of projects."
    echo
    echo "Options:"
    echo "  -w, --workspace Compile a directory of projects"
    echo "  -h, --help      Display this help message and exit"
  }

  is_workspace_flag() {
    local arg="$1"
    [ "$arg" == "--workspace" ] || [ "$arg" == "-w" ]
  }

  for arg in "$@"
  do
      if [ "$arg" == "--help" ] || [ "$arg" == "-h" ]
      then
          display_help
          exit 0
      fi
  done

  if [ -z "$1" ] || ( is_workspace_flag "$1" && [ -z "$2" ] ) || ( ! is_workspace_flag "$1" && [ -n "$2" ] ); then
    display_help
    exit 1
  fi

  if is_workspace_flag "$1"; then
    cd "$2" || exit 1
    for dir in */; do
      if [ -d "$dir" ]; then
        echo "compiling $dir"
        cd "$dir" || continue
        
        ${pint}/bin/pint build
        
        # Return to the parent directory
        cd ..
      fi
    done
  else
    cd "$1" || exit 1
    ${pint}/bin/pint build
  fi
''
