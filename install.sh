#!/usr/bin/env bash
set -euo pipefail

REPO="aquia-inc/dsva-aws-auth"
VERSION="v0.1.0"
ASSET="aws-auth-${VERSION}-aarch64-apple-darwin.tar.gz"
INSTALL_DIR="$HOME/.local/bin"

mkdir -p "$INSTALL_DIR"

echo "Installing aws-auth ${VERSION}..."
curl -sL "https://github.com/${REPO}/releases/download/${VERSION}/${ASSET}" | tar xz -C "$INSTALL_DIR"

if ! grep -q 'aws-auth()' ~/.zshrc 2>/dev/null; then
  echo 'aws-auth() { eval $(command aws-auth "$@"); }' >> ~/.zshrc
  echo "Added shell function to ~/.zshrc"
fi

if ! echo "$PATH" | grep -q "$HOME/.local/bin"; then
  echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
  echo "Added ~/.local/bin to PATH in ~/.zshrc"
fi

echo "Done! Run 'source ~/.zshrc' or open a new terminal, then:"
echo "  aws-auth 123456"
