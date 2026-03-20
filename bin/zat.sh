#!/usr/bin/env bash
SELF="$0"
while [ -L "$SELF" ]; do
  DIR="$(cd "$(dirname "$SELF")" && pwd)"
  SELF="$(readlink "$SELF")"
  [[ "$SELF" != /* ]] && SELF="$DIR/$SELF"
done
LIBEXEC_DIR="$(cd "$(dirname "$SELF")/../libexec/zat" && pwd)"

file="$1"

# Directory support
if [ -d "$file" ]; then
  printed=0
  entry_files=("index.md" "README.md" "index.ts" "index.js" "index.tsx" "index.jsx" "mod.rs" "lib.rs" "main.rs" "__init__.py" ".")
  for entry in "${entry_files[@]}"; do
    if [ "$entry" = "." ]; then
      [ "$printed" -eq 1 ] && echo ""
      ls -1 "$file"
      printed=1
    else
      target="$file/$entry"
      if [ -f "$target" ]; then
        [ "$printed" -eq 1 ] && echo ""
        echo "$entry:"
        "$0" "$target" | sed 's/^/  /'
        printed=1
      fi
    fi
  done
  exit 0
fi

case "$file" in
  *.js|*.jsx|*.cjs|*.mjs)
    exec "$LIBEXEC_DIR/zat-js-viewer" --lang js < "$file"
    ;;
  *.ts|*.tsx)
    exec "$LIBEXEC_DIR/zat-js-viewer" --lang ts < "$file"
    ;;
  *.rs)
    exec "$LIBEXEC_DIR/zat-rust-viewer" < "$file"
    ;;
  *.py)
    exec "$LIBEXEC_DIR/zat-python-viewer" < "$file"
    ;;
  *)
    exec cat -n < "$file"
    ;;
esac
