#!/usr/bin/env sh
set -e

REPO="realloon/mkrimod"
BIN="mkrimod"
if [ -n "$INSTALL_DIR" ]; then
  DEST="$INSTALL_DIR"
elif [ -d "$HOME/.cargo/bin" ]; then
  DEST="$HOME/.cargo/bin"
else
  DEST="$HOME/.local/bin"
fi

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

mkdir -p "$DEST"
cp -f "$BIN_PATH" "$DEST/$BIN"
chmod +x "$DEST/$BIN"
rm -rf "$TMP_DIR"

echo "${BIN} installed successfully to ${DEST}"

case ":$PATH:" in
  *":$DEST:"*) ;;
  *) echo "Note: Add $DEST to your PATH to run ${BIN} directly." ;;
esac
