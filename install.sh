#!/usr/bin/env bash
set -euo pipefail

REPO="aquia-inc/dsva-aws-auth"
VERSION="v0.1.0"
ASSET="aws-auth-${VERSION}-aarch64-apple-darwin.tar.gz"
INSTALL_DIR="/usr/local/bin"

echo "Installing aws-auth ${VERSION}..."
gh release download "$VERSION" --repo "$REPO" --pattern "$ASSET" --output - | tar xz -C "$INSTALL_DIR"

if ! grep -q 'aws-auth()' ~/.zshrc 2>/dev/null; then
  echo 'aws-auth() { eval $(command aws-auth "$@"); }' >> ~/.zshrc
  echo "Added shell function to ~/.zshrc"
fi

echo "Done! Run 'source ~/.zshrc' or open a new terminal, then:"
echo "  aws-auth 123456"
