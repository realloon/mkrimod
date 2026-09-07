#!/usr/bin/env sh
set -e

REPO="realloon/create-rimworld-mod"
BIN="mkrimod"
DEST="${INSTALL_DIR:-/usr/local/bin}"

OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Linux)  TARGET="${ARCH}-unknown-linux-gnu" ;;
  Darwin)
    case "$ARCH" in
      arm64) TARGET="aarch64-apple-darwin" ;;
      *)     TARGET="x86_64-apple-darwin" ;;
    esac
    ;;
  *) echo "Unsupported OS: $OS"; exit 1 ;;
esac

URL="https://github.com/${REPO}/releases/latest/download/${BIN}-${TARGET}.tar.gz"

echo "Downloading ${BIN} for ${TARGET}..."
TMP_DIR="$(mktemp -d)"
curl -sSL "$URL" | tar -xz -C "$TMP_DIR"
BIN_PATH="$(find "$TMP_DIR" -type f -name "$BIN" | head -n 1)"

if [ -z "$BIN_PATH" ]; then
  echo "Error: Failed to find binary in downloaded archive."
  rm -rf "$TMP_DIR"
  exit 1
fi

install -m 755 "$BIN_PATH" "$DEST" 2>/dev/null || sudo install -m 755 "$BIN_PATH" "$DEST"
rm -rf "$TMP_DIR"

echo "${BIN} installed successfully to ${DEST}"
