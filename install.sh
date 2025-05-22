#!/bin/sh

# Return code:
#   0: Installed successfully
#   1: Cancelled
#   2: Error

ilog() { echo "[ INFO ] $*" >&2; }
wlog() { echo "[ WARN ] $*" >&2; }
elog() { echo "[ERROR!] $*" >&2; }

print_header() {
  cat <<EOF
+-------------------+
|  mkpdf installer  |
+-------------------+

Press Enter  to continue
Press Ctrl-C to cancel
EOF
}

check_dependencies() {
  for deps in cargo git; do
    if ! which "$deps" >/dev/null 2>&1; then
      elog "Can't execute '$deps'. Make sure it's in PATH."
      return 2
    fi
  done
}

print_index() {
  printf "\n# %s\n" "$*"
}

install_mkpdf() {
  print_index "Clone from GitHub"
  git clone https://github.com/SATA0384/mkpdf
  if ! cd ./mkpdf; then
    elog "Failed to \`cd $(pwd)/mkpdf\`."
    return 2
  fi

  print_index "Installing"
  if ! cargo install --path .; then
    elog "Failed to \`cargo install\`."
    return 2
  fi
}

print_success_message() {
  echo "Installation has been finished successfully."
  echo "If you want to uninstall this, use \`cargo uninstall mkpdf\`."
}

print_error_hint() {
  ilog "The temporary directory is '$(pwd)'."
  ilog "Try to continue installation manually."
}

main() {
  # Dependencies check
  if ! check_dependencies; then return 2; fi

  # Confirm
  print_header
  read -r _

  # Prepair
  tmpdir="$(mktemp -d)"
  if [ "$(dirname "$tmpdir")" != '/tmp' ]; then
    elog 'Failed to create temporary directory.'
    return 2
  fi

  if ! cd "$tmpdir"; then
    elog "Failed to change directory."
    return 2
  fi

  # Install
  if install_mkpdf; then
    print_success_message
  else
    # If failed
    print_error_hint
    return 2
  fi

  return 0
}

main
