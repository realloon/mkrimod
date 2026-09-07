#!/bin/sh
set -e

OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Linux)  TARGET="${ARCH}-unknown-linux-gnu" ;;
  Darwin)
    [ "$ARCH" = "arm64" ] || { echo "Error: Intel Mac ($ARCH) is not supported."; exit 1; }
    TARGET="aarch64-apple-darwin"
    ;;
  *) echo "Unsupported OS: $OS"; exit 1 ;;
esac

DEST="$HOME/.local/bin"
mkdir -p "$DEST"

TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

curl -fsSL "https://github.com/realloon/mkrimod/releases/latest/download/mkrimod-${TARGET}.tar.gz" | tar -xz -C "$TMP"
BIN_PATH="$(find "$TMP" -type f -name mkrimod | head -n 1)"
cp -f "$BIN_PATH" "$DEST/mkrimod"
chmod +x "$DEST/mkrimod"

echo "Installed mkrimod to $DEST/mkrimod"
case ":$PATH:" in
  *":$DEST:"*) ;;
  *) echo "Add to PATH: export PATH=\"\$HOME/.local/bin:\$PATH\"" ;;
esac
