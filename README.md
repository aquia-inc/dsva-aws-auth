# aws-auth

Simple CLI tool for AWS MFA session authentication. Automates the multi-step process of getting temporary session credentials via `aws sts get-session-token`.

## Prerequisites

- [AWS CLI](https://aws.amazon.com/cli/) installed and configured with your permanent access key
- An MFA device associated with your IAM user
- `jq` is **not** required — this tool handles JSON parsing natively

## Quick Install (macOS Apple Silicon)

```bash
curl -sL https://github.com/aquia-inc/dsva-aws-auth/releases/latest/download/aws-auth-v0.1.0-aarch64-apple-darwin.tar.gz | tar xz -C /usr/local/bin && echo 'aws-auth() { eval $(command aws-auth "$@"); }' >> ~/.zshrc && source ~/.zshrc
```

## Install from Source

Requires [Rust](https://rustup.rs/).

```bash
cargo install --git https://github.com/aquia-inc/dsva-aws-auth
echo 'aws-auth() { eval $(command aws-auth "$@"); }' >> ~/.zshrc
source ~/.zshrc
```

## Usage

Authenticate with your 6-digit MFA code:

```bash
aws-auth 123456
```

This will:
1. Auto-detect your MFA device ARN via `aws iam list-mfa-devices`
2. Request temporary session credentials from AWS STS
3. Export `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, and `AWS_SESSION_TOKEN` into your current shell

Status messages go to stderr, export commands go to stdout — so `eval` captures only the exports.
